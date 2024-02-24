use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;
use substreams_ethereum::pb::eth::v2::Block;
use substreams::errors::Error;
use substreams::{Hex};
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{StoreNew, StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt};
use crate::abi::graph::functions::GetIndexerStakedTokens;
use crate::pb::allocations::types::v1::{AllocationClosedData, AllocationClosedDatas, OwnedStakeTokenChange, OwnedStakeTokenChanges, StakedTokensChange, StakedTokensChanges};
use substreams_database_change::pb::database::DatabaseChanges;
use crate::abi::graph::events::{AllocationClosed1, AllocationClosed2, StakeDeposited, StakeWithdrawn};
use crate::abi::rewards::events::RewardsAssigned;
use crate::abi::transfer::events::Transfer;
use crate::utils::{GRAPH};

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
        owned_stake_token_changes
    })
}
#[substreams::handlers::map]
pub fn map_stake_withdrawn(block: Block) -> Result<OwnedStakeTokenChanges, Error> {
    Ok(OwnedStakeTokenChanges {
        owned_stake_token_changes: block.events::<StakeWithdrawn>(&[&GRAPH]).filter_map(|(event, _)| {
            let stake_deposited = OwnedStakeTokenChange {
                indexer: Hex::encode(&event.indexer),
                tokens_amount: format!("-{}", event.tokens.to_string()),
            };
            Some(stake_deposited)
        }).collect()
    })
}

#[substreams::handlers::store]
pub fn store_stake_token_changes(indexers: String, stake_deposited: OwnedStakeTokenChanges, stake_withdrawn: OwnedStakeTokenChanges, store: StoreAddBigInt) {
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

    let mut map : HashMap<String, BigInt> = std::collections::HashMap::new();

    stake_deposited.owned_stake_token_changes.iter().for_each(|x| {
        if !all && !indexers_map.contains(&x.indexer) {
            return;
        }

        if let Some(val) = map.get_mut(&x.indexer) {
            let a = val.clone();
            let b = BigInt::from_str(&x.tokens_amount).unwrap();
            let new_value = a.add(b);
            map.insert(x.indexer.clone(), new_value.clone());
        } else {
            map.insert(x.indexer.clone(), BigInt::from_str(&x.tokens_amount).unwrap_or(BigInt::zero()));
        }
    });

    stake_withdrawn.owned_stake_token_changes.iter().for_each(|x| {
        if !all && !indexers_map.contains(&x.indexer) {
            return;
        }

        if let Some(val) = map.get_mut(&x.indexer) {
            let a = val.clone();
            let b = BigInt::from_str(&x.tokens_amount).unwrap();
            let new_value = a.add(b);
            map.insert(x.indexer.clone(), new_value.clone());
        } else {
            map.insert(x.indexer.clone(), BigInt::from_str(&x.tokens_amount).unwrap_or(BigInt::zero()));
        }
    });

    map.iter().for_each(|(k, v)| {
        store.add(0, &k, &v);
    });
}

#[substreams::handlers::map]
pub fn map_allocation_closed1(block: Block) -> Result<AllocationClosedDatas, Error> {
    Ok(AllocationClosedDatas {
        allocation_closed_datas: block.events::<AllocationClosed1>(&[&GRAPH]).filter_map(|(event, log)| {
            let allocation_closed = AllocationClosedData {
                address: Hex::encode(&event.indexer),
                deployment_id: Hex::encode(&event.subgraph_deployment_id),
                tokens: event.tokens.to_string(),
                allocation_id: Hex::encode(&event.allocation_id),
                poi: Hex::encode(&event.poi),
                transaction_hash: Hex::encode(&log.receipt.transaction.hash),
            };
            Some(allocation_closed)
        }).collect()
    })
}
#[substreams::handlers::map]
pub fn map_allocation_closed2(block: Block) -> Result<AllocationClosedDatas, Error> {
    Ok(AllocationClosedDatas {
        allocation_closed_datas: block.events::<AllocationClosed2>(&[&GRAPH]).filter_map(|(event, log)| {
            let allocation_closed = AllocationClosedData {
                address: Hex::encode(&event.indexer),
                deployment_id: Hex::encode(&event.subgraph_deployment_id),
                tokens: event.tokens.to_string(),
                allocation_id: Hex::encode(&event.allocation_id),
                poi: Hex::encode(&event.poi),
                transaction_hash: Hex::encode(&log.receipt.transaction.hash),
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
pub fn map_staked_tokens_changes(clock: Clock, allocation_closed: AllocationClosedDatas, stake_store: StoreGetBigInt) -> Result<StakedTokensChanges, Error> {
    Ok(StakedTokensChanges {
        staked_tokens_changes: allocation_closed.allocation_closed_datas.iter().filter_map(|allocation_closed| {
            let st = GetIndexerStakedTokens {
                indexer: Hex::decode(&allocation_closed.address).unwrap()
            };
            if let Some(x) = st.call(GRAPH.to_vec()) {
                Some(StakedTokensChange {
                    indexer: allocation_closed.address.clone(),
                    block_number: clock.number,
                    transaction_hash: allocation_closed.transaction_hash.clone(),
                    tokens: x.to_string(),
                    stake: stake_store.get_last(&allocation_closed.address).unwrap_or(BigInt::zero()).to_string(),
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
        tables.create_row("staked_tokens_changes", [("indexer", (&staked_tokens_change).indexer.to_string()), ("block_number", block_num.to_string()), ("transaction_hash", (&staked_tokens_change).transaction_hash.to_string())])
            .set("indexer", staked_tokens_change.indexer)
            .set("tokens", staked_tokens_change.tokens)
            .set("staked_tokens", staked_tokens_change.stake)
            .set("block_timestamp", &timestamp);
    }

    Ok(tables.to_database_changes())
}