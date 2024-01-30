// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakedTokensChange {
    #[prost(string, tag="1")]
    pub indexer: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    /// big.Int
    #[prost(string, tag="3")]
    pub tokens: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakedTokensChanges {
    #[prost(message, repeated, tag="1")]
    pub staked_tokens_changes: ::prost::alloc::vec::Vec<StakedTokensChange>,
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationClosedDatas {
    #[prost(message, repeated, tag="1")]
    pub allocation_closed_datas: ::prost::alloc::vec::Vec<AllocationClosedData>,
}
// @@protoc_insertion_point(module)
