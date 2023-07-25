# Apecoin Substreams

Quickstart
Make sure you have the latest versions of the following installed:

Rust Link to install
Graph-cli Link to install
substreams-cli Link to install
make Link to install

Although this substreams is tracking the Apecoin Contract, you can use this as a template for any ERC20 token. You will need to update the CONTRACT_ADDRESS, START_BLOCK and the contract metadata in the `store_token` function.

1. Update the CONTRACT_ADDRESS & START_BLOCK variables in src/utils/constants.rs
pub const CONTRACT_ADDRESS: &str = "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D";

3. Update the START_BLOCK, CONTRACT_ADDRESS and contract metadata in the store_token function

```rust
#[substreams::handlers::store]
pub fn store_token(block: eth::v2::Block, o: StoreSetProto<apecoin::Token>) {
    if block.number == START_BLOCK {
        let token = &apecoin::Token {
            name: "Apecoin".to_string(),
            address: append_0x(Hex(CONTRACT_ADDRESS).to_string().as_str()),
            decimal: "18".to_string(),
            symbol: "Ape".to_string(),
        };
        o.set(0, format!("Token: {}", token.address), &token);
    };
}
```

4. Update the initialBlock params for all modules within substreams.yaml

  - name: map_transfers
    kind: map
    initialBlock: 14204533
    inputs:
      - source: sf.ethereum.type.v2.Block
    output:
      type: proto:apecoin.Transfers

5. Compile the Project with `make build`
We now need to recompile our WASM binary with the new changes we made to the rust files.

6. Pack the spkg with `make package`
We need to bundle the protobuf definitions and the WASM binary into a single file. This is what we will deploy the subgraph.

7. Deploy the subgraph with `graph deploy`
Modify the package.json to point to your subgraph name and endpoint.
This endpoint will change if you are deploying to the hosted service or decentralized network. But replace this with the command that is appropriate for your setup.

8. Enjoy your data!
Schema
type Account @entity {
  id: ID!
  holdings: BigInt!
  sent: [Transfer!]! @derivedFrom(field: "sender")
  received: [Transfer!]! @derivedFrom(field: "receiver")
  approvals: [Approval!]! @derivedFrom(field: "owner")
}

type Transfer @entity(immutable: true) {
  id: ID!
  sender: Account
  receiver: Account!
  amount: String!
  token: Token!
  timestamp: BigInt!
  txHash: String!
  blockNumber: BigInt!
  logIndex: BigInt!
}

type Approval @entity(immutable: true) {
  id: ID!
  spender: String!
  owner: Account!
  amount: String!
  timestamp: BigInt!
  token: Token!
  txHash: String!
  blockNumber: BigInt!
  logIndex: BigInt!
}

type Token @entity {
  id: ID!
  name: String!
  address: String!
  symbol: String!
  decimals: String!
  transfers: [Token!]! derivedFrom(field: "token")
  approvals: [Token!]! derivedFrom(field: "token")
}

7. Data Flow

```mermaid
graph TD;
  map_transfer[map: map_transfer];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_transfer;
  map_approval[map: map_approval];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_approval;
  store_account_holdings[store: store_account_holdings];
  map_transfer --> store_account_holdings;
  store_token[store: store_token];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> store_token;
  graph_out[map: graph_out];
  map_transfer --> graph_out;
  map_approval --> graph_out;
  store_account_holdings -- deltas --> graph_out;
  store_token -- deltas --> graph_out;

```
