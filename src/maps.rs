use substreams_ethereum::pb::eth::v2::Block;
use substreams::errors::Error;
use substreams::{Hex};
use substreams::pb::substreams::Clock;
use crate::abi::graph::functions::GetIndexerStakedTokens;
use crate::pb::allocations::types::v1::{AllocationClosedData, AllocationClosedDatas, StakedTokensChange, StakedTokensChanges};
use substreams_database_change::pb::database::DatabaseChanges;
use crate::abi::graph::events::{AllocationClosed1, AllocationClosed2};
use crate::utils::{GRAPH};

#[substreams::handlers::map]
pub fn map_allocation_closed1(block: Block) -> Result<AllocationClosedDatas, Error> {
    Ok(AllocationClosedDatas {
        allocation_closed_datas: block.events::<AllocationClosed1>(&[&GRAPH]).filter_map(|(event, _)| {
            let allocation_closed = AllocationClosedData {
                address: Hex::encode(&event.indexer),
                deployment_id: Hex::encode(&event.subgraph_deployment_id),
                tokens: event.tokens.to_string(),
                allocation_id: Hex::encode(&event.allocation_id),
                poi: Hex::encode(&event.poi),
            };
            Some(allocation_closed)
        }).collect()
    })
}
#[substreams::handlers::map]
pub fn map_allocation_closed2(block: Block) -> Result<AllocationClosedDatas, Error> {
    Ok(AllocationClosedDatas {
        allocation_closed_datas: block.events::<AllocationClosed2>(&[&GRAPH]).filter_map(|(event, _)| {
            let allocation_closed = AllocationClosedData {
                address: Hex::encode(&event.indexer),
                deployment_id: Hex::encode(&event.subgraph_deployment_id),
                tokens: event.tokens.to_string(),
                allocation_id: Hex::encode(&event.allocation_id),
                poi: Hex::encode(&event.poi),
            };
            Some(allocation_closed)
        }).collect()
    })
}

#[substreams::handlers::map]
pub fn map_allocation_closed(indexers: String, _clock: Clock, allocation_closed1: AllocationClosedDatas, allocation_closed2: AllocationClosedDatas) -> Result<AllocationClosedDatas, Error> {
    let mut all = false;
    if indexers == "*" {
        all = true;
    }

    let indexers_map: std::collections::HashSet<String>;
    if !all {
        let indexers_list: Vec<String> = serde_json::from_str(&indexers).unwrap();
        indexers_map = indexers_list.into_iter().collect();
    } else {
        indexers_map = std::collections::HashSet::new();
    }

    let mut res = Vec::new();

    allocation_closed1.allocation_closed_datas.iter().for_each(|x| {
        if all || indexers_map.contains(&x.address) {
            res.push(x.clone());
        }
    });

    allocation_closed2.allocation_closed_datas.iter().for_each(|x| {
        if all || indexers_map.contains(&x.address) {
            res.push(x.clone());
        }
    });

    Ok(AllocationClosedDatas {
        allocation_closed_datas: res,
    })
}

#[substreams::handlers::map]
pub fn map_staked_tokens_changes(clock: Clock, allocation_closed: AllocationClosedDatas) -> Result<StakedTokensChanges, Error> {
    Ok(StakedTokensChanges {
        staked_tokens_changes: allocation_closed.allocation_closed_datas.iter().filter_map(|allocation_closed| {
            let st = GetIndexerStakedTokens {
                indexer: Hex::decode(&allocation_closed.address).unwrap()
            };
            if let Some(x) = st.call(GRAPH.to_vec()) {
                Some(StakedTokensChange {
                    indexer: allocation_closed.address.clone(),
                    block_number: clock.number,
                    tokens: x.to_string(),
                })
            } else {
                None
            }
        }).collect()
    })
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, staked_tokens_changes: StakedTokensChanges) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for staked_tokens_change in staked_tokens_changes.staked_tokens_changes {
        tables.create_row("staked_tokens_changes", [("indexer", (&staked_tokens_change).indexer.to_string()), ("block_number", block_num.to_string())])
            .set("indexer", staked_tokens_change.indexer)
            .set("tokens", staked_tokens_change.tokens)
            .set("block_timestamp", &timestamp);
    }

    Ok(tables.to_database_changes())
}