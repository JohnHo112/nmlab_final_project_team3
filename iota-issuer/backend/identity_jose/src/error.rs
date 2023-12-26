// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur in the library.

/// Alias for a `Result` with the error type [Error].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// All possible errors that can occur in the library.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  /// Caused by invalid json serialization or deserialization.
  #[error("invalid json")]
  InvalidJson(#[source] serde_json::Error),
  /// Caused by invalid base64 encoded data.
  #[error("invalid base64")]
  InvalidBase64(#[source] identity_core::error::Error),
  /// Caused by bytes not being valid utf-8.
  #[error("invalid utf-8")]
  InvalidUtf8(#[source] core::str::Utf8Error),
  /// Caused by a claim not being of an expected value.
  #[error("invalid claim `{0}`")]
  InvalidClaim(&'static str),
  /// Caused by a missing claim.
  #[error("missing claim `{0}`")]
  MissingClaim(&'static str),
  /// Caused by an parameter with an invalid value.
  #[error("invalid param: {0}")]
  InvalidParam(&'static str),
  /// Caused by a missing parameter.
  #[error("missing param `{0}`")]
  MissingParam(&'static str),
  /// Caused by invalid content of a JSON Web Signature.
  #[error("{0}")]
  InvalidContent(&'static str),
  /// Caused by an invalid key format.
  #[error("invalid key format for type `{0}`")]
  KeyError(&'static str),
  /// Caused by a string that does not correspond to a supported [`JwsAlgorithm`](crate::jws::JwsAlgorithm).
  #[error("attempt to parse an unregistered jws algorithm")]
  JwsAlgorithmParsingError,
  /// Caused by an error during signature verification.
  #[error("signature verification error")]
  SignatureVerificationError(#[source] crate::jws::SignatureVerificationError),
  /// Caused by a mising header.
  #[error("missing header")]
  MissingHeader(&'static str),
  /// Caused by a missing `alg` claim in the protected header.
  #[error("missing alg in protected header")]
  ProtectedHeaderWithoutAlg,
}
