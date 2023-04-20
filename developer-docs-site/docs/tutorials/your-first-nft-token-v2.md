--- 
title: "Mint NFT with the token v2 SDKs"
slug: "your-first-nft-token-v2"
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';
import ThemedImage from '@theme/ThemedImage';
import useBaseUrl from '@docusaurus/useBaseUrl';

# Mint Token v2 NFTs with the Aptos SDKs

This tutorial describes how to create and transfer NFTs on the Aptos blockchain. The Aptos implementation for core NFTs can be found in [aptos_token.move](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-token-objects/sources/aptos_token.move) Move module.

## Step 1: Pick an SDK

Install your preferred SDK from the below list:

* [TypeScript SDK](../sdks/ts-sdk/index.md)
* [Python SDK](../sdks/python-sdk.md)

---

## Step 2: Run the example

Each SDK provides an `examples` directory. This tutorial covers the `simple-nft-v2` example. 

Clone the `aptos-core` repo: 
```bash 
git clone git@github.com:aptos-labs/aptos-core.git ~/aptos-core
```

<Tabs groupId="sdk-examples">
  <TabItem value="typescript" label="Typescript">

Navigate to the Typescript SDK examples directory:
  ```bash
  cd ~/aptos-core/ecosystem/typescript/sdk/examples/typescript
  ```

Install the necessary dependencies:
  ```bash
  pnpm install
  ```

Run the Typescript [`simple_nft`](https://github.com/aptos-labs/aptos-core/blob/main/ecosystem/typescript/sdk/examples/typescript/simple_nft_v2.ts) example:
  ```bash
  pnpm run simple_nft_v2 
  ```
  </TabItem>
  <TabItem value="python" label="Python">

Navigate to the Python SDK directory:
  ```bash
  cd ~/aptos-core/ecosystem/python/sdk
  ```

Install the necessary dependencies:
  ```bash
  curl -sSL https://install.python-poetry.org | python3
  poetry update
  ```

Run the Python [`simple-nft`](https://github.com/aptos-labs/aptos-core/blob/main/ecosystem/python/sdk/examples/simple-nft-v2.py) example:
  ```bash
  poetry run python -m examples.simple-nft-v2
  ```
  </TabItem>

