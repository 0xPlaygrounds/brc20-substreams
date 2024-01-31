// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Containing both create table statements and index creation statements.
    #[prost(string, tag="1")]
    pub schema: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub dbt_config: ::core::option::Option<DbtConfig>,
    #[prost(bool, tag="3")]
    pub wire_protocol_access: bool,
    #[prost(message, optional, tag="4")]
    pub hasura_frontend: ::core::option::Option<HasuraFrontend>,
    #[prost(message, optional, tag="5")]
    pub postgraphile_frontend: ::core::option::Option<PostgraphileFrontend>,
    #[prost(message, optional, tag="6")]
    pub pgweb_frontend: ::core::option::Option<PgWebFrontend>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbtConfig {
    #[prost(bytes="vec", tag="1")]
    pub files: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasuraFrontend {
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostgraphileFrontend {
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PgWebFrontend {
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
// @@protoc_insertion_point(module)
