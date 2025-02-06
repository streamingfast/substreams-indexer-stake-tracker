use crate::abi::graph::events::{
    AllocationClosed1, AllocationClosed2, RebateCollected, StakeDeposited, StakeWithdrawn,
};
use crate::abi::graph::functions::GetIndexerStakedTokens;
use crate::abi::rewards::events::RewardsAssigned;
use crate::abi::transfer::events::Transfer;
use crate::pb::allocations::types::v1::{
    AllocationClosedData, AllocationClosedDatas, IndexingRewardsCollected,
    IndexingRewardsCollecteds, OwnedStakeTokenChange, OwnedStakeTokenChanges, QueryFeesCollected,
    QueryFeesCollecteds, StakedTokensChange, StakedTokensChanges,
};
use crate::utils::GRAPH;
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt, StoreNew};
use substreams::{log, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_query_fees_collected(
    indexers: String,
    block: Block,
) -> Result<QueryFeesCollecteds, Error> {
    let mut out = Vec::new();

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

    block.transactions().for_each(|tx| {
        // match event RebateCollected
        tx.logs_with_calls().for_each(|(log, _)| {
            if log.address == GRAPH && RebateCollected::match_log(log) {
                let collection = RebateCollected::decode(log).unwrap();
                let query_fees_collected = QueryFeesCollected {
                    indexer: Hex::encode(&collection.indexer),
                    deployment_id: Hex::encode(&collection.subgraph_deployment_id),
                    allocation_id: Hex::encode(&collection.allocation_id),
                    query_fees: collection.query_rebates.to_string(), //this is not a bug. what is given to the indexer is the "query rebates" field
                };

                if !all && !indexers_map.contains(&query_fees_collected.indexer) {
                    return;
                }

                log::info!(
                    "map_query_fees_collected: indexer: {}, query_fees: {}",
                    query_fees_collected.indexer,
                    query_fees_collected.query_fees
                );
                out.push(query_fees_collected);
            }
        });
    });

    Ok(QueryFeesCollecteds {
        query_fees_collecteds: out,
    })
}

#[substreams::handlers::map]
pub fn map_indexing_rewards_collected(
    indexers: String,
    block: Block,
) -> Result<IndexingRewardsCollecteds, Error> {
    let mut out = Vec::new();

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

    block.transactions().for_each(|tx| {
        let mut match_graph_address = false;
        let mut rewards_assigned_found = false;
        let mut transfer_found = false;

        tx.logs_with_calls().for_each(|(log, _)| {
            if log.address == GRAPH {
                match_graph_address = true;
            }

            if RewardsAssigned::match_log(log) {
                rewards_assigned_found = true;
                return;
            }

            if Transfer::match_log(log) {
                transfer_found = true;
            }
        });

        if !match_graph_address || !rewards_assigned_found || !transfer_found {
            return;
        }

        tx.logs_with_calls().for_each(|(log, _)| {
            if Transfer::match_log(log) {
                let transfer = Transfer::decode(log).unwrap();
                if transfer.from != GRAPH {
                    return;
                }

                let indexing_rewards_collected = IndexingRewardsCollected {
                    indexer: Hex::encode(&transfer.to),
                    rewards: transfer.value.to_string(),
                };
                if !all && !indexers_map.contains(&indexing_rewards_collected.indexer) {
                    return;
                }

                log::info!(
                    "map_query_fees_collected: indexer: {}, rewards: {}",
                    indexing_rewards_collected.indexer,
                    indexing_rewards_collected.rewards
                );
                out.push(indexing_rewards_collected);
            }
        });
    });

    Ok(IndexingRewardsCollecteds {
        indexing_rewards_collecteds: out,
    })
}

#[substreams::handlers::store]
pub fn store_indexing_rewards_collected(
    indexing_rewards_collecteds: IndexingRewardsCollecteds,
    store: StoreAddBigInt,
) {
    indexing_rewards_collecteds
        .indexing_rewards_collecteds
        .iter()
        .for_each(|x| {
            log::info!(
                "store_indexing_rewards_collected: indexer: {}, rewards: {}",
                x.indexer,
                x.rewards
            );
            store.add(
                0,
                &x.indexer,
                &BigInt::from_str(&x.rewards).unwrap_or(BigInt::zero()),
            );
        });
}

#[substreams::handlers::store]
pub fn store_query_fees_collected(
    query_fees_collecteds: QueryFeesCollecteds,
    store: StoreAddBigInt,
) {
    query_fees_collecteds
        .query_fees_collecteds
        .iter()
        .for_each(|x| {
            log::info!(
                "store_query_fees_collected: indexer: {}, query_fees: {}",
                x.indexer,
                x.query_fees
            );
            store.add(
                0,
                &x.indexer,
                &BigInt::from_str(&x.query_fees).unwrap_or(BigInt::zero()),
            );
        });
}

#[substreams::handlers::map]
pub fn map_stake_deposited(block: Block) -> Result<OwnedStakeTokenChanges, Error> {
    let mut owned_stake_token_changes = Vec::new();

    block.transactions().for_each(|tx| {
        if tx.status != 1 {
            return;
        }

        let mut allocations_closed_found = false;
        let mut transfer_found = false;
        let mut rewards_assigned_found = false;

        tx.logs_with_calls().for_each(|(log, _)| {
            if AllocationClosed1::match_log(log) || AllocationClosed2::match_log(log) {
                allocations_closed_found = true;
                return;
            }

            if RewardsAssigned::match_log(log) {
                rewards_assigned_found = true;
                return;
            }

            if Transfer::match_log(log) {
                transfer_found = true;
            }
        });

        if !allocations_closed_found && !rewards_assigned_found && transfer_found {
            tx.logs_with_calls().for_each(|(log, _)| {
                if StakeDeposited::match_log(log) {
                    let stake_deposited = OwnedStakeTokenChange {
                        indexer: Hex::encode(&StakeDeposited::decode(log).unwrap().indexer),
                        tokens_amount: StakeDeposited::decode(log).unwrap().tokens.to_string(),
                    };

                    owned_stake_token_changes.push(stake_deposited);
                }
            });
        }
    });

    Ok(OwnedStakeTokenChanges {
        owned_stake_token_changes,
    })
}
#[substreams::handlers::map]
pub fn map_stake_withdrawn(block: Block) -> Result<OwnedStakeTokenChanges, Error> {
    Ok(OwnedStakeTokenChanges {
        owned_stake_token_changes: block
            .events::<StakeWithdrawn>(&[&GRAPH])
            .filter_map(|(event, _)| {
                let stake_deposited = OwnedStakeTokenChange {
                    indexer: Hex::encode(&event.indexer),
                    tokens_amount: format!("-{}", event.tokens.to_string()),
                };
                Some(stake_deposited)
            })
            .collect(),
    })
}

#[substreams::handlers::store]
pub fn store_stake_token_changes(
    indexers: String,
    stake_deposited: OwnedStakeTokenChanges,
    stake_withdrawn: OwnedStakeTokenChanges,
    store: StoreAddBigInt,
) {
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

    let mut map: HashMap<String, BigInt> = std::collections::HashMap::new();

    stake_deposited
        .owned_stake_token_changes
        .iter()
        .for_each(|x| {
            if !all && !indexers_map.contains(&x.indexer) {
                return;
            }

            if let Some(val) = map.get_mut(&x.indexer) {
                let a = val.clone();
                let b = BigInt::from_str(&x.tokens_amount).unwrap();
                let new_value = a.add(b);
                map.insert(x.indexer.clone(), new_value.clone());
            } else {
                map.insert(
                    x.indexer.clone().to_lowercase(),
                    BigInt::from_str(&x.tokens_amount).unwrap_or(BigInt::zero()),
                );
            }
        });

    stake_withdrawn
        .owned_stake_token_changes
        .iter()
        .for_each(|x| {
            if !all && !indexers_map.contains(&x.indexer) {
                return;
            }

            if let Some(val) = map.get_mut(&x.indexer) {
                let a = val.clone();
                let b = BigInt::from_str(&x.tokens_amount).unwrap();
                let new_value = a.add(b);
                map.insert(x.indexer.clone(), new_value.clone());
            } else {
                map.insert(
                    x.indexer.clone().to_lowercase(),
                    BigInt::from_str(&x.tokens_amount).unwrap_or(BigInt::zero()),
                );
            }
        });

    map.iter().for_each(|(k, v)| {
        store.add(0, &k, &v);
    });
}

#[substreams::handlers::map]
pub fn map_allocation_closed1(block: Block) -> Result<AllocationClosedDatas, Error> {
    Ok(AllocationClosedDatas {
        allocation_closed_datas: block
            .events::<AllocationClosed1>(&[&GRAPH])
            .filter_map(|(event, log)| {
                let allocation_closed = AllocationClosedData {
                    address: Hex::encode(&event.indexer),
                    deployment_id: Hex::encode(&event.subgraph_deployment_id),
                    tokens: event.tokens.to_string(),
                    allocation_id: Hex::encode(&event.allocation_id),
                    poi: Hex::encode(&event.poi),
                    transaction_hash: Hex::encode(&log.receipt.transaction.hash),
                };
                Some(allocation_closed)
            })
            .collect(),
    })
}
#[substreams::handlers::map]
pub fn map_allocation_closed2(block: Block) -> Result<AllocationClosedDatas, Error> {
    Ok(AllocationClosedDatas {
        allocation_closed_datas: block
            .events::<AllocationClosed2>(&[&GRAPH])
            .filter_map(|(event, log)| {
                let allocation_closed = AllocationClosedData {
                    address: Hex::encode(&event.indexer).to_lowercase(),
                    deployment_id: Hex::encode(&event.subgraph_deployment_id),
                    tokens: event.tokens.to_string(),
                    allocation_id: Hex::encode(&event.allocation_id),
                    poi: Hex::encode(&event.poi),
                    transaction_hash: Hex::encode(&log.receipt.transaction.hash),
                };
                Some(allocation_closed)
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn map_allocation_closed(
    indexers: String,
    _clock: Clock,
    allocation_closed1: AllocationClosedDatas,
    allocation_closed2: AllocationClosedDatas,
) -> Result<AllocationClosedDatas, Error> {
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

    allocation_closed1
        .allocation_closed_datas
        .iter()
        .for_each(|x| {
            if all || indexers_map.contains(&x.address) {
                res.push(x.clone());
            }
        });

    allocation_closed2
        .allocation_closed_datas
        .iter()
        .for_each(|x| {
            if all || indexers_map.contains(&x.address) {
                res.push(x.clone());
            }
        });

    Ok(AllocationClosedDatas {
        allocation_closed_datas: res,
    })
}

#[substreams::handlers::map]
pub fn map_staked_tokens_changes(
    clock: Clock,
    allocation_closed: AllocationClosedDatas,
    stake_store: StoreGetBigInt,
    query_fees_store: StoreGetBigInt,
    indexing_rewards_store: StoreGetBigInt,
) -> Result<StakedTokensChanges, Error> {
    Ok(StakedTokensChanges {
        staked_tokens_changes: allocation_closed
            .allocation_closed_datas
            .iter()
            .filter_map(|allocation_closed| {
                let st = GetIndexerStakedTokens {
                    indexer: Hex::decode(&allocation_closed.address).unwrap(),
                };
                if let Some(x) = st.call(GRAPH.to_vec()) {
                    Some(StakedTokensChange {
                        indexer: allocation_closed.address.clone().to_lowercase(),
                        block_number: clock.number,
                        transaction_hash: allocation_closed.transaction_hash.clone(),
                        tokens: x.to_string(),
                        stake: stake_store
                            .get_last(&allocation_closed.address.to_lowercase())
                            .unwrap_or(BigInt::zero())
                            .to_string(),
                        query_fees: query_fees_store
                            .get_last(&allocation_closed.address.to_lowercase())
                            .unwrap_or(BigInt::zero())
                            .to_string(),
                        indexing_rewards: indexing_rewards_store
                            .get_last(&allocation_closed.address.to_lowercase())
                            .unwrap_or(BigInt::zero())
                            .to_string(),
                    })
                } else {
                    None
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    staked_tokens_changes: StakedTokensChanges,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for staked_tokens_change in staked_tokens_changes.staked_tokens_changes {
        tables
            .create_row(
                "staked_tokens_changes",
                [
                    ("indexer", (&staked_tokens_change).indexer.to_string()),
                    ("block_number", block_num.to_string()),
                    (
                        "transaction_hash",
                        (&staked_tokens_change).transaction_hash.to_string(),
                    ),
                ],
            )
            .set("indexer", staked_tokens_change.indexer)
            .set("tokens", staked_tokens_change.tokens)
            .set("staked_tokens", staked_tokens_change.stake)
            .set("indexing_rewards", staked_tokens_change.indexing_rewards)
            .set("query_fees", staked_tokens_change.query_fees)
            .set("block_timestamp", &timestamp);
    }

    Ok(tables.to_database_changes())
}
