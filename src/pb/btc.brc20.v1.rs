// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deploy {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub max_supply: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub mint_limit: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub decimals: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mint {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
}
/// Used to keep track of pending transfers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InscribedTransfer {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub utxo: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub offset: u64,
    #[prost(uint64, tag="7")]
    pub utxo_amount: u64,
}
/// Used to keep track of pending transfers location in a UTXO
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InscribedTransferLocation {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub offset: u64,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub utxo_amount: u64,
}
/// Represents executed transfer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedTransfer {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brc20Events {
    #[prost(uint64, tag="1")]
    pub block_height: u64,
    #[prost(uint64, tag="2")]
    pub timestamp: u64,
    #[prost(message, repeated, tag="3")]
    pub deploys: ::prost::alloc::vec::Vec<Deploy>,
    #[prost(message, repeated, tag="4")]
    pub mints: ::prost::alloc::vec::Vec<Mint>,
    #[prost(message, repeated, tag="5")]
    pub inscribed_transfers: ::prost::alloc::vec::Vec<InscribedTransfer>,
    #[prost(message, repeated, tag="6")]
    pub executed_transfers: ::prost::alloc::vec::Vec<ExecutedTransfer>,
}
// @@protoc_insertion_point(module)
