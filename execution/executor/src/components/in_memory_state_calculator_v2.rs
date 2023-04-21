// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{ParsedTransactionOutput, ProofReader};
use anyhow::{anyhow, bail, Result};
use aptos_crypto::{hash::CryptoHash, HashValue};
use aptos_scratchpad::{FrozenSparseMerkleTree, SparseMerkleTree};
use aptos_state_view::account_with_state_cache::AsAccountWithStateCache;
use aptos_storage_interface::{cached_state_view::StateCache, state_delta::StateDelta};
use aptos_types::{
    account_config::CORE_CODE_ADDRESS,
    account_view::AccountView,
    epoch_state::EpochState,
    event::EventKey,
    on_chain_config,
    state_store::{
        state_key::StateKey, state_storage_usage::StateStorageUsage, state_value::StateValue,
    },
    transaction::{Transaction, Version},
    write_set::{TransactionWrite, WriteOp, WriteSet},
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::HashMap;

pub static NEW_EPOCH_EVENT_KEY: Lazy<EventKey> = Lazy::new(on_chain_config::new_epoch_event_key);

/// Helper class for calculating `InMemState` after a chunk or block of transactions are executed.
///
/// A new SMT is spawned in two situations:
///   1. a state checkpoint is encountered.
///   2. a transaction chunk or block ended (where `finish()` is called)
///
/// | ------------------------------------------ | -------------------------- |
/// |  (updates_between_checkpoint_and_latest)   |  (updates_after_latest)    |
/// \                                            \                            |
///  checkpoint SMT                               latest SMT                  |
///                                                                          /
///                                (creates checkpoint SMT on checkpoint txn)
///                                        (creates "latest SMT" on finish())
pub struct InMemoryStateCalculatorV2 {
    ///// These don't change during the calculation.
    // This makes sure all in-mem nodes seen while proofs were fetched stays in mem during the
    // calculation
    _frozen_base: FrozenSparseMerkleTree<StateValue>,
    proof_reader: ProofReader,

    //// These changes every time a new txn is added to the calculator.
    state_cache: DashMap<StateKey, Option<StateValue>>,
    next_version: Version,
    updates_after_latest: HashMap<StateKey, Option<StateValue>>,
    usage: StateStorageUsage,

    //// These changes whenever make_checkpoint() happens.
    checkpoint: SparseMerkleTree<StateValue>,
    checkpoint_version: Option<Version>,
    // This doesn't need to be frozen since `_frozen_base` holds a ref to the oldest ancestor
    // already, but frozen SMT is used here anyway to avoid exposing the `batch_update()` interface
    // on the non-frozen SMT.
    latest: FrozenSparseMerkleTree<StateValue>,
}

impl InMemoryStateCalculatorV2 {
    pub fn new(base: &StateDelta, state_cache: StateCache) -> Self {
        let StateCache {
            frozen_base,
            state_cache,
            proofs,
        } = state_cache;
        let StateDelta {
            base,
            base_version,
            current,
            current_version,
            updates_since_base,
        } = base.clone();

        assert!(base_version == current_version);
        assert!(updates_since_base.len() == 0);

        Self {
            _frozen_base: frozen_base,
            proof_reader: ProofReader::new(proofs),

            state_cache,
            next_version: current_version.map_or(0, |v| v + 1),
            updates_after_latest: HashMap::new(),
            usage: current.usage(),

            checkpoint: base,
            checkpoint_version: base_version,
            latest: current.freeze(),
        }
    }

    pub fn calculate_for_transaction_block(
        mut self,
        to_keep: &[(Transaction, ParsedTransactionOutput)],
        new_epoch: bool,
    ) -> Result<(
        Vec<HashMap<StateKey, Option<StateValue>>>,
        Vec<Option<HashValue>>,
        StateDelta,
        Option<EpochState>,
    )> {
        let num_txns = to_keep.len();

        let state_updates_vec = Self::get_state_updates(to_keep);

        let mut state_checkpoint_hashes = Vec::with_capacity(num_txns);
        for i in 0..num_txns - 1 {
            let (txn, txn_output) = to_keep[i];
            assert!(!Self::need_checkpoint(txn, txn_output));
            state_checkpoint_hashes.push(None);
        }

        self.next_version += num_txns as u64;
        let updates_after_latest = state_updates_vec.iter().flatten().collect();
        let (txn, txn_output) = to_keep[num_txns - 1];
        assert!(Self::need_checkpoint(txn, txn_output));

        let state_checkpoint_hash = self.make_checkpoint(updates_after_latest)?;
        state_checkpoint_hashes.push(Some(state_checkpoint_hash));

        // TODO(grao): state cache, usage!!!

        let (result_state, accounts) = self.finish()?;

        // Get the updated validator set from updated account state.
        let next_epoch_state = if new_epoch {
            Some(Self::parse_validator_set(&accounts)?)
        } else {
            None
        };

        Ok((
            state_updates_vec,
            state_checkpoint_hashes,
            result_state,
            next_epoch_state,
        ))
    }

    fn need_checkpoint(txn: Transaction, txn_output: ParsedTransactionOutput) -> bool {
        if txn_output.is_reconfig() {
            return true;
        }
        match txn {
            Transaction::BlockMetadata(_) | Transaction::UserTransaction(_) => false,
            Transaction::GenesisTransaction(_) | Transaction::StateCheckpoint(_) => true,
        }
    }

    fn get_state_updates(
        to_keep: &[(Transaction, ParsedTransactionOutput)],
    ) -> Vec<HashMap<StateKey, Option<StateValue>>> {
        to_keep
            .par_iter()
            .map(|(_, txn_output)| {
                txn_output
                    .write_set()
                    .iter()
                    .map(|(state_key, write_op)| (state_key.clone(), write_op.as_state_value()))
                    .collect()
            })
            .collect()
    }

    fn make_checkpoint(
        &mut self,
        updates_after_latest: HashMap<&StateKey, &Option<StateValue>>,
    ) -> Result<HashValue> {
        // Update SMT.
        let smt_updates: Vec<_> = updates_after_latest
            .iter()
            .map(|(key, value)| (key.hash(), value.as_ref()))
            .collect();
        let new_checkpoint =
            self.latest
                .batch_update(smt_updates, self.usage, &self.proof_reader)?;
        let root_hash = new_checkpoint.root_hash();

        // Move self to the new checkpoint.
        self.latest = new_checkpoint.clone();
        self.checkpoint = new_checkpoint.unfreeze();
        self.checkpoint_version = self.next_version.checked_sub(1);

        Ok(root_hash)
    }

    fn parse_validator_set(state_cache: &HashMap<StateKey, StateValue>) -> Result<EpochState> {
        let account_state_view = state_cache.as_account_with_state_cache(&CORE_CODE_ADDRESS);
        let validator_set = account_state_view
            .get_validator_set()?
            .ok_or_else(|| anyhow!("ValidatorSet not touched on epoch change"))?;
        let configuration = account_state_view
            .get_configuration_resource()?
            .ok_or_else(|| anyhow!("Configuration resource not touched on epoch change"))?;

        Ok(EpochState {
            epoch: configuration.epoch(),
            verifier: (&validator_set).into(),
        })
    }

    fn finish(mut self) -> Result<(StateDelta, HashMap<StateKey, StateValue>)> {
        let result_state = StateDelta::new(
            self.checkpoint,
            self.checkpoint_version,
            self.latest.unfreeze(),
            self.next_version.checked_sub(1),
            HashMap::new(),
        );

        Ok((
            result_state,
            self.state_cache
                .into_iter()
                .filter_map(|(k, v_opt)| v_opt.map(|v| (k, v)))
                .collect(),
        ))
    }
}

fn process_state_key_write_op(
    state_cache: &mut DashMap<StateKey, Option<StateValue>>,
    usage: &mut StateStorageUsage,
    state_key: StateKey,
    write_op: WriteOp,
) -> Result<(StateKey, Option<StateValue>)> {
    let key_size = state_key.size();
    let state_value = write_op.as_state_value();
    if let Some(ref value) = state_value {
        usage.add_item(key_size + value.size())
    }
    let cached = state_cache.insert(state_key.clone(), state_value.clone());
    if let Some(old_value_opt) = cached {
        if let Some(old_value) = old_value_opt {
            usage.remove_item(key_size + old_value.size());
        }
    }
    Ok((state_key, state_value))
}
