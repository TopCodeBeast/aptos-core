---
title: "Fungible Asset"
id: "fungible-asset"
---
import ThemedImage from '@theme/ThemedImage';
import useBaseUrl from '@docusaurus/useBaseUrl';

# Fungible Asset

Fungible assets (FA) are an essential part of the Aptos ecosystem, as they enable the creation and transfer of fungible
units, which can represent different things, such as currency, shares, material in games, or any other type of asset.
Fungible assets can be used to build decentralized applications that require a token economy, such as decentralized
exchanges or gaming platforms.

[Fungible asset module](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-framework/sources/fungible_asset.move)
provides a standard, typesafe framework for assets with fungibility.

In this standard, fungible assets are stored in `Object<FungibleStore>` that has a specific amount of units, which
can be transferred, burned, or minted. Fungible assets are units that are interchangeable with others of the same
metadata. The standard is built upon object model so all the resources defined here are included in object resource
group and stored inside objects.

## Structures

### Metadata Object

FA metadata defines the type of FA. A metadata is defined in Move as:

```rust
#[resource_group_member(group = aptos_framework::object::ObjectGroup)]
struct Metadata has key {
/// Optional tracking of the current supply of the fungible asset.
supply: Option<Supply>,
/// Name of the fungible metadata, i.e., "USDT".
name: String,
/// Symbol of the fungible metadata, usually a shorter version of the name.
/// For example, Singapore Dollar is SGD.
symbol: String,
/// Number of decimals used for display purposes.
/// For example, if `decimals` equals `2`, a balance of `505` coins should
/// be displayed to a user as `5.05` (`505 / 10 ** 2`).
decimals: u8,
}
```

:::tip
This is similar to `CoinType` in coin standard but with richer info. The most significant difference is that even two
exactly the same metadata object denote two types instead of one.
:::

A Coin uses the `CoinType` to support re-usability of the Coin framework for distinct Coins. For example, `Coin<A>`
and `Coin<B>` are two distinct coins.

### Fungible Asset and Fungible Store

Since metadata is specified, FA could be defined as follows:

```rust
struct FungibleAsset {
    metadata: Object<Metadata>,
    amount: u64,
}
```

Simple, right? An object representing the type and the amount of units held. It is noted that it does not have has
no abilities, hence it can only be alive in execution runtime. In other words, it must be consumed and cannot be
directly stored anywhere. But how to store it? Here comes `FungibleStore` for storing them in objects:

```rust
#[resource_group_member(group = aptos_framework::object::ObjectGroup)]
struct FungibleStore has key {
/// The address of the base metadata object.
metadata: Object<Metadata>,
/// The balance of the fungible metadata.
balance: u64,
/// Fungible Assets transferring is a common operation, this allows for freezing/unfreezing accounts.
allow_ungated_balance_transfer: bool,
}
```

The only extra field added here is `allow_ungated_balance_transfer`. if it is `true`, this object is frozen, i.e.
deposit and withdraw are both disabled without using `TransferRef` in the next section.

### References

Reference(ref) is the means to implement permission control across different standard in Aptos. In different contexts,
it may be called capabilities sometimes. In FA standard, there are three refs for Mint, Transfer and Burn operations on
FA of the same metadata specified in the ref struct.

```rust
struct MintRef has drop, store {
metadata: Object<Metadata>
}

struct TransferRef has drop, store {
metadata: Object<Metadata>
}

struct BurnRef has drop, store {
metadata: Object<Metadata>
}
```

- `MintRef` offers the capability to mint FA.
- `TransferRef` offers the capability to mutate the value of `allow_ungated_balance_transfer` in any
  `FungbibleStore` of the same metadata or transfer FA by ignoring `allow_ungated_balance_transfer`.
- `MintRef` offers the capability to mint FA.

The three refs collectively act as the building blocks of various permission control system as they have `store` so
can be passed around and stored anywhere. Please refer to the source file for `mint()`, `mint_to()`, `burn()`,
`burn_from()`, `withdraw_with_ref()`, `deposit_with_ref()`, and `transfer_with_ref()`: These functions are used to
mint, burn, withdraw, deposit, and transfer FA using the MintRef, BurnRef, and TransferRef.

It is worth noting that these functions are only a part of a larger smart contract system, and they need to be  
integrated with other functions and modules to create a complete system. Developers who want to use these functions  
should familiarize themselves with the smart contract system they are working with and ensure that they understand  
how these functions fit into the larger architecture.

### Creators

FA creators can:

