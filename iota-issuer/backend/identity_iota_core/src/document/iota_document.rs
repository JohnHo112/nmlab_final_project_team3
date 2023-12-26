// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;
use identity_credential::credential::Jws;
#[cfg(feature = "client")]
use identity_did::CoreDID;
use identity_did::DIDUrl;
use identity_document::verifiable::JwsVerificationOptions;
use identity_verification::jose::jws::DecodedJws;
use identity_verification::jose::jws::JwsVerifier;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
#[cfg(feature = "client")]
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_document::document::CoreDocument;
use identity_document::service::Service;
use identity_document::utils::DIDUrlQuery;
use identity_verification::MethodRelationship;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;

use crate::error::Result;
use crate::Error;
use crate::IotaDID;
use crate::IotaDocumentMetadata;
use crate::NetworkName;
use crate::StateMetadataDocument;
use crate::StateMetadataEncoding;

/// Struct used internally when deserializing [`IotaDocument`].
#[derive(Debug, Deserialize)]
struct ProvisionalIotaDocument {
  #[serde(rename = "doc")]
  document: CoreDocument,
  #[serde(rename = "meta")]
  metadata: IotaDocumentMetadata,
}

impl TryFrom<ProvisionalIotaDocument> for IotaDocument {
  type Error = Error;
  fn try_from(provisional: ProvisionalIotaDocument) -> std::result::Result<Self, Self::Error> {
    let ProvisionalIotaDocument { document, metadata } = provisional;

    IotaDID::check_validity(document.id()).map_err(|_| {
      Error::SerializationError(
        "deserializing iota document failed: id does not conform to the IOTA method specification",
        None,
      )
    })?;

    for controller_id in document
      .controller()
      .map(|controller_set| controller_set.iter())
      .into_iter()
      .flatten()
    {
      IotaDID::check_validity(controller_id).map_err(|_| {
        Error::SerializationError(
          "deserializing iota document failed: controller not conforming to the iota method specification detected",
          None,
        )
      })?;
    }

    Ok(IotaDocument { document, metadata })
  }
}

/// A DID Document adhering to the IOTA DID method specification.
///
/// This extends [`CoreDocument`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "ProvisionalIotaDocument")]
pub struct IotaDocument {
  /// The DID document.
  #[serde(rename = "doc")]
  pub(crate) document: CoreDocument,
  /// The metadata of an IOTA DID document.
  #[serde(rename = "meta")]
  pub metadata: IotaDocumentMetadata,
}

impl IotaDocument {
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Constructs an empty DID Document with a [`IotaDID::placeholder`] identifier
  /// for the given `network`.
  // TODO: always take Option<NetworkName> or `new_with_options` for a particular network?
  pub fn new(network: &NetworkName) -> Self {
    Self::new_with_id(IotaDID::placeholder(network))
  }

  /// Constructs an empty DID Document with the given identifier.
  pub fn new_with_id(id: IotaDID) -> Self {
    // PANIC: constructing an empty DID Document is infallible, caught by tests otherwise.
    let document: CoreDocument = CoreDocument::builder(Object::default())
      .id(id.into())
      .build()
      .expect("empty IotaDocument constructor failed");
    let metadata: IotaDocumentMetadata = IotaDocumentMetadata::new();
    Self { document, metadata }
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the DID document identifier.
  pub fn id(&self) -> &IotaDID {
    // CORRECTNESS: This cast is OK because the public API does not expose methods
    // enabling unchecked mutation of the `id` field.
    IotaDID::from_inner_ref_unchecked(self.document.id())
  }

  /// Returns an iterator yielding the DID controllers.
  ///
  /// NOTE: controllers are determined by the `state_controller` unlock condition of the output
  /// during resolution and are omitted when publishing.
  pub fn controller(&self) -> impl Iterator<Item = &IotaDID> + '_ {
    let core_did_controller_iter = self
      .document
      .controller()
      .map(|controllers| controllers.iter())
      .into_iter()
      .flatten();

    // CORRECTNESS: These casts are OK because the public API does not expose methods
    // enabling unchecked mutation of the controllers.
    core_did_controller_iter.map(IotaDID::from_inner_ref_unchecked)
  }

  /// Returns a reference to the `alsoKnownAs` set.
  pub fn also_known_as(&self) -> &OrderedSet<Url> {
    self.document.also_known_as()
  }

  /// Returns a mutable reference to the `alsoKnownAs` set.
  pub fn also_known_as_mut(&mut self) -> &mut OrderedSet<Url> {
    self.document.also_known_as_mut()
  }

