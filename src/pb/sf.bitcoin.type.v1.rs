// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// the block hash
    /// Bitcoin core reverses the byte order of the hash when printing it out has Hex, to prevent
    /// the end user from making a mistake we store it as a string directly
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    // NON-DETERMINISTIC: The number of confirmations, or -1 if the block is not on the main chain
    // int64 confirmations = 2;
    // it will be read and reflected indirectly inside the pbbstream.block's LIB

    /// The block size
    #[prost(int32, tag="3")]
    pub size: i32,
    /// The block size excluding witness data
    #[prost(int32, tag="4")]
    pub stripped_size: i32,
    /// The block weight as defined in BIP 141
    #[prost(int32, tag="5")]
    pub weight: i32,
    /// The block height or index
    #[prost(int64, tag="6")]
    pub height: i64,
    /// The block version
    #[prost(int32, tag="7")]
    pub version: i32,
    /// The block version formatted in hexadecimal
    #[prost(string, tag="8")]
    pub version_hex: ::prost::alloc::string::String,
    /// The merkle root
    #[prost(string, tag="9")]
    pub merkle_root: ::prost::alloc::string::String,
    /// Transaction array
    #[prost(message, repeated, tag="10")]
    pub tx: ::prost::alloc::vec::Vec<Transaction>,
    /// The block time expressed in UNIX epoch time
    #[prost(int64, tag="11")]
    pub time: i64,
    /// The median block time expressed in UNIX epoch time
    #[prost(int64, tag="12")]
    pub mediantime: i64,
    /// The nonce
    #[prost(uint32, tag="13")]
    pub nonce: u32,
    /// The bits
    #[prost(string, tag="14")]
    pub bits: ::prost::alloc::string::String,
    /// The difficulty
    #[prost(double, tag="15")]
    pub difficulty: f64,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    #[prost(string, tag="16")]
    pub chainwork: ::prost::alloc::string::String,
    /// The number of transactions in the block
    #[prost(uint32, tag="17")]
    pub n_tx: u32,
    /// The hash of the previous block
    #[prost(string, tag="18")]
    pub previous_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// The serialized, hex-encoded data for 'txid'
    #[prost(string, tag="1")]
    pub hex: ::prost::alloc::string::String,
    /// The transaction id
    #[prost(string, tag="2")]
    pub txid: ::prost::alloc::string::String,
    /// The transaction hash (differs from txid for witness transactions)
    #[prost(string, tag="3")]
    pub hash: ::prost::alloc::string::String,
    /// The serialized transaction size
    #[prost(int32, tag="4")]
    pub size: i32,
    /// The virtual transaction size (differs from size for witness transactions)
    #[prost(int32, tag="5")]
    pub vsize: i32,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    #[prost(int32, tag="6")]
    pub weight: i32,
    ///   The version
    #[prost(uint32, tag="7")]
    pub version: u32,
    /// The lock time
    #[prost(uint32, tag="8")]
    pub locktime: u32,
    #[prost(message, repeated, tag="9")]
    pub vin: ::prost::alloc::vec::Vec<Vin>,
    #[prost(message, repeated, tag="10")]
    pub vout: ::prost::alloc::vec::Vec<Vout>,
    /// the block hash
    #[prost(string, tag="11")]
    pub blockhash: ::prost::alloc::string::String,
    /// The block time expressed in UNIX epoch time
    #[prost(int64, tag="12")]
    pub blocktime: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vin {
    /// The transaction id
    #[prost(string, tag="1")]
    pub txid: ::prost::alloc::string::String,
    /// The output number
    #[prost(uint32, tag="2")]
    pub vout: u32,
    /// The script
    #[prost(message, optional, tag="3")]
    pub script_sig: ::core::option::Option<ScriptSig>,
    /// The script sequence number
    #[prost(uint32, tag="4")]
    pub sequence: u32,
    /// hex-encoded witness data (if any)
    #[prost(string, repeated, tag="5")]
    pub txinwitness: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// hex-encoded coinbase
    #[prost(string, tag="6")]
    pub coinbase: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vout {
    /// The value in BTC
    #[prost(double, tag="1")]
    pub value: f64,
    /// index
    #[prost(uint32, tag="2")]
    pub n: u32,
    #[prost(message, optional, tag="3")]
    pub script_pub_key: ::core::option::Option<ScriptPubKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptSig {
    /// The asm
    #[prost(string, tag="1")]
    pub asm: ::prost::alloc::string::String,
    /// The hex
    #[prost(string, tag="2")]
    pub hex: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptPubKey {
    /// the asm
    #[prost(string, tag="1")]
    pub asm: ::prost::alloc::string::String,
    /// the hex
    #[prost(string, tag="2")]
    pub hex: ::prost::alloc::string::String,
    /// The required sigs
    #[prost(int32, tag="3")]
    pub req_sigs: i32,
    /// The type, eg 'pubkeyhash'
    #[prost(string, tag="4")]
    pub r#type: ::prost::alloc::string::String,
    /// bitcoin address
    #[prost(string, tag="5")]
    pub address: ::prost::alloc::string::String,
    /// bitcoin addresses (deprecated, empty when 'address' is set)
    #[prost(string, repeated, tag="6")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)
