// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedStakeTokenChange {
    #[prost(string, tag="1")]
    pub indexer: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="2")]
    pub tokens_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedStakeTokenChanges {
    #[prost(message, repeated, tag="1")]
    pub owned_stake_token_changes: ::prost::alloc::vec::Vec<OwnedStakeTokenChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationClosedData {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub deployment_id: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="3")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub allocation_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub poi: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub transaction_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationClosedDatas {
    #[prost(message, repeated, tag="1")]
    pub allocation_closed_datas: ::prost::alloc::vec::Vec<AllocationClosedData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationClosedResult {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub allocation_id: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="3")]
    pub tokens: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="5")]
    pub stake: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakedTokensChange {
    #[prost(string, tag="1")]
    pub indexer: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub transaction_hash: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="4")]
    pub tokens: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="5")]
    pub stake: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="6")]
    pub query_fees: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="7")]
    pub indexing_rewards: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakedTokensChanges {
    #[prost(message, repeated, tag="1")]
    pub staked_tokens_changes: ::prost::alloc::vec::Vec<StakedTokensChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeesCollected {
    #[prost(string, tag="1")]
    pub indexer: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub deployment_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub allocation_id: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="4")]
    pub query_fees: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeesCollecteds {
    #[prost(message, repeated, tag="1")]
    pub query_fees_collecteds: ::prost::alloc::vec::Vec<QueryFeesCollected>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexingRewardsCollected {
    #[prost(string, tag="1")]
    pub indexer: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub allocation_id: ::prost::alloc::string::String,
    /// big.Int
    #[prost(string, tag="3")]
    pub rewards: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexingRewardsCollecteds {
    #[prost(message, repeated, tag="1")]
    pub indexing_rewards_collecteds: ::prost::alloc::vec::Vec<IndexingRewardsCollected>,
}
// @@protoc_insertion_point(module)
