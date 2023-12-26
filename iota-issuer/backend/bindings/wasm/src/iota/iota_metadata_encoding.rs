// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::StateMetadataEncoding;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = StateMetadataEncoding)]
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum WasmStateMetadataEncoding {
  Json = 0,
}

impl From<WasmStateMetadataEncoding> for StateMetadataEncoding {
  fn from(encoding: WasmStateMetadataEncoding) -> Self {
    match encoding {
      WasmStateMetadataEncoding::Json => Self::Json,
    }
  }
}
