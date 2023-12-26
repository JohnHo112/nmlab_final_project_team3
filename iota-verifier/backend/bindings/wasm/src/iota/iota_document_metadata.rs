// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::IotaDocumentMetadata;
use wasm_bindgen::prelude::*;

use crate::common::MapStringAny;
use crate::common::WasmTimestamp;
use crate::error::Result;

/// Additional attributes related to an IOTA DID Document.
#[wasm_bindgen(js_name = IotaDocumentMetadata, inspectable)]
pub struct WasmIotaDocumentMetadata(pub(crate) IotaDocumentMetadata);

// NOTE: these properties are read-only (no setters) to prevent bugs where a clone of the metadata
//       is updated instead of the actual instance in the document.
#[wasm_bindgen(js_class = IotaDocumentMetadata)]
impl WasmIotaDocumentMetadata {
  /// Returns a copy of the timestamp of when the DID document was created.
  #[wasm_bindgen]
  pub fn created(&self) -> Option<WasmTimestamp> {
    self.0.created.map(WasmTimestamp::from)
  }

  /// Returns a copy of the timestamp of the last DID document update.
  #[wasm_bindgen]
  pub fn updated(&self) -> Option<WasmTimestamp> {
    self.0.updated.map(WasmTimestamp::from)
  }

  /// Returns a copy of the deactivated status of the DID document.
  #[wasm_bindgen]
  pub fn deactivated(&self) -> Option<bool> {
    self.0.deactivated
  }

  /// Returns a copy of the Bech32-encoded state controller address, if present.
  #[wasm_bindgen(js_name = stateControllerAddress)]
  pub fn state_controller_address(&self) -> Option<String> {
    self.0.state_controller_address.clone()
  }

  /// Returns a copy of the Bech32-encoded governor address, if present.
  #[wasm_bindgen(js_name = governorAddress)]
  pub fn governor_address(&self) -> Option<String> {
    self.0.governor_address.clone()
  }

  /// Returns a copy of the custom metadata properties.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.properties())
  }
}

impl_wasm_json!(WasmIotaDocumentMetadata, IotaDocumentMetadata);
impl_wasm_clone!(WasmIotaDocumentMetadata, IotaDocumentMetadata);

impl From<IotaDocumentMetadata> for WasmIotaDocumentMetadata {
  fn from(metadata: IotaDocumentMetadata) -> Self {
    Self(metadata)
  }
}
