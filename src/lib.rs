mod abi;
mod pb;
mod utils;

use crate::utils::helper::{append_0x, generate_id};
use abi::abi::apecoin::v1 as apecoin_events;

use pb::eth::apecoin::v1 as apecoin;
use substreams::pb::substreams::store_delta::Operation;
use substreams::{
    log,
    store::{DeltaProto, Deltas, StoreNew, StoreSet, StoreSetProto},
    Hex,
};

use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth;
use utils::constants::APECOIN_CONTRACT;

use substreams::errors::Error;

substreams_ethereum::init!();

#[substreams::handlers::map]
pub fn map_transfer(block: eth::v2::Block) -> Result<apecoin::Transfers, Error> {
    Ok(apecoin::Transfers {
        transfers: block
            .events::<apecoin_events::events::Transfer>(&[&APECOIN_CONTRACT])
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
            .events::<apecoin_events::events::Approval>(&[&APECOIN_CONTRACT])
            .map(|(approval, log)| {
                log::info!("Apecoin transfer seen");

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
pub fn store_accounts(
    i: apecoin::Approvals,
    i2: apecoin::Transfers,
    o: StoreSetProto<apecoin::Account>,
) {
    for approval in i.approvals {
        o.set(
            0,
            format!("Owner: {}", &approval.owner.as_ref().unwrap().address),
            &approval.owner.as_ref().unwrap(),
        );
    }
    for transfer in i2.transfers {
        o.set(
            0,
            format!("Sender: {}", &transfer.from.as_ref().unwrap().address),
            &transfer.from.as_ref().unwrap(),
        );

        o.set(
            0,
            format!("Receiver: {}", &transfer.to.as_ref().unwrap().address),
            &transfer.to.as_ref().unwrap(),
        );
    }
}

#[substreams::handlers::store]
pub fn store_token(block: eth::v2::Block, o: StoreSetProto<apecoin::Token>) {
    if block.number == 14204533 as u64 {
        let token = &apecoin::Token {
            name: "Apecoin".to_string(),
            address: append_0x(Hex(APECOIN_CONTRACT).to_string().as_str()),
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
    accounts: Deltas<DeltaProto<apecoin::Account>>,
    tokens: Deltas<DeltaProto<apecoin::Token>>,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for transfer in &transfers.transfers {
        let id: String = generate_id(&transfer.tx_hash, &transfer.log_index.to_string().as_str());
        let row = tables.create_row("Transfer", &id);

        row.set("sender", &transfer.from.as_ref().unwrap().address);
        row.set("receiver", &transfer.to.as_ref().unwrap().address);
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
        row.set("blockNumber", approval.block_number);
        row.set("logIndex", approval.log_index);
        row.set("txHash", &approval.tx_hash);
        row.set("amount", &approval.amount);
    }

    for delta in accounts.deltas {
        let address = delta.key.as_str().split(":").last().unwrap().trim();

        match delta.operation {
            Operation::Create => tables.create_row("Account", address),
            Operation::Update => continue,
            Operation::Delete => todo!(),
            x => panic!("unsupported operation {:?}", x),
        };
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