- Initialize a FA metadata object based on which FA can be minted.

```rust
public fun add_fungibility(
constructor_ref: & ConstructorRef,
monitoring_supply_with_maximum: Option<Option<u128> >,
name: String,
symbol: String,
decimals: u8,
): Object<Metadata>
```

### Ref Owners (Managers)

Ref owners can do the following operations depending on the refs they own:

- Mint (to account) and/or burn (from account).
- Modify `allow_ungated_balance_transfer` of any `FungibleStore` with the same metadata object.
- Transfer FA ignoring `allow_ungated_balance_transfer`.

### Users

Coin users can:

- Merging two FAs of the same metadata object.
- Extracting FA from a fungible store into another.
- Ability to deposit and withdraw from a `FungibleStore` and emit events as a result.
- Allows for users to register a `CoinStore<CoinType>` in their account to handle coin.

### Creating

A FA creator can add fungibility to any object at creation by taking `&ConstructorRef` with required information to
make that object a metadata of the associated FA. Then FA of this metadata can be minted and used.
new `CoinType`.

```rust
public fun add_fungibility(
constructor_ref: & ConstructorRef,
monitoring_supply_with_maximum: Option<Option<u128> >,
name: String,
symbol: String,
decimals: u8,
): Object<Metadata>
```

The creator has the opportunity to define a name, symbol, decimals, and whether or not the total supply for the FA is
monitored. The following applies:

- The first three of the above (`name`, `symbol`, `decimals`)  are purely metadata and have no impact for on-chain
  applications. Some applications may use decimal to equate a single Coin from fractional coin.
- Monitoring supply (`monitor_supply`) helps track total FA in supply. However, due to the way the parallel executor
  works, turning on this option will prevent any parallel execution of mint and burn. If the coin will be regularly
  minted or burned, consider disabling `monitor_supply`.

### Primitives

At creation, the creator has the option to generate refs from the same `&ConstructorRef` to manage FA. These will need
to be stored in global storage to be used later.

#### Mint

If the manager would like to mint FA, they must retrieve a reference to `MintRef`, and call:

```rust
public fun mint( ref: & MintRef, amount: u64): FungibleAsset
```

This will produce a new FA of the metadata in the ref, containing a value as dictated by the `amount`. If supply is
tracked, then it will also be adjusted. There is also a `mint_to` function that also deposits to a `FungibleStore`
after minting as a helper.

#### Burn

The opposite operation of minting. Likewise, a reference to `BurnRef` is required and call:

```rust
public fun burn( ref: & BurnRef, fa: FungibleAsset)
```

This will reduce the passed-in `fa` to ashes as your will. There is also a `burn_from` function that forcibly withdraw
from an account first and then burn the fa withdrawn as a helper.

#### Transfer and Freeze/Unfreeze

`TransferRef` has two functions:

- Flip `ungated_balance_transfer_allowed` in `FungibleStore` holding FA of the same metadata in the `TransferRef`. if
  it is false, the store is "frozen" that nobody can deposit to or withdraw from this store without using the ref.
- Withdraw from or deposit to a store ignoring its `ungated_balance_transfer_allowed`.

To change `ungated_balance_transfer_allowed`, call:

```rust
public fun set_ungated_transfer<T: key>(
ref: & TransferRef,
store: Object<T>,
allow: bool,
) acquires FungibleStore, FungibleAssetEvents
```

:::tip
This function will emit a `SetUngatedBalanceTransferEvent`.
:::

To forcibly withdraw, call:

```Rust
public fun withdraw_with_ref<T: key>(
ref: & TransferRef,
store: Object<T>,
amount: u64
): FungibleAsset
```

:::tip
This function will emit a `WithdrawEvent`.
:::

To forcibly deposit, call

```rust
public fun deposit_with_ref<T: key>(
ref: & TransferRef,
store: Object<T>,
fa: FungibleAsset
)
```

:::tip
This function will emit a `DepositEvent`.
:::

There is a function named `transfer_with_ref` that combining `withdraw_with_ref` and `deposit_with_ref` together as
a helper.

#### Merging Fungible Assets

Two FAs of the same type can be merged into a single struct that represents the accumulated value of the two  
independently by calling:

```rust
public fun merge(dst_fungible_asset: & mut FungibleAsset, src_fungible_asset: FungibleAsset)
```

After merging, `dst_fungible_asset` will have all the amounts.

#### Extracting Fungible Asset

A FA can have amount deducted to create another FA by calling:

```rust
public fun extract(fungible_asset: & mut FungibleAsset, amount: u64): FungibleAsset
```

