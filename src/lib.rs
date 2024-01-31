mod ord;
mod pb;
mod address;
mod sats_utils;
mod brc20;

use anyhow::Result;
use address::address_from_scriptpubkey;
use bitcoin::{consensus::deserialize, hashes::hex::FromHex, Transaction};
use brc20::Brc20Event;
use ord::envelope::ParsedEnvelope;
use pb::ordinals::v1::{self as ord_proto, Inscription};
use pb::btc::brc20::v1::{Brc20Events, Deploy, InscribedTransferLocation, Mint, ExecutedTransfer, InscribedTransfer};
use pb::sf::bitcoin::r#type::v1 as btc;
use sats_utils::btc_to_sats;
use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto};

struct Location {
    pub utxo: String,
    pub offset: u64,
    pub utxo_amount: u64,
}

#[substreams::handlers::map]
fn map_brc20_events(
    block: btc::Block,
) -> Result<Brc20Events, substreams::errors::Error> {
    let events = block.tx.into_iter()
        // Filter if tx data contains "text/plain;charset=utf-8" inscriptions
        .filter(|tx| tx.hex.contains("746578742f706c61696e3b636861727365743d7574662d38"))
        .flat_map(|tx| {
            let txid = tx.txid.clone();
            match parse_inscriptions(&tx) {
                Ok(inscriptions) => {
                    inscriptions.into_iter()
                        .map(|inscription| {
                            let (vout, offset) = tx.nth_sat_utxo(inscription.pointer.unwrap_or(0) as u64).unwrap();
                            (Location {
                                utxo: format!("{}:{}", tx.txid, offset),
                                offset,
                                utxo_amount: btc_to_sats(vout.value),
                            }, vout.address(), inscription)
                        })
                        .collect()
                },
                Err(err) => {
                    substreams::log::info!("Error parsing inscriptions in tx {}: {}", txid, err);
                    vec![]
                }
            }
        })
        .filter_map(|(location, address, inscription)| {
            match serde_json::from_str::<Brc20Event>(&inscription.content) {
                Ok(event) => Some((location, address, event)),
                Err(err) => {
                    substreams::log::info!("Error parsing inscription content {}: {}", inscription.id, err);
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(Brc20Events {
        // block_height: block.height as u64,
        // timestamp: block.time as u64,
        block_height: 0,
        timestamp: 0,
        deploys: events.iter()
            .filter_map(|(_, address, event)| match (address, event) {
                (_, Brc20Event::Deploy(deploy)) => Some(Deploy {
                    id: "".into(),
                    symbol: deploy.tick.clone(),
                    max_supply: deploy.max().to_string(),
                    mint_limit: deploy.lim().map(|lim| lim.to_string()),
                    decimals: deploy.dec().to_string(),
                }),
                _ => None,
            })
            .collect(),
        mints: events.iter()
            .filter_map(|(_, address, event)| match (address, event) {
                (Some(address), Brc20Event::Mint(mint)) => Some(Mint {
                    id: "".into(),
                    token: mint.tick.clone(),
                    to: address.into(),
                    amount: mint.amt.to_string(),
                }),
                _ => None,
            })
            .collect(),
        inscribed_transfers: events.iter()
            .filter_map(|(location, address, event)| match (address, event) {
                (Some(address), Brc20Event::Transfer(transfer)) => Some(InscribedTransfer {
                    id: "".into(),
                    token: transfer.tick.clone(),
                    // to: "".into(),
                    from: address.into(),
                    amount: transfer.amt.to_string(),
                    utxo: location.utxo.clone(),
                    offset: location.offset,
                    utxo_amount: location.utxo_amount,
                }),
                _ => None,
            })
            .collect(),
        executed_transfers: vec![]
    })
}

#[substreams::handlers::store]
fn store_inscribed_transfers(events: Brc20Events, store: StoreSetProto<InscribedTransferLocation>) {
    events.inscribed_transfers.iter()
        .for_each(|transfer| {
            store.set(
                0,
                transfer.utxo.clone(),
                &InscribedTransferLocation {
                    id: transfer.id.clone(),
                    offset: transfer.offset.clone(),
                    from: transfer.from.clone(),
                    amount: transfer.amount.clone(),
                    utxo_amount: transfer.utxo_amount.clone(),
                }
            );
        });
} 

#[substreams::handlers::map]
fn map_resolve_transfers(
    block: btc::Block,
    events: Brc20Events,
    store: StoreGetProto<InscribedTransferLocation>,
) -> Result<Brc20Events, substreams::errors::Error> {
    let executed_transfers = block.tx.into_iter()
        .filter_map(|tx| {
            // Note: Without tracking UTXO values, we can only reliably resolve transfers where the
            // inscribed sat is held by the first input UTXO of the transaction
            if let Some(inscribed_transfer_loc) = store.get_at(0, format!("{}:{}", tx.vin[0].txid, tx.vin[0].vout)) {
                let (vout, _) = tx.nth_sat_utxo(inscribed_transfer_loc.offset).unwrap();
                Some(ExecutedTransfer {
                    id: inscribed_transfer_loc.id,
                    from: inscribed_transfer_loc.from,
                    to: vout.address().unwrap(),
                    amount: inscribed_transfer_loc.amount,
                })
            } else {
                // Log that we could not resolve transfer
                if let Some(inscribed_transfer_loc) = tx.vin.iter().find_map(|vin| store.get_at(0, format!("{}:{}", vin.txid, vin.vout))) {
                    substreams::log::info!("Could not resolve inscribed transfer {}", inscribed_transfer_loc.id);
                }
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(Brc20Events {executed_transfers, ..events})
}

#[substreams::handlers::map]
fn map_inscriptions(block: btc::Block) -> Result<ord_proto::Inscriptions, substreams::errors::Error> {
    let inscriptions = block.tx.into_iter()
        .filter(|tx| tx.hex.contains("0063"))
        .flat_map(|tx| {
            let txid = tx.txid.clone();
            match parse_inscriptions(&tx) {
                Ok(inscriptions) => inscriptions,
                Err(err) => {
                    substreams::log::info!("Error parsing inscriptions in tx {}: {}", txid, err);
                    vec![]
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(ord_proto::Inscriptions { inscriptions })
}

impl btc::Transaction {
    /// Returns the nth satoshi UTXO in the transaction along with its offset within the UTXO.
    pub fn nth_sat_utxo(&self, offset: u64) -> Option<(btc::Vout, u64)> {
        let mut sat = 0;
        for (idx, output) in self.vout.iter().enumerate() {
            let utxo_sats = btc_to_sats(output.value);
            if sat + utxo_sats > offset {
                return Some((self.vout[idx].clone(), offset - sat));
            }
            sat += utxo_sats;
        }
        None
    }
}

impl btc::Vout {
    pub fn address(&self) -> Option<String> {
        self.script_pub_key
            .as_ref()
            .and_then(|script_pub_key| address_from_scriptpubkey(&script_pub_key.hex))
    }
}

fn parse_inscriptions(tx: &btc::Transaction) -> Result<Vec<Inscription>> {
    let raw_trx = Vec::from_hex(&tx.hex).unwrap();
    let tx_: Transaction = deserialize(&raw_trx).unwrap();
    let envelopes = ParsedEnvelope::from_transaction(&tx_);
    let inscriptions = envelopes.into_iter()
        .enumerate()
        .filter_map(move |(idx, envelope)| {
            Some(Inscription {
                id: format!("{}i{}", tx.txid, idx),
                content_type: envelope.payload.content_type().map(|s| s.to_string()),
                content_length: envelope.payload.content_length().map(|s| s.to_string()).unwrap_or("0".into()),
                pointer: envelope.payload.pointer().map(|ptr| ptr as i64),
                parent: envelope.payload.parent().map(|parent| parent.to_string()),
                metadata: envelope.payload.metadata.clone().map(|metadata| match String::from_utf8(metadata.clone()) {
                    Ok(metadata) => metadata,   
                    Err(_) => hex::encode(metadata)
                }),
                metaprotocol: envelope.payload.metaprotocol().map(|s| s.to_string()),
                content_encoding: envelope.payload.content_encoding().map(|s| match String::from_utf8(s.as_ref().to_vec()) {
                    Ok(content_type) => content_type,
                    Err(_) => hex::encode(s.as_ref())
                }),
                content: match String::from_utf8(envelope.payload.body().unwrap_or_default().to_vec()) {
                    Ok(content) => content,
                    Err(_) => hex::encode(envelope.payload.body().unwrap_or_default())
                }
            })
        })
        .collect();

    Ok(inscriptions)
}
