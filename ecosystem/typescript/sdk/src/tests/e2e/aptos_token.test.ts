import { AptosAccount } from "../../account";
import { AptosToken } from "../../plugins";
import { Provider } from "../../providers";
import { APTOS_COIN } from "../../utils";
import { NODE_URL, getFaucetClient, longTestTimeout } from "../unit/test_helper.test";

const provider = new Provider({ fullnodeUrl: NODE_URL, indexerUrl: NODE_URL });
const faucetClient = getFaucetClient();
const aptosToken = new AptosToken(provider);

const alice = new AptosAccount();
const bob = new AptosAccount();

const collectionName = "AliceCollection";
const tokenName = "Alice Token";

describe("token objects", () => {
  beforeAll(async () => {
    // Fund Alice's Account
    await faucetClient.fundAccount(alice.address(), 100000000);
    console.log(alice);
  }, longTestTimeout);

  test(
    "create collection",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.createCollection(
          alice,
          "Alice's simple collection",
          collectionName,
          "https://aptos.dev",
          5,
          10,
          10,
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "mint",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.mint(
          alice,
          collectionName,
          "Alice's simple token",
          tokenName,
          "https://aptos.dev/img/nyan.jpeg",
          ["key"],
          ["bool"],
          ["true"],
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "mint soul bound",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.mintSoulBound(
          alice,
          collectionName,
          "Alice's simple soul bound token",
          "Alice's soul bound token",
          "https://aptos.dev/img/nyan.jpeg",
          bob,
          ["key"],
          ["bool"],
          ["true"],
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "freeze transfer",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.freezeTokenTransafer(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "unfreeze token transfer",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.unfreezeTokenTransafer(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "set token description",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.setTokenDescription(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
          "my updated token description",
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "set token name",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.setTokenName(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
          "my updated token name",
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "set token uri",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.setTokenName(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
          "https://aptos.dev/img/hero.jpg",
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "add token property",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.addTokenProperty(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
          "newKey",
          "bool",
          "true",
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "update token property",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.updateTokenProperty(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
          "newKey",
          "u64",
          "0",
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "remove token property",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.removeTokenProperty(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
          "newKey",
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "transfer token ownership",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.transferTokenOwnership(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
          bob.address(),
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "burn token",
    async () => {
      await provider.waitForTransaction(
        await aptosToken.burnToken(
          alice,
          AptosAccount.getTokenObjectAddress(alice.address().hex(), collectionName, tokenName),
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "transfer token amount",
    async () => {
      // create moon coin
      // alice to mint a coin
      // alice to transfer 1 moin coin to bob
      await provider.waitForTransaction(
        await aptosToken.transferTokenAmount(
          alice,
          "0x0000000000000000000000000000000000000000000000000000000000000001",
          bob.address(),
          1,
          APTOS_COIN,
        ),
        {
          checkSuccess: true,
        },
      );
    },
    longTestTimeout,
  );
});
