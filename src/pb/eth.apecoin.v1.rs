// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Account>,
    #[prost(message, optional, tag="2")]
    pub to: ::core::option::Option<Account>,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub block_number: u64,
    #[prost(uint64, tag="12")]
    pub timestamp: u64,
    #[prost(uint32, tag="13")]
    pub log_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Approval {
    #[prost(string, tag="1")]
    pub spender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub owner: ::core::option::Option<Account>,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub block_number: u64,
    #[prost(uint64, tag="12")]
    pub timestamp: u64,
    #[prost(uint32, tag="13")]
    pub log_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Approvals {
    #[prost(message, repeated, tag="1")]
    pub approvals: ::prost::alloc::vec::Vec<Approval>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub decimal: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
