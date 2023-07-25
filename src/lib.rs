mod abi;
mod pb;
mod utils;

use crate::utils::helper::{append_0x, generate_id};
use abi::abi::apecoin::v1 as apecoin_events;

use pb::eth::apecoin::v1 as apecoin;
use substreams::pb::substreams::store_delta::Operation;
use substreams::store::{DeltaBigDecimal, StoreAdd, StoreAddBigDecimal};
use substreams::{
    log,
    store::{DeltaProto, Deltas, StoreNew, StoreSet, StoreSetProto},
    Hex,
};

use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth;
use utils::constants::{CONTRACT_ADDRESS, START_BLOCK};

use substreams::errors::Error;
use utils::math::to_big_decimal;

#[substreams::handlers::map]
pub fn map_transfer(block: eth::v2::Block) -> Result<apecoin::Transfers, Error> {
    Ok(apecoin::Transfers {
        transfers: block
            .events::<apecoin_events::events::Transfer>(&[&CONTRACT_ADDRESS])
            .map(|(transfer, log)| {
                log::info!("Apecoin transfer seen");

                apecoin::Transfer {
                    from: Some(apecoin::Account {
                        address: append_0x(&Hex(transfer.from).to_string()),
                    }),
                    to: Some(apecoin::Account {
                        address: append_0x(&Hex(transfer.to).to_string()),
                    }),
                    block_number: block.number,
                    timestamp: block.timestamp_seconds(),
                    amount: transfer.value.to_string(),
                    tx_hash: append_0x(&Hex(&log.receipt.transaction.hash).to_string()),
                    log_index: log.index(),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn map_approval(block: eth::v2::Block) -> Result<apecoin::Approvals, Error> {
    Ok(apecoin::Approvals {
        approvals: block
            .events::<apecoin_events::events::Approval>(&[&CONTRACT_ADDRESS])
            .map(|(approval, log)| {
                log::info!("Apecoin approval seen");

                apecoin::Approval {
                    spender: append_0x(&Hex(approval.spender).to_string()),
                    owner: Some(apecoin::Account {
                        address: append_0x(&Hex(approval.owner).to_string()),
                    }),
                    block_number: block.number,
                    timestamp: block.timestamp_seconds(),
                    amount: approval.value.to_string(),
                    tx_hash: append_0x(&Hex(&log.receipt.transaction.hash).to_string()),
                    log_index: log.index(),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::store]
pub fn store_account_holdings(i0: apecoin::Transfers, o: StoreAddBigDecimal) {
    for transfer in i0.transfers {
        let amount_decimal = to_big_decimal(transfer.amount.as_str()).unwrap();
        o.add(
            0,
            format!("Account: {}", &transfer.from.as_ref().unwrap().address),
            amount_decimal.neg(),
        );

        o.add(
            0,
            format!("Account: {}", &transfer.to.as_ref().unwrap().address),
            amount_decimal,
        );
    }
}

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

#[substreams::handlers::map]
pub fn graph_out(
    transfers: apecoin::Transfers,
    approvals: apecoin::Approvals,
    account_holdings: Deltas<DeltaBigDecimal>,
    tokens: Deltas<DeltaProto<apecoin::Token>>,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    for delta in account_holdings.deltas {
        let address = delta.key.as_str().split(":").last().unwrap().trim();

        match delta.operation {
            Operation::Create => {
                let row = tables.create_row("Account", address);

                row.set("holdings", delta.old_value);
            }
            Operation::Update => {
                let row = tables.update_row("Account", address);
                row.set("holdings", delta.new_value);
            }
            Operation::Delete => todo!(),
            x => panic!("unsupported operation {:?}", x),
        };
    }

    for transfer in &transfers.transfers {
        let id: String = generate_id(&transfer.tx_hash, &transfer.log_index.to_string().as_str());
        let row = tables.create_row("Transfer", &id);

        row.set("sender", &transfer.from.as_ref().unwrap().address);
        row.set("receiver", &transfer.to.as_ref().unwrap().address);
        row.set(
            "token",
            append_0x(Hex(CONTRACT_ADDRESS).to_string().as_str()),
        );
        row.set("timestamp", transfer.timestamp);
        row.set("blockNumber", transfer.block_number);
        row.set("logIndex", transfer.log_index);
        row.set("txHash", &transfer.tx_hash);
        row.set("amount", &transfer.amount);
    }

    for approval in &approvals.approvals {
        let id: String = generate_id(&approval.tx_hash, &approval.log_index.to_string().as_str());
        let row = tables.create_row("Approval", &id);

        row.set("owner", &approval.owner.as_ref().unwrap().address);
        row.set("timestamp", approval.timestamp);
        row.set("spender", &approval.spender);
        row.set(
            "token",
            append_0x(Hex(CONTRACT_ADDRESS).to_string().as_str()),
        );
        row.set("blockNumber", approval.block_number);
        row.set("logIndex", approval.log_index);
        row.set("txHash", &approval.tx_hash);
        row.set("amount", &approval.amount);
    }

    for delta in tokens.deltas {
        match delta.operation {
            Operation::Create => {
                let token_row = tables.create_row("Token", &delta.new_value.address);
                token_row.set("name", delta.new_value.name);
                token_row.set("address", delta.new_value.address);
                token_row.set("decimals", delta.new_value.decimal);
                token_row.set("symbol", delta.new_value.symbol);
            }
            Operation::Update => todo!(),
            Operation::Delete => todo!(),
            x => panic!("unsupported opeation {:?}", x),
        };
    }

    let entity_changes = tables.to_entity_changes();
    Ok(entity_changes)
}