  /// Returns a reference to the underlying [`CoreDocument`].
  pub fn core_document(&self) -> &CoreDocument {
    &self.document
  }

  /// Returns a mutable reference to the underlying [`CoreDocument`].
  ///
  /// WARNING: Mutating the inner document directly bypasses checks and
  /// may have undesired consequences.
  pub(crate) fn core_document_mut(&mut self) -> &mut CoreDocument {
    &mut self.document
  }

  /// Returns a reference to the custom DID Document properties.
  pub fn properties(&self) -> &Object {
    self.document.properties()
  }

  /// Returns a mutable reference to the custom DID Document properties.
  ///
  /// # Warning
  ///
  /// The properties returned are not checked against the standard fields in a [`CoreDocument`]. Incautious use can have
  /// undesired consequences such as key collision when attempting to serialize the document or distinct resources (such
  /// as services and methods) being identified by the same DID URL.  
  pub fn properties_mut_unchecked(&mut self) -> &mut Object {
    self.document.properties_mut_unchecked()
  }

  // ===========================================================================
  // Services
  // ===========================================================================

  /// Return a set of all [`Service`]s in the document.
  pub fn service(&self) -> &OrderedSet<Service> {
    self.document.service()
  }

  /// Add a new [`Service`] to the document.
  ///
  /// # Errors
  /// An error is returned if there already exists a service or (verification) method with
  /// the same identifier in the document.  
  pub fn insert_service(&mut self, service: Service) -> Result<()> {
    self
      .core_document_mut()
      .insert_service(service)
      .map_err(Error::InvalidDoc)
  }

  /// Remove and return the [`Service`] identified by the given [`DIDUrl`] from the document.
  ///
  /// `None` is returned if the service does not exist in the document.
  pub fn remove_service(&mut self, did_url: &DIDUrl) -> Option<Service> {
    self.core_document_mut().remove_service(did_url)
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Returns a `Vec` of verification method references whose verification relationship matches `scope`.
  ///
  /// If `scope` is `None`, all **embedded** methods are returned.
  pub fn methods(&self, scope: Option<MethodScope>) -> Vec<&VerificationMethod> {
    self.document.methods(scope)
  }

  /// Adds a new [`VerificationMethod`] to the document in the given [`MethodScope`].
  ///
  /// # Errors
  ///
  /// Returns an error if a method with the same fragment already exists.
  pub fn insert_method(&mut self, method: VerificationMethod, scope: MethodScope) -> Result<()> {
    self
      .core_document_mut()
      .insert_method(method, scope)
      .map_err(Error::InvalidDoc)
  }

  /// Removes and returns the [`VerificationMethod`] identified by `did_url` from the document.
  ///
  /// # Note
  ///
  /// All _references to the method_ found in the document will be removed.
  /// This includes cases where the reference is to a method contained in another DID document.
  pub fn remove_method(&mut self, did_url: &DIDUrl) -> Option<VerificationMethod> {
    self.core_document_mut().remove_method(did_url)
  }

  /// Removes and returns the [`VerificationMethod`] from the document. The [`MethodScope`] under which the method was
  /// found is appended to the second position of the returned tuple.
  ///
  /// # Note
  ///
  /// All _references to the method_ found in the document will be removed.
  /// This includes cases where the reference is to a method contained in another DID document.
  pub fn remove_method_and_scope(&mut self, did_url: &DIDUrl) -> Option<(VerificationMethod, MethodScope)> {
    self.core_document_mut().remove_method_and_scope(did_url)
  }

  /// Attaches the relationship to the method resolved by `method_query`.
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist or if it is embedded.
  /// To convert an embedded method into a generic verification method, remove it first
  /// and insert it with [`MethodScope::VerificationMethod`].
  pub fn attach_method_relationship<'query, Q>(
    &mut self,
    method_query: Q,
    relationship: MethodRelationship,
  ) -> Result<bool>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self
      .core_document_mut()
      .attach_method_relationship(method_query, relationship)
      .map_err(Error::InvalidDoc)
  }

  /// Detaches the `relationship` from the method identified by `did_url`.
  /// Returns `true` if the relationship was found and removed, `false` otherwise.
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist or is embedded.
  /// To remove an embedded method, use [`Self::remove_method`].
  ///
  /// # Note
  ///
  /// If the method is referenced in the given scope, but the document does not contain the referenced verification
  /// method, then the reference will persist in the document (i.e. it is not removed).
  pub fn detach_method_relationship<'query, Q>(
    &mut self,
    method_query: Q,
    relationship: MethodRelationship,
  ) -> Result<bool>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self
      .core_document_mut()
      .detach_method_relationship(method_query, relationship)
      .map_err(Error::InvalidDoc)
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the
  /// provided `method_query` and the verification relationship specified by `scope` if present.
  ///
  /// # Warning
  ///
  /// Incorrect use of this method can lead to distinct document resources being identified by the same DID URL.
  pub fn resolve_method_mut<'query, Q>(
    &mut self,
    method_query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&mut VerificationMethod>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.document.resolve_method_mut(method_query, scope)
  }

  /// Returns the first [`Service`] with an `id` property matching the provided `service_query`, if present.
  // NOTE: This method demonstrates unexpected behaviour in the edge cases where the document contains
  // services whose ids are of the form <did different from this document's>#<fragment>.
  pub fn resolve_service<'query, 'me, Q>(&'me self, service_query: Q) -> Option<&Service>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.document.resolve_service(service_query)
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the
  /// provided `method_query` and the verification relationship specified by `scope` if present.
  // NOTE: This method demonstrates unexpected behaviour in the edge cases where the document contains methods
  // whose ids are of the form <did different from this document's>#<fragment>.
  pub fn resolve_method<'query, 'me, Q>(
    &'me self,
    method_query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&VerificationMethod>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.document.resolve_method(method_query, scope)
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

  /// Decodes and verifies the provided JWS according to the passed [`JwsVerificationOptions`] and
  /// [`JwsVerifier`].
  ///
  /// Regardless of which options are passed the following conditions must be met in order for a verification attempt to
  /// take place.
  /// - The JWS must be encoded according to the JWS compact serialization.
  /// - The `kid` value in the protected header must be an identifier of a verification method in this DID document.
  pub fn verify_jws<'jws, T: JwsVerifier>(
    &self,
    jws: &'jws Jws,
    detached_payload: Option<&'jws [u8]>,
    signature_verifier: &T,
    options: &JwsVerificationOptions,
  ) -> Result<DecodedJws<'jws>> {
    self
      .core_document()
      .verify_jws(jws.as_str(), detached_payload, signature_verifier, options)
      .map_err(Error::JwsVerificationError)
  }

  // ===========================================================================
  // Packing
  // ===========================================================================

  /// Serializes the document for inclusion in an Alias Output's state metadata
  /// with the default [`StateMetadataEncoding`].
  pub fn pack(self) -> Result<Vec<u8>> {
    self.pack_with_encoding(StateMetadataEncoding::default())
  }

  /// Serializes the document for inclusion in an Alias Output's state metadata.
  pub fn pack_with_encoding(self, encoding: StateMetadataEncoding) -> Result<Vec<u8>> {
    StateMetadataDocument::from(self).pack(encoding)
  }
}