:::tip
This function may produce FA with 0 amount, which is not usable. It is supposed to be merged with other FA or destroyed
through `destroy_zero()` in the module.
:::

#### Withdraw from FungibleStore

The owner of a `FungibleStore` object can extract FA with a specified amount if `ungated_balance_transfer_allowed` is
true, by calling:

```rust
public fun withdraw<T: key>(owner: & signer, store: Object<T>, amount: u64): FungibleAsset
```

:::tip
This function will emit a `WithdrawEvent`.
:::

#### Deposit to FungibleStore

Any entity can deposit FA into a `FungibleStore` object if `ungated_balance_transfer_allowed` is true, by calling:

```rust
public fun deposit<T: key>(store: Object<T>, fa: FungibleAsset)
```

:::tip
This function will emit a `DepositEvent`.
:::

#### Transfer between FungibleStore

The owner of a `CoinStore` can directly transfer FA from that store to another if `ungated_balance_transfer_allowed`
is true by calling:

```rust
public entry fun transfer<T: key>(sender: & signer, from: Object<T>, to: Object<T>, amount: u64)
```

:::tip
This will emit both `WithdrawEvent` and `DepositEvent` on the respective `Fungibletore`s.
:::

## Events

### DepositEvent

Emitted when fungible assets are deposited into a store.

```rust
struct DepositEvent has drop, store {
amount: u64,
}
```

### WithdrawEvent

Emitted when fungible assets are withdrawn from a store.

```rust
struct WithdrawEvent has drop, store {
amount: u64,
}
```

### SetUngatedBalanceTransferEvent

Emitted when a store's `ungated_balance_transfer_allowed` is updated.

```rust
struct SetUngatedTransferEvent has drop, store {
transfer_allowed: bool,
}
```

# Primary FungibleStore

Each FungibleStore object has an owner. However, an owner may possess more than one store. When Alice sends FA to
Bob, how does she determine the correct destination? Additionally, what happens if Bob doesn't have a store yet?

To address these questions, the standard has been expanded to define primary and secondary stores.

- Each account owns only one undeletable primary FungibleStore, the address of which is deterministically derived from
  the account address and metadata object address. If primary store does not exist, it will be created if FA is going
  to be deposited by calling functions defined in `primary_fungible_store.move`
- Secondary stores do not have deterministic address and theoretically deletable. Users are able to create as many
  secondary stores as they want using the provided functions but they have to take care of the indexing by themselves.

The vast majority of users will have primary store as their only store for a specific type of fungible assets. It is
expected that secondary stores would be useful in complicated defi or other asset management contracts.

## How to enable Primary FungibleStore?

To add primary store support, when creating metadata object, instead of aforementioned `add_fungibility()`, creator
has to call:

```rust
public fun create_primary_store_enabled_fungible_asset(
constructor_ref: & ConstructorRef,
monitoring_supply_with_maximum: Option<Option<u128> >,
name: String,
symbol: String,
decimals: u8,
)
```

The parameters are the same as those of `add_fungibility()`.

## Primitives

### Get Primary FungibleStore

To get the primary store object of a metadata object belonging to an account, call:

```rust
public fun primary_store<T: key>(owner: address, metadata: Object<T>): Object<FungibleStore>
```

:::tip
There are other utility functions. `primary_store_address` returns the deterministic address the primary store,
and `primary_store_exists` checks the existence, etc.
:::

### Manually Create Primary FungibleStore

If a primary store does not exist, any entity is able to create it by calling:

```rust
public fun create_primary_store<T: key>(owner_addr: address, metadata: Object<T>): Object<FungibleStore>
```

### Check Balance and Frozen Status

To check the balance of a primary store, call:

```rust
public fun balance<T: key>(account: address, metadata: Object<T>): u64
```

To check the value of `ungated_balance_transfer_allowed`, call:

```rust
 public fun ungated_balance_transfer_allowed<T: key>(account: address, metadata: Object<T>): bool
```

### Withdraw

An owner can withdraw FA from their primary store by calling:

```rust
public fun withdraw<T: key>(owner: & signer, metadata: Object<T>, amount: u64): FungibleAsset
```

### Deposit

An owner can deposit FA to their primary store by calling:

```rust
public fun deposit(owner: address, fa: FungibleAsset)
```

### Transfer

An owner can deposit FA from their primary store to that of another account by calling:

```rust
public entry fun transfer<T: key>(sender: & signer, metadata: Object<T>, recipient: address, amount: u64)
```