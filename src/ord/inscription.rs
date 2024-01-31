use bitcoin::{Txid, hashes::Hash};

use super::inscription_id::InscriptionId;

use {
  super::*,
  bitcoin::{
    blockdata::{
      opcodes,
      script::{self, PushBytesBuf},
    },
    ScriptBuf,
  },
  std::str,
  http::header::HeaderValue,
};

#[derive(Debug, PartialEq, Clone, Eq, Default)]
pub struct Inscription {
  pub body: Option<Vec<u8>>,
  pub content_encoding: Option<Vec<u8>>,
  pub content_type: Option<Vec<u8>>,
  pub duplicate_field: bool,
  pub incomplete_field: bool,
  pub metadata: Option<Vec<u8>>,
  pub metaprotocol: Option<Vec<u8>>,
  pub parent: Option<Vec<u8>>,
  pub pointer: Option<Vec<u8>>,
  pub unrecognized_even_field: bool,
}

impl Inscription {
  #[cfg(test)]
  pub(crate) fn new(content_type: Option<Vec<u8>>, body: Option<Vec<u8>>) -> Self {
    Self {
      content_type,
      body,
      ..Default::default()
    }
  }

  pub(crate) fn pointer_value(pointer: u64) -> Vec<u8> {
    let mut bytes = pointer.to_le_bytes().to_vec();

    while bytes.last().copied() == Some(0) {
      bytes.pop();
    }

    bytes
  }

  pub(crate) fn append_reveal_script_to_builder(
    &self,
    mut builder: script::Builder,
  ) -> script::Builder {
    builder = builder
      .push_opcode(opcodes::OP_FALSE)
      .push_opcode(opcodes::all::OP_IF)
      .push_slice(envelope::PROTOCOL_ID);

    if let Some(content_type) = self.content_type.clone() {
      builder = builder
        .push_slice(envelope::CONTENT_TYPE_TAG)
        .push_slice(PushBytesBuf::try_from(content_type).unwrap());
    }

    if let Some(content_encoding) = self.content_encoding.clone() {
      builder = builder
        .push_slice(envelope::CONTENT_ENCODING_TAG)
        .push_slice(PushBytesBuf::try_from(content_encoding).unwrap());
    }

    if let Some(protocol) = self.metaprotocol.clone() {
      builder = builder
        .push_slice(envelope::METAPROTOCOL_TAG)
        .push_slice(PushBytesBuf::try_from(protocol).unwrap());
    }

    if let Some(parent) = self.parent.clone() {
      builder = builder
        .push_slice(envelope::PARENT_TAG)
        .push_slice(PushBytesBuf::try_from(parent).unwrap());
    }

    if let Some(pointer) = self.pointer.clone() {
      builder = builder
        .push_slice(envelope::POINTER_TAG)
        .push_slice(PushBytesBuf::try_from(pointer).unwrap());
    }

    if let Some(metadata) = &self.metadata {
      for chunk in metadata.chunks(520) {
        builder = builder.push_slice(envelope::METADATA_TAG);
        builder = builder.push_slice(PushBytesBuf::try_from(chunk.to_vec()).unwrap());
      }
    }

    if let Some(body) = &self.body {
      builder = builder.push_slice(envelope::BODY_TAG);
      for chunk in body.chunks(520) {
        builder = builder.push_slice(PushBytesBuf::try_from(chunk.to_vec()).unwrap());
      }
    }

    builder.push_opcode(opcodes::all::OP_ENDIF)
  }

  #[cfg(test)]
  pub(crate) fn append_reveal_script(&self, builder: script::Builder) -> ScriptBuf {
    self.append_reveal_script_to_builder(builder).into_script()
  }

  pub(crate) fn append_batch_reveal_script_to_builder(
    inscriptions: &[Inscription],
    mut builder: script::Builder,
  ) -> script::Builder {
    for inscription in inscriptions {
      builder = inscription.append_reveal_script_to_builder(builder);
    }

    builder
  }

  pub(crate) fn append_batch_reveal_script(
    inscriptions: &[Inscription],
    builder: script::Builder,
  ) -> ScriptBuf {
    Inscription::append_batch_reveal_script_to_builder(inscriptions, builder).into_script()
  }


  pub(crate) fn body(&self) -> Option<&[u8]> {
    Some(self.body.as_ref()?)
  }

  pub(crate) fn into_body(self) -> Option<Vec<u8>> {
    self.body
  }

  pub(crate) fn content_length(&self) -> Option<usize> {
    Some(self.body()?.len())
  }

  pub(crate) fn content_type(&self) -> Option<&str> {
    str::from_utf8(self.content_type.as_ref()?).ok()
  }

  pub(crate) fn content_encoding(&self) -> Option<HeaderValue> {
    HeaderValue::from_str(str::from_utf8(self.content_encoding.as_ref()?).unwrap_or_default()).ok()
  }

  pub(crate) fn metaprotocol(&self) -> Option<&str> {
    str::from_utf8(self.metaprotocol.as_ref()?).ok()
  }

  pub(crate) fn parent(&self) -> Option<InscriptionId> {
    let value = self.parent.as_ref()?;

    if value.len() < Txid::LEN {
      return None;
    }

    if value.len() > Txid::LEN + 4 {
      return None;
    }

    let (txid, index) = value.split_at(Txid::LEN);

    if let Some(last) = index.last() {
      // Accept fixed length encoding with 4 bytes (with potential trailing zeroes)
      // or variable length (no trailing zeroes)
      if index.len() != 4 && *last == 0 {
        return None;
      }
    }

    let txid = Txid::from_slice(txid).unwrap();

    let index = [
      index.first().copied().unwrap_or(0),
      index.get(1).copied().unwrap_or(0),
      index.get(2).copied().unwrap_or(0),
      index.get(3).copied().unwrap_or(0),
    ];

    let index = u32::from_le_bytes(index);

    Some(InscriptionId { txid, index })
  }

  pub(crate) fn pointer(&self) -> Option<u64> {
    let value = self.pointer.as_ref()?;

    if value.iter().skip(8).copied().any(|byte| byte != 0) {
      return None;
    }

    let pointer = [
      value.first().copied().unwrap_or(0),
      value.get(1).copied().unwrap_or(0),
      value.get(2).copied().unwrap_or(0),
      value.get(3).copied().unwrap_or(0),
      value.get(4).copied().unwrap_or(0),
      value.get(5).copied().unwrap_or(0),
      value.get(6).copied().unwrap_or(0),
      value.get(7).copied().unwrap_or(0),
    ];

    Some(u64::from_le_bytes(pointer))
  }

}