#[cfg(feature = "client")]
mod client_document {
  use iota_sdk::types::block::address::Hrp;
  use iota_sdk::types::block::address::ToBech32Ext;

  use crate::block::address::Address;
  use crate::block::output::AliasId;
  use crate::block::output::AliasOutput;
  use crate::block::output::Output;
  use crate::block::output::OutputId;
  use crate::block::payload::transaction::TransactionEssence;
  use crate::block::payload::Payload;
  use crate::block::Block;
  use crate::error::Result;
  use crate::Error;
  use crate::NetworkName;

  use super::*;

  impl IotaDocument {
    // ===========================================================================
    // Unpacking
    // ===========================================================================

    /// Deserializes the document from an Alias Output.
    ///
    /// If `allow_empty` is true, this will return an empty DID document marked as `deactivated`
    /// if `state_metadata` is empty.
    ///
    /// NOTE: `did` is required since it is omitted from the serialized DID Document and
    /// cannot be inferred from the state metadata. It also indicates the network, which is not
    /// encoded in the `AliasId` alone.
    pub fn unpack_from_output(did: &IotaDID, alias_output: &AliasOutput, allow_empty: bool) -> Result<IotaDocument> {
      let mut document: IotaDocument = if alias_output.state_metadata().is_empty() && allow_empty {
        let mut empty_document = IotaDocument::new_with_id(did.clone());
        empty_document.metadata.created = None;
        empty_document.metadata.updated = None;
        empty_document.metadata.deactivated = Some(true);
        empty_document
      } else {
        StateMetadataDocument::unpack(alias_output.state_metadata()).and_then(|doc| doc.into_iota_document(did))?
      };

      document.set_controller_and_governor_addresses(alias_output, &did.network_str().to_owned().try_into()?)?;

      Ok(document)
    }

