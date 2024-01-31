use bitcoin::{
    blockdata::script::Script,
    network::Network,
    address::Address,
    // hash160::Hash
};

pub fn address_from_scriptpubkey(script_pub_key_hex: &str) -> Option<String> {
    // Decode the script from hex
    let hex_data = hex::decode(script_pub_key_hex).expect("Valid hex script");
    let script = Script::from_bytes(&hex_data);

    // Create a Bitcoin address from the public key script
    Address::from_script(script, Network::Bitcoin)
        .map(|address| address.to_string())
        .ok()
}

#[cfg(test)]
mod tests {
    use super::address_from_scriptpubkey;

    #[test]
    fn test_address_from_scriptpubkey() {
        assert_eq!(
            address_from_scriptpubkey("76a914534e48e9a49ce7ebf8d84c8313e4edfa48852fa188ac"),
            Some("18bUsFHLgFotUqAL9ftLBVenJDVP7M64Nu".into())
        )
    }
}