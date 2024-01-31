use std::{str::FromStr, fmt::{Display, Formatter, self}};

use bitcoin::{Txid, hashes::Hash};


#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct InscriptionId {
  pub txid: Txid,
  pub index: u32,
}

impl Default for InscriptionId {
  fn default() -> Self {
    Self {
      txid: Txid::all_zeros(),
      index: 0,
    }
  }
}

impl InscriptionId {
  pub(crate) fn parent_value(self) -> Vec<u8> {
    let index = self.index.to_le_bytes();
    let mut index_slice = index.as_slice();

    while index_slice.last().copied() == Some(0) {
      index_slice = &index_slice[0..index_slice.len() - 1];
    }

    self
      .txid
      .to_byte_array()
      .iter()
      .chain(index_slice)
      .copied()
      .collect()
  }
}


impl Display for InscriptionId {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}i{}", self.txid, self.index)
  }
}

#[derive(Debug)]
pub enum ParseError {
  Character(char),
  Length(usize),
  Separator(char),
  Txid(bitcoin::hashes::hex::HexToArrayError),
  Index(std::num::ParseIntError),
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Character(c) => write!(f, "invalid character: '{c}'"),
      Self::Length(len) => write!(f, "invalid length: {len}"),
      Self::Separator(c) => write!(f, "invalid separator: `{c}`"),
      Self::Txid(err) => write!(f, "invalid txid: {err}"),
      Self::Index(err) => write!(f, "invalid index: {err}"),
    }
  }
}

impl std::error::Error for ParseError {}

impl FromStr for InscriptionId {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(char) = s.chars().find(|char| !char.is_ascii()) {
      return Err(ParseError::Character(char));
    }

    const TXID_LEN: usize = 64;
    const MIN_LEN: usize = TXID_LEN + 2;

    if s.len() < MIN_LEN {
      return Err(ParseError::Length(s.len()));
    }

    let txid = &s[..TXID_LEN];

    let separator = s.chars().nth(TXID_LEN).unwrap();

    if separator != 'i' {
      return Err(ParseError::Separator(separator));
    }

    let vout = &s[TXID_LEN + 1..];

    Ok(Self {
      txid: txid.parse().map_err(ParseError::Txid)?,
      index: vout.parse().map_err(ParseError::Index)?,
    })
  }
}