    fn set_controller_and_governor_addresses(
      &mut self,
      alias_output: &AliasOutput,
      network_name: &NetworkName,
    ) -> Result<()> {
      let hrp: Hrp = network_name.try_into()?;
      self.metadata.governor_address = Some(alias_output.governor_address().to_bech32(hrp).to_string());
      self.metadata.state_controller_address = Some(alias_output.state_controller_address().to_bech32(hrp).to_string());

      // Overwrite the DID Document controller.
      let controller_did: Option<IotaDID> = match alias_output.state_controller_address() {
        Address::Alias(alias_address) => Some(IotaDID::new(alias_address.alias_id(), network_name)),
        _ => None,
      };

      *self.core_document_mut().controller_mut() = controller_did.map(CoreDID::from).map(OneOrSet::new_one);

      Ok(())
    }

    /// Returns all DID documents of the Alias Outputs contained in the block's transaction payload
    /// outputs, if any.
    ///
    /// Errors if any Alias Output does not contain a valid or empty DID Document.
    pub fn unpack_from_block(network: &NetworkName, block: &Block) -> Result<Vec<IotaDocument>> {
      let mut documents = Vec::new();

      if let Some(Payload::Transaction(tx_payload)) = block.payload() {
        let TransactionEssence::Regular(regular) = tx_payload.essence();

        for (index, output) in regular.outputs().iter().enumerate() {
          if let Output::Alias(alias_output) = output {
            let alias_id = if alias_output.alias_id().is_null() {
              AliasId::from(
                &OutputId::new(
                  tx_payload.id(),
                  index
                    .try_into()
                    .map_err(|_| Error::OutputIdConversionError(format!("output index {index} must fit into a u16")))?,
                )
                .map_err(|err| Error::OutputIdConversionError(err.to_string()))?,
              )
            } else {
              alias_output.alias_id().to_owned()
            };

            let did: IotaDID = IotaDID::new(&alias_id, network);
            documents.push(IotaDocument::unpack_from_output(&did, alias_output, true)?);
          }
        }
      }

      Ok(documents)
    }
  }
}

impl AsRef<CoreDocument> for IotaDocument {
  fn as_ref(&self) -> &CoreDocument {
    &self.document
  }
}

#[cfg(feature = "revocation-bitmap")]
mod iota_document_revocation {
  use identity_credential::revocation::RevocationDocumentExt;
  use identity_document::utils::DIDUrlQuery;

  use crate::Error;
  use crate::Result;

  use super::IotaDocument;

  impl IotaDocument {
    /// If the document has a [`RevocationBitmap`](identity_credential::revocation::RevocationBitmap)
    /// service identified by `service_query`, revoke all specified `indices`.
    pub fn revoke_credentials<'query, 'me, Q>(&mut self, service_query: Q, indices: &[u32]) -> Result<()>
    where
      Q: Into<DIDUrlQuery<'query>>,
    {
      self
        .core_document_mut()
        .revoke_credentials(service_query, indices)
        .map_err(Error::RevocationError)
    }

    /// If the document has a [`RevocationBitmap`](identity_credential::revocation::RevocationBitmap)
    /// service with an id by `service_query`, unrevoke all specified `indices`.
    pub fn unrevoke_credentials<'query, 'me, Q>(&'me mut self, service_query: Q, indices: &[u32]) -> Result<()>
    where
      Q: Into<DIDUrlQuery<'query>>,
    {
      self
        .core_document_mut()
        .unrevoke_credentials(service_query, indices)
        .map_err(Error::RevocationError)
    }
  }
}

impl From<IotaDocument> for CoreDocument {
  fn from(document: IotaDocument) -> Self {
    document.document
  }
}

impl TryFrom<(CoreDocument, IotaDocumentMetadata)> for IotaDocument {
  type Error = Error;
  /// Converts the tuple into an [`IotaDocument`] if the given [`CoreDocument`] has an identifier satisfying the
  /// requirements of the IOTA UTXO method and the same holds for all of the [`CoreDocument's`](CoreDocument)
  /// controllers.
  ///
  /// # Important
  /// This does not check the relationship between the [`CoreDocument`] and the [`IotaDocumentMetadata`].
  fn try_from(value: (CoreDocument, IotaDocumentMetadata)) -> std::result::Result<Self, Self::Error> {
    ProvisionalIotaDocument {
      document: value.0,
      metadata: value.1,
    }
    .try_into()
  }
}

impl Display for IotaDocument {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.fmt_json(f)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Timestamp;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;
  use identity_did::DID;

