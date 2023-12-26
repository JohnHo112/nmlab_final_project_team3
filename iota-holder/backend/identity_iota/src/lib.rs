// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![doc = include_str!("./../README.md")]
#![allow(clippy::upper_case_acronyms)]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_docs,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc,
  clippy::missing_errors_doc
)]

pub mod core {
  //! Core Traits and Types

  pub use identity_core::common::*;
  pub use identity_core::convert::*;
  pub use identity_core::error::*;

  #[doc(inline)]
  pub use identity_core::json;
}

pub mod credential {
  //! Verifiable Credentials
  //!
  //! [Specification](https://www.w3.org/TR/vc-data-model/)

  pub use identity_credential::credential::*;
  #[cfg(feature = "domain-linkage")]
  pub use identity_credential::domain_linkage::*;
  pub use identity_credential::error::*;
  pub use identity_credential::presentation::*;
  #[cfg(feature = "revocation-bitmap")]
  pub use identity_credential::revocation::*;
  pub use identity_credential::validator::*;
}

pub mod did {
  //! Decentralized Identifiers
  //!
  //! [Specification](https://www.w3.org/TR/did-core/)
  pub use identity_did::*;
}
pub mod document {
  //! DID Documents
  //!
  //! [Specification](https://www.w3.org/TR/did-core/)

  pub use identity_document::document::*;
  pub use identity_document::error::*;

  pub use identity_document::service::*;
  pub use identity_document::utils::*;

  pub use identity_document::verifiable;
}

pub mod iota {
  //! The IOTA DID method implementation for the IOTA ledger.

  pub use identity_iota_core::*;
}

pub mod prelude {
  //! Prelude of commonly used types

  pub use identity_iota_core::IotaDID;
  pub use identity_iota_core::IotaDocument;

  #[cfg(feature = "iota-client")]
  #[cfg_attr(docsrs, doc(cfg(feature = "iota-client")))]
  pub use identity_iota_core::IotaClientExt;
  #[cfg(feature = "client")]
  #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
  pub use identity_iota_core::IotaIdentityClient;
  #[cfg(feature = "client")]
  #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
  pub use identity_iota_core::IotaIdentityClientExt;

  #[cfg(feature = "resolver")]
  #[cfg_attr(docsrs, doc(cfg(feature = "resolver")))]
  pub use identity_resolver::Resolver;
}

#[cfg(feature = "resolver")]
#[cfg_attr(docsrs, doc(cfg(feature = "resolver")))]
pub mod resolver {
  //! DID resolution utilities

  pub use identity_resolver::*;
}

pub mod verification {
  //! Types for verifiable data
  pub use identity_verification::*;
}

pub mod storage {
  //! Storage traits.
  pub use identity_storage::*;
}
