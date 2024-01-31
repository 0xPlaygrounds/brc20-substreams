use ethabi::ethereum_types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Deploy {
    pub tick: String,
    pub max: U256,
    pub lim: Option<U256>,
    pub dec: Option<u8>,
}

impl Deploy {
    pub fn max(&self) -> U256 {
        // self.max.parse().unwrap()
        self.max
    }

    pub fn lim(&self) -> Option<U256> {
        // self.lim.as_ref().map(|lim| lim.parse().unwrap())
        self.lim
    }

    pub fn dec(&self) -> u8 {
        // self.dec.as_ref().unwrap_or(&"18".into()).parse().unwrap()
        self.dec.unwrap_or(18)
    }
}

#[derive(Debug, Deserialize)]
pub struct Mint {
    pub tick: String,
    pub amt: U256,
}

impl Mint {
    pub fn amt(&self) -> U256 {
        // self.amt.parse().unwrap()
        self.amt
    }
}

#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub tick: String,
    pub amt: U256,
}

impl Transfer {
    pub fn amt(&self) -> U256 {
        // self.amt.parse().unwrap()
        self.amt
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum Brc20Event {
    Deploy(Deploy),
    Mint(Mint),
    Transfer(Transfer),
}