  use crate::block::address::Address;
  use crate::block::address::AliasAddress;
  use crate::block::output::unlock_condition::GovernorAddressUnlockCondition;
  use crate::block::output::unlock_condition::StateControllerAddressUnlockCondition;
  use crate::block::output::AliasId;
  use crate::block::output::AliasOutput;
  use crate::block::output::AliasOutputBuilder;
  use crate::block::output::UnlockCondition;

  use super::*;
  use crate::test_utils::generate_method;

  fn valid_did() -> IotaDID {
    "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      .parse()
      .unwrap()
  }

  fn generate_document(id: &IotaDID) -> IotaDocument {
    let mut metadata: IotaDocumentMetadata = IotaDocumentMetadata::new();
    metadata.created = Some(Timestamp::parse("2020-01-02T00:00:00Z").unwrap());
    metadata.updated = Some(Timestamp::parse("2020-01-02T00:00:00Z").unwrap());

    let document: CoreDocument = CoreDocument::builder(Object::default())
      .id(id.clone().into())
      .controller(id.clone().into())
      .verification_method(generate_method(id, "#key-1"))
      .verification_method(generate_method(id, "#key-2"))
      .verification_method(generate_method(id, "#key-3"))
      .authentication(generate_method(id, "#auth-key"))
      .authentication(id.to_url().join("#key-3").unwrap())
      .build()
      .unwrap();

    IotaDocument { document, metadata }
  }

  #[test]
  fn test_new() {
    // VALID new().
    let network: NetworkName = NetworkName::try_from("test").unwrap();
    let placeholder: IotaDID = IotaDID::placeholder(&network);
    let doc1: IotaDocument = IotaDocument::new(&network);
    assert_eq!(doc1.id().network_str(), network.as_ref());
    assert_eq!(doc1.id().tag_str(), placeholder.tag_str());
    assert_eq!(doc1.id(), &placeholder);
    assert_eq!(doc1.methods(None).len(), 0);
    assert!(doc1.service().is_empty());

    // VALID new_with_id().
    let did: IotaDID = valid_did();
    let doc2: IotaDocument = IotaDocument::new_with_id(did.clone());
    assert_eq!(doc2.id(), &did);
    assert_eq!(doc2.methods(None).len(), 0);
    assert!(doc2.service().is_empty());
  }

  #[test]
  fn test_methods() {
    let controller: IotaDID = valid_did();
    let document: IotaDocument = generate_document(&controller);
    let expected: [&'static str; 4] = ["key-1", "key-2", "key-3", "auth-key"];

    let mut methods = document.methods(None).into_iter();
    assert_eq!(methods.next().unwrap().id().fragment().unwrap(), expected[0]);
    assert_eq!(methods.next().unwrap().id().fragment().unwrap(), expected[1]);
    assert_eq!(methods.next().unwrap().id().fragment().unwrap(), expected[2]);
    assert_eq!(methods.next().unwrap().id().fragment().unwrap(), expected[3]);
    assert_eq!(methods.next(), None);
  }

  #[test]
  fn test_services() {
    // VALID: add one service.
    let mut document: IotaDocument = IotaDocument::new_with_id(valid_did());
    let url1: DIDUrl = document.id().to_url().join("#linked-domain").unwrap();
    let service1: Service = Service::from_json(&format!(
      r#"{{
      "id":"{url1}",
      "type": "LinkedDomains",
      "serviceEndpoint": "https://bar.example.com"
    }}"#
    ))
    .unwrap();
    assert!(document.insert_service(service1.clone()).is_ok());
    assert_eq!(1, document.service().len());
    assert_eq!(document.resolve_service(&url1), Some(&service1));
    assert_eq!(document.resolve_service("#linked-domain"), Some(&service1));
    assert_eq!(document.resolve_service("linked-domain"), Some(&service1));
    assert_eq!(document.resolve_service(""), None);
    assert_eq!(document.resolve_service("#other"), None);

