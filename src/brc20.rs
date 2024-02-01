use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde_json::Value;
use substreams::scalar::BigInt;

// #[derive(Debug)]
// struct _BigInt(BigInt);

// impl<'de> Deserialize<'de> for _BigInt {
//     fn deserialize<D>(deserializer: D) -> Result<_BigInt, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         BigInt::from_str(&s)
//             .map(_BigInt)
//             .map_err(serde::de::Error::custom)
//     }
// }

fn deserialize_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => BigInt::from_str(&s).map_err(serde::de::Error::custom),
        Value::Number(n) => Ok(BigInt::from(n.as_i64().ok_or(serde::de::Error::custom("Invalid number"))?)),
        _ => Err(serde::de::Error::custom("Invalid type")),
    }
}

fn deserialize_bigint_option<'de, D>(deserializer: D) -> Result<Option<BigInt>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => BigInt::from_str(&s).map(Some).map_err(serde::de::Error::custom),
        Value::Number(n) => Ok(Some(BigInt::from(n.as_i64().ok_or(serde::de::Error::custom("Invalid number"))?))),
        Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom("Invalid type")),
    }
}

#[derive(Debug, Deserialize)]
pub struct Deploy {
    pub tick: String,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub max: BigInt,
    #[serde(deserialize_with = "deserialize_bigint_option")]
    pub lim: Option<BigInt>,
    pub dec: Option<i32>,
}

impl Deploy {
    // pub fn max(&self) -> BigInt {
    //     self.max
    // }

    // pub fn lim(&self) -> Option<BigInt> {
    //     self.lim
    // }

    pub fn dec(&self) -> i32 {
        self.dec.unwrap_or(18)
    }
}

#[derive(Debug, Deserialize)]
pub struct Mint {
    pub tick: String,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub amt: BigInt,
}

// impl Mint {
//     pub fn amt(&self) -> BigInt {
//         self.amt
//     }
// }

#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub tick: String,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub amt: BigInt,
}

// impl Transfer {
//     pub fn amt(&self) -> BigInt {
//         self.amt
//     }
// }

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum Brc20Event {
    Deploy(Deploy),
    Mint(Mint),
    Transfer(Transfer),
}
