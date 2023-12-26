// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_storage::JwkStorage;
use crate::key_storage::KeyId;
use crate::key_storage::KeyStorageErrorKind;
use crate::key_storage::KeyType;
use crypto::signatures::ed25519::PublicKey;
use crypto::signatures::ed25519::SecretKey;
use crypto::signatures::ed25519::Signature;
use identity_verification::jose::jwk::EdCurve;
use identity_verification::jose::jwk::Jwk;
use identity_verification::jose::jwk::JwkParamsOkp;
use identity_verification::jose::jwu;
use identity_verification::jwk::EcCurve;
use identity_verification::jwk::JwkParamsEc;
use identity_verification::jws::JwsAlgorithm;

pub(crate) async fn test_insertion(store: impl JwkStorage) {
  let (private_key, public_key) = generate_ed25519();
  let mut jwk: Jwk = crate::key_storage::ed25519::encode_jwk(&private_key, &public_key);

  // INVALID: Inserting a Jwk without an `alg` parameter should fail.
  let err = store.insert(jwk.clone()).await.unwrap_err();
  assert!(matches!(err.kind(), KeyStorageErrorKind::UnsupportedSignatureAlgorithm));

  // VALID: Inserting a Jwk with all private key components set should succeed.
  jwk.set_alg(JwsAlgorithm::EdDSA.name());
  store.insert(jwk.clone()).await.unwrap();

  // INVALID: Inserting a Jwk with all private key components unset should fail.
  let err = store.insert(jwk.to_public().unwrap()).await.unwrap_err();
  assert!(matches!(err.kind(), KeyStorageErrorKind::Unspecified))
}

pub(crate) async fn test_incompatible_key_alg(store: impl JwkStorage) {
  let (private_key, public_key) = generate_ed25519();
  let mut jwk: Jwk = crate::key_storage::ed25519::encode_jwk(&private_key, &public_key);
  jwk.set_alg(JwsAlgorithm::ES256.name());

  // INVALID: Inserting an Ed25519 key with the ES256 alg is not compatible.
  let err = store.insert(jwk.clone()).await.unwrap_err();
  assert!(matches!(err.kind(), KeyStorageErrorKind::KeyAlgorithmMismatch));
}

pub(crate) async fn test_incompatible_key_type(store: impl JwkStorage) {
  let mut ec_params = JwkParamsEc::new();
  ec_params.crv = EcCurve::P256.name().to_owned();
  ec_params.x = "".to_owned();
  ec_params.y = "".to_owned();
  ec_params.d = Some("".to_owned());
  let jwk_ec = Jwk::from_params(ec_params);

  let err = store.insert(jwk_ec).await.unwrap_err();
  assert!(matches!(err.kind(), KeyStorageErrorKind::UnsupportedKeyType));
}
pub(crate) async fn test_generate_and_sign(store: impl JwkStorage) {
  let test_msg: &[u8] = b"test";

  let generate = store
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await
    .unwrap();

  let signature = store.sign(&generate.key_id, test_msg, &generate.jwk).await.unwrap();

  let signature: Signature = Signature::from_bytes(signature.try_into().unwrap());
  let public_key: PublicKey = expand_public_jwk(&generate.jwk);
  assert!(public_key.verify(&signature, test_msg));

  let key_id: KeyId = generate.key_id;
  assert!(store.exists(&key_id).await.unwrap());
  store.delete(&key_id).await.unwrap();
}

pub(crate) async fn test_key_exists(store: impl JwkStorage) {
  assert!(!store.exists(&KeyId::new("non-existent-id")).await.unwrap());
}

pub(crate) fn expand_public_jwk(jwk: &Jwk) -> PublicKey {
  let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

  if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
    panic!("expected an ed25519 jwk");
  }

  let pk: [u8; PublicKey::LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();

  PublicKey::try_from(pk).unwrap()
}

pub(crate) fn generate_ed25519() -> (SecretKey, PublicKey) {
  let private_key = SecretKey::generate().unwrap();
  let public_key = private_key.public_key();
  (private_key, public_key)
}
