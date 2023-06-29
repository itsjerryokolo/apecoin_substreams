mod abi;
mod pb;
mod utils;

use crate::utils::helper::{append_0x, generate_id};
use abi::abi::apecoin::v1 as apecoin_events;

use pb::eth::apecoin::v1 as apecoin;
use substreams::{log, Hex};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth;
use utils::{constants::APECOIN_CONTRACT, math::to_big_decimal};

use substreams::errors::Error;

#[substreams::handlers::map]
fn map_transfer(block: eth::v2::Block) -> Result<apecoin::Transfers, Error> {
    Ok(apecoin::Transfers {
        transfers: block
            .events::<apecoin_events::events::Transfer>(&[&APECOIN_CONTRACT])
            .map(|(transfer, log)| {
                log::info!("Apecoin transfer seen");

                apecoin::Transfer {
                    from: append_0x(&Hex(transfer.from).to_string()),
                    to: append_0x(&Hex(transfer.to).to_string()),
                    block_number: block.number,
                    timestamp: block.timestamp_seconds(),
                    amount: to_big_decimal(transfer.value.to_string().as_str())
                        .unwrap()
                        .to_string(),
                    tx_hash: append_0x(&Hex(&log.receipt.transaction.hash).to_string()),
                    log_index: log.index(),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn graph_out(transfers: apecoin::Transfers) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for transfer in &transfers.transfers {
        let id: String = generate_id(&transfer.tx_hash, &transfer.log_index.to_string().as_str());

        let row = tables.create_row("Transfer", &id);
        row.set("timestamp", transfer.timestamp);
        row.set("blockNumber", transfer.block_number);
        row.set("logIndex", transfer.log_index);
        row.set("txHash", &transfer.tx_hash);
        row.set("amount", &transfer.amount);
        row.set("sender", &transfer.to);
        row.set("receiver", &transfer.from);
    }
    let entity_changes = tables.to_entity_changes();
    log::info!("Entity changes: {:?}", entity_changes);
    Ok(entity_changes)
}