    // VALID: add two services.
    let url2: DIDUrl = document.id().to_url().join("#revocation").unwrap();
    let service2: Service = Service::from_json(&format!(
      r#"{{
      "id":"{url2}",
      "type": "RevocationBitmap2022",
      "serviceEndpoint": "data:,blah"
    }}"#
    ))
    .unwrap();
    assert!(document.insert_service(service2.clone()).is_ok());
    assert_eq!(2, document.service().len());
    assert_eq!(document.resolve_service(&url2), Some(&service2));
    assert_eq!(document.resolve_service("#revocation"), Some(&service2));
    assert_eq!(document.resolve_service("revocation"), Some(&service2));
    assert_eq!(document.resolve_service(""), None);
    assert_eq!(document.resolve_service("#other"), None);

    // INVALID: insert service with duplicate fragment fails.
    let duplicate: Service = Service::from_json(&format!(
      r#"{{
      "id":"{url1}",
      "type": "DuplicateService",
      "serviceEndpoint": "data:,duplicate"
    }}"#
    ))
    .unwrap();
    assert!(document.insert_service(duplicate.clone()).is_err());
    assert_eq!(2, document.service().len());
    let resolved: &Service = document.resolve_service(&url1).unwrap();
    assert_eq!(resolved, &service1);
    assert_ne!(resolved, &duplicate);

    // VALID: remove services.
    assert_eq!(service1, document.remove_service(&url1).unwrap());
    assert_eq!(1, document.service().len());
    let last_service: &Service = document.resolve_service(&url2).unwrap();
    assert_eq!(last_service, &service2);

    assert_eq!(service2, document.remove_service(&url2).unwrap());
    assert_eq!(0, document.service().len());
  }

  #[test]
  fn test_document_equality() {
    let mut original_doc: IotaDocument = IotaDocument::new_with_id(valid_did());
    let method1: VerificationMethod = generate_method(original_doc.id(), "test-0");
    original_doc
      .insert_method(method1, MethodScope::capability_invocation())
      .unwrap();

    // Update the key material of the existing verification method #test-0.
    let mut doc1 = original_doc.clone();
    let method2: VerificationMethod = generate_method(original_doc.id(), "test-0");

    doc1
      .remove_method(&doc1.id().to_url().join("#test-0").unwrap())
      .unwrap();
    doc1
      .insert_method(method2, MethodScope::capability_invocation())
      .unwrap();

    // Even though the method fragment is the same, the key material has been updated
    // so the two documents are expected to not be equal.
    assert_ne!(original_doc, doc1);

    let mut doc2 = doc1.clone();
    let method3: VerificationMethod = generate_method(original_doc.id(), "test-0");

    let insertion_result = doc2.insert_method(method3, MethodScope::capability_invocation());

    // Nothing was inserted, because a method with the same fragment already existed.
    assert!(insertion_result.is_err());
    assert_eq!(doc1, doc2);
  }

  #[test]
  fn test_unpack_empty() {
    let controller_did: IotaDID = valid_did();

    // VALID: unpack empty, deactivated document.
    let did: IotaDID = "did:iota:0xBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB"
      .parse()
      .unwrap();
    let alias_output: AliasOutput = AliasOutputBuilder::new_with_amount(1, AliasId::from(&did))
      .add_unlock_condition(UnlockCondition::StateControllerAddress(
        StateControllerAddressUnlockCondition::new(Address::Alias(AliasAddress::new(AliasId::from(&controller_did)))),
      ))
      .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
        Address::Alias(AliasAddress::new(AliasId::from(&controller_did))),
      )))
      .finish()
      .unwrap();
    let document: IotaDocument = IotaDocument::unpack_from_output(&did, &alias_output, true).unwrap();
    assert_eq!(document.id(), &did);
    assert_eq!(document.metadata.deactivated, Some(true));

    // Ensure no other fields are injected.
    let json: String = format!(
      r#"{{"doc":{{"id":"{did}","controller":"{controller_did}"}},"meta":{{"deactivated":true,"governorAddress":"iota1pz424242424242424242424242424242424242424242424242425ryaqzy","stateControllerAddress":"iota1pz424242424242424242424242424242424242424242424242425ryaqzy"}}}}"#
    );
    assert_eq!(document.to_json().unwrap(), json);

    // INVALID: reject empty document.
    assert!(IotaDocument::unpack_from_output(&did, &alias_output, false).is_err());

    // Ensure re-packing removes the controller, state controller address, and governor address.
    let packed: Vec<u8> = document.pack_with_encoding(StateMetadataEncoding::Json).unwrap();
    let state_metadata_document: StateMetadataDocument = StateMetadataDocument::unpack(&packed).unwrap();
    let unpacked_document: IotaDocument = state_metadata_document.into_iota_document(&did).unwrap();
    assert!(unpacked_document.document.controller().is_none());
    assert!(unpacked_document.metadata.state_controller_address.is_none());
    assert!(unpacked_document.metadata.governor_address.is_none());
  }

  #[test]
  fn test_json_roundtrip() {
    let document: IotaDocument = generate_document(&valid_did());

    let ser: String = document.to_json().unwrap();
    let de: IotaDocument = IotaDocument::from_json(&ser).unwrap();
    assert_eq!(document, de);
  }

  #[test]
  fn test_json_fieldnames() {
    // Changing the serialization is a breaking change!
    let document: IotaDocument = IotaDocument::new_with_id(valid_did());
    let serialization: String = document.to_json().unwrap();
    assert_eq!(
      serialization,
      format!("{{\"doc\":{},\"meta\":{}}}", document.document, document.metadata)
    );
  }

  #[test]
  fn deserializing_id_from_other_method_fails() {
    const JSON_DOC_INVALID_ID: &str = r#"
    {
      "doc": {
        "id": "did:foo:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "verificationMethod": [
          {
            "id": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa#issuerKey",
            "controller": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "type": "Ed25519VerificationKey2018",
            "publicKeyMultibase": "zFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z"
          }
        ]
      },
      "meta": {
        "created": "2022-08-31T09:33:31Z",
        "updated": "2022-08-31T09:33:31Z"
      }
    }"#;

    let deserialization_result = IotaDocument::from_json(&JSON_DOC_INVALID_ID);

    assert!(deserialization_result.is_err());

    // Check that deserialization works after correcting the json document to have a valid IOTA DID as its identifier.
    const JSON_DOC_CORRECT_ID: &str = r#"
    {
      "doc": {
        "id": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "verificationMethod": [
          {
            "id": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa#issuerKey",
            "controller": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "type": "Ed25519VerificationKey2018",
            "publicKeyMultibase": "zFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z"
          }
        ]
      },
      "meta": {
        "created": "2022-08-31T09:33:31Z",
        "updated": "2022-08-31T09:33:31Z"
      }
    }"#;

    let corrected_deserialization_result = IotaDocument::from_json(&JSON_DOC_CORRECT_ID);
    assert!(corrected_deserialization_result.is_ok());
  }

  #[test]
  fn deserializing_controller_from_other_method_fails() {
    const JSON_DOC_INVALID_CONTROLLER_ID: &str = r#"
    {
    "doc": {
      "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
      "controller": "did:example:rms:0xfbaaa919b51112d51a8f18b1500d98f0b2e91d793bc5b27fd5ab04cb1b806343",
      "verificationMethod": [
        {
          "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38#key-2",
          "controller": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
          "type": "Ed25519VerificationKey2018",
          "publicKeyMultibase": "z7eTUXFdLCFg1LFVFhG8qUAM2aSjfTuPLB2x9XGXgQh6G"
        }
      ]
    },
    "meta": {
      "created": "2023-01-25T15:48:09Z",
      "updated": "2023-01-25T15:48:09Z",
      "governorAddress": "rms1pra642gek5g394g63uvtz5qdnrct96ga0yautvnl6k4sfjcmsp35xv6nagu",
      "stateControllerAddress": "rms1pra642gek5g394g63uvtz5qdnrct96ga0yautvnl6k4sfjcmsp35xv6nagu"
    }
  }
  "#;

    let deserialization_result = IotaDocument::from_json(&JSON_DOC_INVALID_CONTROLLER_ID);
    assert!(deserialization_result.is_err());

    // Check that deserialization works after correcting the json document to have a valid IOTA DID as the controller.
    const JSON_DOC_CORRECT_CONTROLLER_ID: &str = r#"
  {
  "doc": {
    "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
    "controller": "did:iota:rms:0xfbaaa919b51112d51a8f18b1500d98f0b2e91d793bc5b27fd5ab04cb1b806343",
    "verificationMethod": [
      {
        "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38#key-2",
        "controller": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "z7eTUXFdLCFg1LFVFhG8qUAM2aSjfTuPLB2x9XGXgQh6G"
      }
    ]
  },
  "meta": {
    "created": "2023-01-25T15:48:09Z",
    "updated": "2023-01-25T15:48:09Z",
    "governorAddress": "rms1pra642gek5g394g63uvtz5qdnrct96ga0yautvnl6k4sfjcmsp35xv6nagu",
    "stateControllerAddress": "rms1pra642gek5g394g63uvtz5qdnrct96ga0yautvnl6k4sfjcmsp35xv6nagu"
  }
}
"#;
    let corrected_deserialization_result = IotaDocument::from_json(JSON_DOC_CORRECT_CONTROLLER_ID);
    assert!(corrected_deserialization_result.is_ok());
  }

  #[test]
  fn controller_iterator_without_controller() {
    const DOC_JSON: &str = r#"
    {
      "doc": {
        "id": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
      },
      "meta": {
        "created": "2022-08-31T09:33:31Z",
        "updated": "2022-08-31T09:33:31Z"
      }
    }
    "#;

    let doc = IotaDocument::from_json(DOC_JSON).unwrap();
    assert!(doc.controller().next().is_none());
  }

  #[test]
  fn controller_iterator_with_controller() {
    const DOC_JSON: &str = r#"
  {
    "doc": {
      "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
      "controller": "did:iota:rms:0xfbaaa919b51112d51a8f18b1500d98f0b2e91d793bc5b27fd5ab04cb1b806343"
    },
    "meta": {
      "created": "2023-01-25T15:48:09Z",
      "updated": "2023-01-25T15:48:09Z",
      "governorAddress": "rms1pra642gek5g394g63uvtz5qdnrct96ga0yautvnl6k4sfjcmsp35xv6nagu",
      "stateControllerAddress": "rms1pra642gek5g394g63uvtz5qdnrct96ga0yautvnl6k4sfjcmsp35xv6nagu"
    }
  }
  "#;
    let doc = IotaDocument::from_json(DOC_JSON).unwrap();
    let expected_controller =
      IotaDID::parse("did:iota:rms:0xfbaaa919b51112d51a8f18b1500d98f0b2e91d793bc5b27fd5ab04cb1b806343").unwrap();
    let controllers: Vec<&IotaDID> = doc.controller().collect();
    assert_eq!(&controllers, &[&expected_controller]);
  }

  #[test]
  fn try_from_doc_metadata() {
    const DOC_JSON_NOT_IOTA_DOCUMENT_BECAUSE_OF_ID: &str = r#"
    {
      "id": "did:foo:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      "verificationMethod": [
        {
          "id": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa#issuerKey",
          "controller": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          "type": "Ed25519VerificationKey2018",
          "publicKeyMultibase": "zFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z"
        }
      ]
    }
    "#;

    const DOC_JSON_NOT_IOTA_DOCUMENT_BECAUSE_OF_CONTROLLER: &str = r#"
    {
      "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
      "controller": "did:example:rms:0xfbaaa919b51112d51a8f18b1500d98f0b2e91d793bc5b27fd5ab04cb1b806343",
      "verificationMethod": [
        {
          "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38#key-2",
          "controller": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
          "type": "Ed25519VerificationKey2018",
          "publicKeyMultibase": "z7eTUXFdLCFg1LFVFhG8qUAM2aSjfTuPLB2x9XGXgQh6G"
        }
      ]
    }
    "#;

    const METADATA_JSON: &str = r#"
    {
      "created": "2022-08-31T09:33:31Z",
      "updated": "2022-08-31T09:33:31Z"
    }
    "#;

    const DOCUMENT_WITH_IOTA_ID_AND_CONTROLLER_JSON: &str = r#"
    {
      "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
      "controller": "did:iota:rms:0xfbaaa919b51112d51a8f18b1500d98f0b2e91d793bc5b27fd5ab04cb1b806343",
      "verificationMethod": [
        {
          "id": "did:foo:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa#issuerKey",
          "controller": "did:bar:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          "type": "Ed25519VerificationKey2018",
          "publicKeyMultibase": "zFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z"
        }
      ]
    }
    "#;

    let doc_not_iota_because_of_id: CoreDocument =
      CoreDocument::from_json(DOC_JSON_NOT_IOTA_DOCUMENT_BECAUSE_OF_ID).unwrap();
    let doc_not_iota_because_of_controller: CoreDocument =
      CoreDocument::from_json(DOC_JSON_NOT_IOTA_DOCUMENT_BECAUSE_OF_CONTROLLER).unwrap();
    let doc_with_iota_id_and_controller: CoreDocument =
      CoreDocument::from_json(DOCUMENT_WITH_IOTA_ID_AND_CONTROLLER_JSON).unwrap();
    let metadata: IotaDocumentMetadata = IotaDocumentMetadata::from_json(METADATA_JSON).unwrap();

    assert!(IotaDocument::try_from((doc_not_iota_because_of_id, metadata.clone())).is_err());

    assert!(IotaDocument::try_from((doc_not_iota_because_of_controller, metadata.clone())).is_err());

    assert!(IotaDocument::try_from((doc_with_iota_id_and_controller, metadata)).is_ok());
  }
}
