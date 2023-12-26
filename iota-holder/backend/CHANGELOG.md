# Changelog

## [1.0.0](https://github.com/iotaledger/identity.rs/tree/v1.0.0) (2023-11-02)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.6.3...v1.0.0)

This version introduces a new DID method targeting the IOTA UTXO ledger. This method works fundamentally differently from the previous method and introduces new capabilities to interact with Layer 1 assets like Native Tokens, NFTs and various Output types.

This version changes the credential and presentation format to JWT, as specified by the [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

Note: Identities and credentials created with the earlier versions cannot be resolved with this version of the library.

### Changed

- Add dedicated stronghold crate [#1243](https://github.com/iotaledger/identity.rs/pull/1243)
- Add dedicated EdDSA verifier crate [#1238](https://github.com/iotaledger/identity.rs/pull/1238)
- Use `VC Data Model v1.1` JWT encoding instead of `VC-JWT` [#1234](https://github.com/iotaledger/identity.rs/pull/1234)
- Remove `vp` and `vc` from JWT claims in JOSE [#1233](https://github.com/iotaledger/identity.rs/pull/1233)
- Mark error enums as non-exhaustive [#1227](https://github.com/iotaledger/identity.rs/pull/1227)
- Change `verifiable_credential` to type `Vec<CRED>` in `Presentation` [#1231](https://github.com/iotaledger/identity.rs/pull/1231)
- Bring JwkDocumentExt names in line with Wasm [#1233](https://github.com/iotaledger/identity.rs/pull/1223)
- Add lints for all crates [#1222](https://github.com/iotaledger/identity.rs/pull/1222)
- Bump `iota-sdk` and other dependencies [#1208](https://github.com/iotaledger/identity.rs/pull/1208)
- Polish `identity_credential` [#1205](https://github.com/iotaledger/identity.rs/pull/1205)
- Polish `identity_resolver` and`identity_storage` [#1204](https://github.com/iotaledger/identity.rs/pull/1204)
- Polish `identity_iota_core` [#1203](https://github.com/iotaledger/identity.rs/pull/1203)
- Rename `JwtPresentation` to `Presentation` [#1200](https://github.com/iotaledger/identity.rs/pull/1200)
- Polish `identity_document` [#1198](https://github.com/iotaledger/identity.rs/pull/1198)
- Polish `identity_did` & `identity_verification` [#1197](https://github.com/iotaledger/identity.rs/pull/1197)
- Polish `identity_core` [#1196](https://github.com/iotaledger/identity.rs/pull/1196)
- Remove identity-diff remains [#1195](https://github.com/iotaledger/identity.rs/pull/1195)
- Remove legacy signing and verification APIs [#1194](https://github.com/iotaledger/identity.rs/pull/1194)
- Remove old `Presentation` type [#1190](https://github.com/iotaledger/identity.rs/pull/1190)
- Remove reexported `Resolver` validation APIs [#1183](https://github.com/iotaledger/identity.rs/pull/1183)
- Use JWT credentials for Domain Linkage [#1180](https://github.com/iotaledger/identity.rs/pull/1180)
- Remove `identity_agent` & `identity_comm` [#1168](https://github.com/iotaledger/identity.rs/pull/1168)
- Remove `identity-diff` crate [#1167](https://github.com/iotaledger/identity.rs/pull/1167)
- JwkStorageDocument & JwtCredential validation [#1152](https://github.com/iotaledger/identity.rs/pull/1152)
- Adapt StorageError to be more generic [#1144](https://github.com/iotaledger/identity.rs/pull/1144)
- Add initial PublicKeyJwk support [#1143](https://github.com/iotaledger/identity.rs/pull/1143)
- Split JWS `Decoder` functionality [#1133](https://github.com/iotaledger/identity.rs/pull/1133)
- `CoreDocument` & `Service` and `VerificationMethod` are now in the `document` and `verification` modules respectively [#1104](https://github.com/iotaledger/identity.rs/pull/1104)
- Remove generics in CoreDocument, VerificationMethod, Service, DIDUrl and LinkedDomainService [#1110](https://github.com/iotaledger/identity.rs/pull/1110)
- Updated `iota-types` dependency to `1.0.0-rc.6` [#1121](https://github.com/iotaledger/identity.rs/pull/1121)
- Refactor `MethodType` to make it extensible [#1112](https://github.com/iotaledger/identity.rs/pull/1112)
- More identifier checks in `CoreDocument` [#1067](https://github.com/iotaledger/identity.rs/pull/1067)
- Update to `iota-client` 2.0.1-rc.4 and `iota-client-wasm` 0.5.0-alpha.6 [#1088](https://github.com/iotaledger/identity.rs/pull/1088)
- Update iota client 2.0.1 rc.3 [\#1062](https://github.com/iotaledger/identity.rs/pull/1062)
- Use Bech32-encoded state controller and governor addresses [\#1044](https://github.com/iotaledger/identity.rs/pull/1044)
- Expose iteration over verification relationship fields [\#1024](https://github.com/iotaledger/identity.rs/pull/1024)
- Remove `identity_agent` reexport [\#1031](https://github.com/iotaledger/identity.rs/pull/1031)
- Rename `MixedResolver` to `Resolver` in Wasm [\#1026](https://github.com/iotaledger/identity.rs/pull/1026)
- Add length prefix to DID Document payloads [\#1010](https://github.com/iotaledger/identity.rs/pull/1010)
- Feature-gate `Resolver` [\#1007](https://github.com/iotaledger/identity.rs/pull/1007)
- Rename `Stardust` types to `Iota` [\#1000](https://github.com/iotaledger/identity.rs/pull/1000)
- Change Stardust DID method to IOTA [\#982](https://github.com/iotaledger/identity.rs/pull/982)
- Add Wasm Stardust Client [\#975](https://github.com/iotaledger/identity.rs/pull/975)
- Generalized Resolver [\#970](https://github.com/iotaledger/identity.rs/pull/970)
- Change `Storage` to handle `CoreDID` [\#968](https://github.com/iotaledger/identity.rs/pull/968)
- Feature-gate `iota-client` dependency, integrate `StardustDID` [\#958](https://github.com/iotaledger/identity.rs/pull/958)
- Change `Storage` to store arbitrary blobs [\#953](https://github.com/iotaledger/identity.rs/pull/953)
- Add `StardustDocumentMetadata`, implement `StardustDocument` methods [\#951](https://github.com/iotaledger/identity.rs/pull/951)
- Fix stack overflow in `CoreDID` `PartialEq` impl [\#946](https://github.com/iotaledger/identity.rs/pull/946)
- Change `Service` `type` field to allow sets [\#944](https://github.com/iotaledger/identity.rs/pull/944)
- Generalise `CredentialValidator`, `PresentationValidator` to support arbitrary DID Documents [\#935](https://github.com/iotaledger/identity.rs/pull/935)


### Added

- Allow arbitrary JWS header parameters [#1245](https://github.com/iotaledger/identity.rs/pull/1245)
- Allow custom JWT claims for presentations [#1244](https://github.com/iotaledger/identity.rs/pull/1244)
- Allow custom `kid` to be set in JWS [#1239](https://github.com/iotaledger/identity.rs/pull/1239)
- Allow custom JWT claims for credentials [#1237](https://github.com/iotaledger/identity.rs/pull/1237)
- Improve `Proof` [#1209](https://github.com/iotaledger/identity.rs/pull/1209)
- Polish `identity_jose` [#1201](https://github.com/iotaledger/identity.rs/pull/1201)
- Add `resolve_multiple` to Resolver [#1189](https://github.com/iotaledger/identity.rs/pull/1189)
- Make JWT presentations generic [#1186](https://github.com/iotaledger/identity.rs/pull/1186)
- Support JWT presentations [#1175](https://github.com/iotaledger/identity.rs/pull/1175)
- Polish JWK thumbprint and document extension API [#1173](https://github.com/iotaledger/identity.rs/pull/1173)
- Stronghold Storage Implementation [#1157](https://github.com/iotaledger/identity.rs/pull/1157)
- Implement `KeyIdStorage` in Rust [#1134](https://github.com/iotaledger/identity.rs/pull/1134)
- Implement `JwkStorage` [#1116](https://github.com/iotaledger/identity.rs/pull/1133)
- Add Wasm Bindings for Domain Linkage [#1115](https://github.com/iotaledger/identity.rs/pull/1115)
- Introduce `IToCoreDocument` and document locks in the bindings [#1120](https://github.com/iotaledger/identity.rs/pull/1120)
- Add Support for Domain Linkage in Rust [#1094](https://github.com/iotaledger/identity.rs/pull/1094)
- Add JSON Object Signing capabilities [#1105](https://github.com/iotaledger/identity.rs/pull/1105)
- Make `StateMetadataDocument` public [#1085](https://github.com/iotaledger/identity.rs/pull/1085)
- Add v. credentials and presentations examples [#1070](https://github.com/iotaledger/identity.rs/pull/1070)
- Add revocation examples [#1076](https://github.com/iotaledger/identity.rs/pull/1076)
- Expose Controller and Governor Addresses in metadata [\#1023](https://github.com/iotaledger/identity.rs/pull/1023)
- Add Stardust Client Extension Trait [\#963](https://github.com/iotaledger/identity.rs/pull/963)
- Add StardustDID [\#949](https://github.com/iotaledger/identity.rs/pull/949)
- State metadata serialization for the stardust DID method [\#947](https://github.com/iotaledger/identity.rs/pull/947)
- Stardust DID Method Proof-of-Concept [\#940](https://github.com/iotaledger/identity.rs/pull/940)
- Implement the Identity Agent [\#322](https://github.com/iotaledger/identity.rs/pull/322)


### Patch

- Fix holder claim check in VP [#1236](https://github.com/iotaledger/identity.rs/pull/1236)
- Fix issuer claim check in VC [#1235](https://github.com/iotaledger/identity.rs/pull/1235)
- Feature-gate Domain Linkage [#1184](https://github.com/iotaledger/identity.rs/pull/1184)
- Update method spec and JWK method type [#1176](https://github.com/iotaledger/identity.rs/pull/1176)
- Replace `iota-client` with `iota-sdk` [#1161](https://github.com/iotaledger/identity.rs/pull/1161)
- Pin `form_urlencoded` to `1.1.0` [#1136](https://github.com/iotaledger/identity.rs/pull/1136)
- Remove legacy crates [#1080](https://github.com/iotaledger/identity.rs/pull/1080)
- Recommend unique `credentialStatus.id` in `RevocationBitmap2022` [\#1039](https://github.com/iotaledger/identity.rs/pull/1039)
- Pin agent dev-dependencies to crates versions [\#1029](https://github.com/iotaledger/identity.rs/pull/1029)
- Support case insensitive serialization of `RentStructure` [\#1012](https://github.com/iotaledger/identity.rs/pull/1012)
- Update stronghold to 0.6.4 [\#928](https://github.com/iotaledger/identity.rs/pull/928)

## [1.0.0-rc.1](https://github.com/iotaledger/identity.rs/tree/v1.0.0-rc.1) (2023-09-29)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.6.3...v1.0.0-rc.1)

This version introduces a new DID method targeting the IOTA UTXO ledger. This method works fundamentally differently from the previous method and introduces new capabilities to interact with Layer 1 assets like Native Tokens, NFTs and various Output types.

This version changes the credential and presentation format to JWT, as specified by the [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

Note: Identities and credentials created with the earlier versions cannot be resolved with this version of the library.

### Changed

- Add dedicated stronghold crate [#1243](https://github.com/iotaledger/identity.rs/pull/1243)
- Add dedicated EdDSA verifier crate [#1238](https://github.com/iotaledger/identity.rs/pull/1238)
- Use `VC Data Model v1.1` JWT encoding instead of `VC-JWT` [#1234](https://github.com/iotaledger/identity.rs/pull/1234)
- Remove `vp` and `vc` from JWT claims in JOSE [#1233](https://github.com/iotaledger/identity.rs/pull/1233)
- Mark error enums as non-exhaustive [#1227](https://github.com/iotaledger/identity.rs/pull/1227)
- Change `verifiable_credential` to type `Vec<CRED>` in `Presentation` [#1231](https://github.com/iotaledger/identity.rs/pull/1231)
- Bring JwkDocumentExt names in line with Wasm [#1233](https://github.com/iotaledger/identity.rs/pull/1223)
- Add lints for all crates [#1222](https://github.com/iotaledger/identity.rs/pull/1222)
- Bump `iota-sdk` and other dependencies [#1208](https://github.com/iotaledger/identity.rs/pull/1208)
- Polish `identity_credential` [#1205](https://github.com/iotaledger/identity.rs/pull/1205)
- Polish `identity_resolver` and`identity_storage` [#1204](https://github.com/iotaledger/identity.rs/pull/1204)
- Polish `identity_iota_core` [#1203](https://github.com/iotaledger/identity.rs/pull/1203)
- Rename `JwtPresentation` to `Presentation` [#1200](https://github.com/iotaledger/identity.rs/pull/1200)
- Polish `identity_document` [#1198](https://github.com/iotaledger/identity.rs/pull/1198)
- Polish `identity_did` & `identity_verification` [#1197](https://github.com/iotaledger/identity.rs/pull/1197)
- Polish `identity_core` [#1196](https://github.com/iotaledger/identity.rs/pull/1196)
- Remove identity-diff remains [#1195](https://github.com/iotaledger/identity.rs/pull/1195)
- Remove legacy signing and verification APIs [#1194](https://github.com/iotaledger/identity.rs/pull/1194)
- Remove old `Presentation` type [#1190](https://github.com/iotaledger/identity.rs/pull/1190)
- Remove reexported `Resolver` validation APIs [#1183](https://github.com/iotaledger/identity.rs/pull/1183)
- Use JWT credentials for Domain Linkage [#1180](https://github.com/iotaledger/identity.rs/pull/1180)
- Remove `identity_agent` & `identity_comm` [#1168](https://github.com/iotaledger/identity.rs/pull/1168)
- Remove `identity-diff` crate [#1167](https://github.com/iotaledger/identity.rs/pull/1167)
- JwkStorageDocument & JwtCredential validation [#1152](https://github.com/iotaledger/identity.rs/pull/1152)
- Adapt StorageError to be more generic [#1144](https://github.com/iotaledger/identity.rs/pull/1144)
- Add initial PublicKeyJwk support [#1143](https://github.com/iotaledger/identity.rs/pull/1143)
- Split JWS `Decoder` functionality [#1133](https://github.com/iotaledger/identity.rs/pull/1133)
- `CoreDocument` & `Service` and `VerificationMethod` are now in the `document` and `verification` modules respectively [#1104](https://github.com/iotaledger/identity.rs/pull/1104)
- Remove generics in CoreDocument, VerificationMethod, Service, DIDUrl and LinkedDomainService [#1110](https://github.com/iotaledger/identity.rs/pull/1110)
- Updated `iota-types` dependency to `1.0.0-rc.6` [#1121](https://github.com/iotaledger/identity.rs/pull/1121)
- Refactor `MethodType` to make it extensible [#1112](https://github.com/iotaledger/identity.rs/pull/1112)
- More identifier checks in `CoreDocument` [#1067](https://github.com/iotaledger/identity.rs/pull/1067)
- Update to `iota-client` 2.0.1-rc.4 and `iota-client-wasm` 0.5.0-alpha.6 [#1088](https://github.com/iotaledger/identity.rs/pull/1088)
- Update iota client 2.0.1 rc.3 [\#1062](https://github.com/iotaledger/identity.rs/pull/1062)
- Use Bech32-encoded state controller and governor addresses [\#1044](https://github.com/iotaledger/identity.rs/pull/1044)
- Expose iteration over verification relationship fields [\#1024](https://github.com/iotaledger/identity.rs/pull/1024)
- Remove `identity_agent` reexport [\#1031](https://github.com/iotaledger/identity.rs/pull/1031)
- Rename `MixedResolver` to `Resolver` in Wasm [\#1026](https://github.com/iotaledger/identity.rs/pull/1026)
- Add length prefix to DID Document payloads [\#1010](https://github.com/iotaledger/identity.rs/pull/1010)
- Feature-gate `Resolver` [\#1007](https://github.com/iotaledger/identity.rs/pull/1007)
- Rename `Stardust` types to `Iota` [\#1000](https://github.com/iotaledger/identity.rs/pull/1000)
- Change Stardust DID method to IOTA [\#982](https://github.com/iotaledger/identity.rs/pull/982)
- Add Wasm Stardust Client [\#975](https://github.com/iotaledger/identity.rs/pull/975)
- Generalized Resolver [\#970](https://github.com/iotaledger/identity.rs/pull/970)
- Change `Storage` to handle `CoreDID` [\#968](https://github.com/iotaledger/identity.rs/pull/968)
- Feature-gate `iota-client` dependency, integrate `StardustDID` [\#958](https://github.com/iotaledger/identity.rs/pull/958)
- Change `Storage` to store arbitrary blobs [\#953](https://github.com/iotaledger/identity.rs/pull/953)
- Add `StardustDocumentMetadata`, implement `StardustDocument` methods [\#951](https://github.com/iotaledger/identity.rs/pull/951)
- Fix stack overflow in `CoreDID` `PartialEq` impl [\#946](https://github.com/iotaledger/identity.rs/pull/946)
- Change `Service` `type` field to allow sets [\#944](https://github.com/iotaledger/identity.rs/pull/944)
- Generalise `CredentialValidator`, `PresentationValidator` to support arbitrary DID Documents [\#935](https://github.com/iotaledger/identity.rs/pull/935)

### Added

- Allow arbitrary JWS header parameters [#1245](https://github.com/iotaledger/identity.rs/pull/1245)
- Allow custom JWT claims for presentations [#1244](https://github.com/iotaledger/identity.rs/pull/1244)
- Allow custom `kid` to be set in JWS [#1239](https://github.com/iotaledger/identity.rs/pull/1239)
- Allow custom JWT claims for credentials [#1237](https://github.com/iotaledger/identity.rs/pull/1237)
- Improve `Proof` [#1209](https://github.com/iotaledger/identity.rs/pull/1209)
- Polish `identity_jose` [#1201](https://github.com/iotaledger/identity.rs/pull/1201)
- Add `resolve_multiple` to Resolver [#1189](https://github.com/iotaledger/identity.rs/pull/1189)
- Make JWT presentations generic [#1186](https://github.com/iotaledger/identity.rs/pull/1186)
- Support JWT presentations [#1175](https://github.com/iotaledger/identity.rs/pull/1175)
- Polish JWK thumbprint and document extension API [#1173](https://github.com/iotaledger/identity.rs/pull/1173)
- Stronghold Storage Implementation [#1157](https://github.com/iotaledger/identity.rs/pull/1157)
- Implement `KeyIdStorage` in Rust [#1134](https://github.com/iotaledger/identity.rs/pull/1134)
- Implement `JwkStorage` [#1116](https://github.com/iotaledger/identity.rs/pull/1133)
- Add Wasm Bindings for Domain Linkage [#1115](https://github.com/iotaledger/identity.rs/pull/1115)
- Introduce `IToCoreDocument` and document locks in the bindings [#1120](https://github.com/iotaledger/identity.rs/pull/1120)
- Add Support for Domain Linkage in Rust [#1094](https://github.com/iotaledger/identity.rs/pull/1094)
- Add JSON Object Signing capabilities [#1105](https://github.com/iotaledger/identity.rs/pull/1105)
- Make `StateMetadataDocument` public [#1085](https://github.com/iotaledger/identity.rs/pull/1085)
- Add v. credentials and presentations examples [#1070](https://github.com/iotaledger/identity.rs/pull/1070)
- Add revocation examples [#1076](https://github.com/iotaledger/identity.rs/pull/1076)
- Expose Controller and Governor Addresses in metadata [\#1023](https://github.com/iotaledger/identity.rs/pull/1023)
- Add Stardust Client Extension Trait [\#963](https://github.com/iotaledger/identity.rs/pull/963)
- Add StardustDID [\#949](https://github.com/iotaledger/identity.rs/pull/949)
- State metadata serialization for the stardust DID method [\#947](https://github.com/iotaledger/identity.rs/pull/947)
- Stardust DID Method Proof-of-Concept [\#940](https://github.com/iotaledger/identity.rs/pull/940)
- Implement the Identity Agent [\#322](https://github.com/iotaledger/identity.rs/pull/322)

### Patch

- Fix holder claim check in VP [#1236](https://github.com/iotaledger/identity.rs/pull/1236)
- Fix issuer claim check in VC [#1235](https://github.com/iotaledger/identity.rs/pull/1235)
- Feature-gate Domain Linkage [#1184](https://github.com/iotaledger/identity.rs/pull/1184)
- Update method spec and JWK method type [#1176](https://github.com/iotaledger/identity.rs/pull/1176)
- Replace `iota-client` with `iota-sdk` [#1161](https://github.com/iotaledger/identity.rs/pull/1161)
- Pin `form_urlencoded` to `1.1.0` [#1136](https://github.com/iotaledger/identity.rs/pull/1136)
- Remove legacy crates [#1080](https://github.com/iotaledger/identity.rs/pull/1080)
- Recommend unique `credentialStatus.id` in `RevocationBitmap2022` [\#1039](https://github.com/iotaledger/identity.rs/pull/1039)
- Pin agent dev-dependencies to crates versions [\#1029](https://github.com/iotaledger/identity.rs/pull/1029)
- Support case insensitive serialization of `RentStructure` [\#1012](https://github.com/iotaledger/identity.rs/pull/1012)
- Update stronghold to 0.6.4 [\#928](https://github.com/iotaledger/identity.rs/pull/928)

## [0.7.0-alpha.8](https://github.com/iotaledger/identity.rs/tree/v0.7.0-alpha.8) (2023-09-28)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.7.0-alpha.7...v0.7.0-alpha.8)


### Changed
- Add dedicated stronghold crate [#1243](https://github.com/iotaledger/identity.rs/pull/1243)
- Add dedicated EdDSA verifier crate [#1238](https://github.com/iotaledger/identity.rs/pull/1238)
- Use `VC Data Model v1.1` JWT encoding instead of `VC-JWT` [#1234](https://github.com/iotaledger/identity.rs/pull/1234)
- Remove `vp` and `vc` from JWT claims in JOSE [#1233](https://github.com/iotaledger/identity.rs/pull/1233)
- Mark error enums as non-exhaustive [#1227](https://github.com/iotaledger/identity.rs/pull/1227)
- Change `verifiable_credential` to type `Vec<CRED>` in `Presentation` [#1231](https://github.com/iotaledger/identity.rs/pull/1231)
- Bring JwkDocumentExt names in line with Wasm [#1233](https://github.com/iotaledger/identity.rs/pull/1223)
- Add lints for all crates [#1222](https://github.com/iotaledger/identity.rs/pull/1222)

### Added
- Allow arbitrary JWS header parameters [#1245](https://github.com/iotaledger/identity.rs/pull/1245)
- Allow custom JWT claims for presentations [#1244](https://github.com/iotaledger/identity.rs/pull/1244)
- Allow custom `kid` to be set in JWS [#1239](https://github.com/iotaledger/identity.rs/pull/1239)
- Allow custom JWT claims for credentials [#1237](https://github.com/iotaledger/identity.rs/pull/1237)

### Patch
- Fix holder claim check in VP [#1236](https://github.com/iotaledger/identity.rs/pull/1236)
- Fix issuer claim check in VC [#1235](https://github.com/iotaledger/identity.rs/pull/1235)

## [v0.7.0-alpha.7](https://github.com/iotaledger/identity.rs/tree/v0.7.0-alpha.7) (2023-08-15)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.7.0-alpha.6...v0.7.0-alpha.7)

### Added

- Improve `Proof` [#1209](https://github.com/iotaledger/identity.rs/pull/1209)
- Polish `identity_jose` [#1201](https://github.com/iotaledger/identity.rs/pull/1201)
- Add `resolve_multiple` to Resolver [#1189](https://github.com/iotaledger/identity.rs/pull/1189)
- Make JWT presentations generic [#1186](https://github.com/iotaledger/identity.rs/pull/1186)
- Support JWT presentations [#1175](https://github.com/iotaledger/identity.rs/pull/1175)
- Polish JWK thumbprint and document extension API [#1173](https://github.com/iotaledger/identity.rs/pull/1173)
- Stronghold Storage Implementation [#1157](https://github.com/iotaledger/identity.rs/pull/1157)
- Implement `KeyIdStorage` in Rust [#1134](https://github.com/iotaledger/identity.rs/pull/1134)

### Changed

- Bump `iota-sdk` and other dependencies [#1208](https://github.com/iotaledger/identity.rs/pull/1208)
- Polish `identity_credential` [#1205](https://github.com/iotaledger/identity.rs/pull/1205)
- Polish `identity_resolver` and`identity_storage` [#1204](https://github.com/iotaledger/identity.rs/pull/1204)
- Polish `identity_iota_core` [#1203](https://github.com/iotaledger/identity.rs/pull/1203)
- Rename `JwtPresentation` to `Presentation` [#1200](https://github.com/iotaledger/identity.rs/pull/1200)
- Polish `identity_document` [#1198](https://github.com/iotaledger/identity.rs/pull/1198)
- Polish `identity_did` & `identity_verification` [#1197](https://github.com/iotaledger/identity.rs/pull/1197)
- Polish `identity_core` [#1196](https://github.com/iotaledger/identity.rs/pull/1196)
- Remove identity-diff remains [#1195](https://github.com/iotaledger/identity.rs/pull/1195)
- Remove legacy signing and verification APIs [#1194](https://github.com/iotaledger/identity.rs/pull/1194)
- Remove old `Presentation` type [#1190](https://github.com/iotaledger/identity.rs/pull/1190)
- Remove reexported `Resolver` validation APIs [#1183](https://github.com/iotaledger/identity.rs/pull/1183)
- Use JWT credentials for Domain Linkage [#1180](https://github.com/iotaledger/identity.rs/pull/1180)
- Remove `identity_agent` & `identity_comm` [#1168](https://github.com/iotaledger/identity.rs/pull/1168)
- Remove `identity-diff` crate [#1167](https://github.com/iotaledger/identity.rs/pull/1167)
- JwkStorageDocument & JwtCredential validation [#1152](https://github.com/iotaledger/identity.rs/pull/1152)
- Adapt StorageError to be more generic [#1144](https://github.com/iotaledger/identity.rs/pull/1144)
- Add initial PublicKeyJwk support [#1143](https://github.com/iotaledger/identity.rs/pull/1143)
- Split JWS `Decoder` functionality [#1133](https://github.com/iotaledger/identity.rs/pull/1133)

### Patch

- Feature-gate Domain Linkage [#1184](https://github.com/iotaledger/identity.rs/pull/1184)
- Update method spec and JWK method type [#1176](https://github.com/iotaledger/identity.rs/pull/1176)
- Replace `iota-client` with `iota-sdk` [#1161](https://github.com/iotaledger/identity.rs/pull/1161)
- Pin `form_urlencoded` to `1.1.0` [#1136](https://github.com/iotaledger/identity.rs/pull/1136)

## [v0.7.0-alpha.6](https://github.com/iotaledger/identity.rs/tree/v0.7.0-alpha.6) (2023-03-03)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.7.0-alpha.5...v0.7.0-alpha.6)

### Added 
- Implement `JwkStorage` [#1116](https://github.com/iotaledger/identity.rs/pull/1133)
- Add Wasm Bindings for Domain Linkage [#1115](https://github.com/iotaledger/identity.rs/pull/1115)
- Introduce `IToCoreDocument` and document locks in the bindings [#1120](https://github.com/iotaledger/identity.rs/pull/1120)

### Patch

- Pin `form_urlencoded` to `1.1.0` [#1136](https://github.com/iotaledger/identity.rs/pull/1136)

## [v0.7.0-alpha.5](https://github.com/iotaledger/identity.rs/tree/v0.7.0-alpha.5) (2023-02-15)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.7.0-alpha.4...v0.7.0-alpha.5)

### Added
- Add Support for Domain Linkage in Rust [#1094](https://github.com/iotaledger/identity.rs/pull/1094)
- Add JSON Object Signing capabilities [#1105](https://github.com/iotaledger/identity.rs/pull/1105)

### Changed 
- `CoreDocument` & `Service` and `VerificationMethod` are now in the `document` and `verification` modules respectively [#1104](https://github.com/iotaledger/identity.rs/pull/1104)
-  Remove generics in CoreDocument, VerificationMethod, Service, DIDUrl and LinkedDomainService [#1110](https://github.com/iotaledger/identity.rs/pull/1110)
- Updated `iota-types` dependency to `1.0.0-rc.6` [#1121](https://github.com/iotaledger/identity.rs/pull/1121)
- Refactor `MethodType` to make it extensible [#1112](https://github.com/iotaledger/identity.rs/pull/1112)

## [v0.7.0-alpha.4](https://github.com/iotaledger/identity.rs/tree/v0.7.0-alpha.4) (2022-11-24)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.7.0-alpha.3...v0.7.0-alpha.4)

### Added

- Make `StateMetadataDocument` public [#1085](https://github.com/iotaledger/identity.rs/pull/1085)
- Add v. credentials and presentations examples [#1070](https://github.com/iotaledger/identity.rs/pull/1070)
- Add revocation examples [#1076](https://github.com/iotaledger/identity.rs/pull/1076)

### Changed 
- More identifier checks in `CoreDocument` [#1067](https://github.com/iotaledger/identity.rs/pull/1067)
- Update to `iota-client` 2.0.1-rc.4 and `iota-client-wasm` 0.5.0-alpha.6 [#1088](https://github.com/iotaledger/identity.rs/pull/1088)
### Patch

- Remove legacy crates [#1080](https://github.com/iotaledger/identity.rs/pull/1080)

## [v0.7.0-alpha.3](https://github.com/iotaledger/identity.rs/tree/v0.7.0-alpha.3) (2022-09-30)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.7.0-alpha.2...v0.7.0-alpha.3)
 
### Changed

- Update iota client 2.0.1 rc.3 [\#1062](https://github.com/iotaledger/identity.rs/pull/1062)
## [v0.7.0-alpha.2](https://github.com/iotaledger/identity.rs/tree/v0.7.0-alpha.2) (2022-09-30)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.7.0-alpha.1...v0.7.0-alpha.2)

### Changed

- Use Bech32-encoded state controller and governor addresses [\#1044](https://github.com/iotaledger/identity.rs/pull/1044)
- Expose iteration over verification relationship fields [\#1024](https://github.com/iotaledger/identity.rs/pull/1024)

### Added

- Expose Controller and Governor Addresses in metadata [\#1023](https://github.com/iotaledger/identity.rs/pull/1023)

### Patch

- Recommend unique `credentialStatus.id` in `RevocationBitmap2022` [\#1039](https://github.com/iotaledger/identity.rs/pull/1039)

## [v0.7.0-alpha.1](https://github.com/iotaledger/identity.rs/tree/v0.7.0-alpha.1) (2022-09-19)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.6.0...v0.7.0-alpha.1)
 
This version introduces a new DID method targeting the IOTA UTXO ledger. This method works fundamentally differently from the previous method and introduces new capabilities to interact with Layer 1 entities like native tokens, NFTs and smart contracts. 

 This is an early alpha release, so there may be breaking changes in upcoming versions that invalidate DIDs created with this version. The method at this point is only intended for experimentation. 

 Note: Identities created with the earlier versions cannot be resolved with this version of the library. 

### Changed

- Remove `identity_agent` reexport [\#1031](https://github.com/iotaledger/identity.rs/pull/1031)
- Rename `MixedResolver` to `Resolver` in Wasm [\#1026](https://github.com/iotaledger/identity.rs/pull/1026)
- Add length prefix to DID Document payloads [\#1010](https://github.com/iotaledger/identity.rs/pull/1010)
- Feature-gate `Resolver` [\#1007](https://github.com/iotaledger/identity.rs/pull/1007)
- Rename `Stardust` types to `Iota` [\#1000](https://github.com/iotaledger/identity.rs/pull/1000)
- Change Stardust DID method to IOTA [\#982](https://github.com/iotaledger/identity.rs/pull/982)
- Add Wasm Stardust Client [\#975](https://github.com/iotaledger/identity.rs/pull/975)
- Generalized Resolver [\#970](https://github.com/iotaledger/identity.rs/pull/970)
- Change `Storage` to handle `CoreDID` [\#968](https://github.com/iotaledger/identity.rs/pull/968)
- Feature-gate `iota-client` dependency, integrate `StardustDID` [\#958](https://github.com/iotaledger/identity.rs/pull/958)
- Change `Storage` to store arbitrary blobs [\#953](https://github.com/iotaledger/identity.rs/pull/953)
- Add `StardustDocumentMetadata`, implement `StardustDocument` methods [\#951](https://github.com/iotaledger/identity.rs/pull/951)
- Fix stack overflow in `CoreDID` `PartialEq` impl [\#946](https://github.com/iotaledger/identity.rs/pull/946)
- Change `Service` `type` field to allow sets [\#944](https://github.com/iotaledger/identity.rs/pull/944)
- Generalise `CredentialValidator`, `PresentationValidator` to support arbitrary DID Documents [\#935](https://github.com/iotaledger/identity.rs/pull/935)

### Added

- Add Stardust Client Extension Trait [\#963](https://github.com/iotaledger/identity.rs/pull/963)
- Add StardustDID [\#949](https://github.com/iotaledger/identity.rs/pull/949)
- State metadata serialization for the stardust DID method [\#947](https://github.com/iotaledger/identity.rs/pull/947)
- Stardust DID Method Proof-of-Concept [\#940](https://github.com/iotaledger/identity.rs/pull/940)
- Implement the Identity Agent [\#322](https://github.com/iotaledger/identity.rs/pull/322)

### Patch

- Pin agent dev-dependencies to crates versions [\#1029](https://github.com/iotaledger/identity.rs/pull/1029)
- Support case insensitive serialization of `RentStructure` [\#1012](https://github.com/iotaledger/identity.rs/pull/1012)
- Update stronghold to 0.6.4 [\#928](https://github.com/iotaledger/identity.rs/pull/928)

## [v0.6.0](https://github.com/iotaledger/identity.rs/tree/v0.6.0) (2022-06-15)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.5.0...v0.6.0)
 
The main feature of this release is the addition of the `RevocationBitmap2022` specification, offering efficient credential revocation on-Tangle. This is the replacement for the `MerkleKeyCollection` removed in v0.5.0, which offered similar functionality but fundamentally failed to scale beyond a few thousand revocations. 

 Other changes include encryption support using Elliptic Curve Diffie-Hellman (ECDH) and quality of life improvements for verifiable credential and presentation types in the Wasm bindings. 

 DID Documents created with v0.5.0 remain compatible with v0.6.0. This will be the last major release prior to changes for the Stardust update. 



### Changed

- Rename crates to use underscores [\#895](https://github.com/iotaledger/identity.rs/pull/895)
- Change `remove_service` to return boolean [\#877](https://github.com/iotaledger/identity.rs/pull/877)
- Change `DIDUrl::join` to borrow self [\#871](https://github.com/iotaledger/identity.rs/pull/871)
- Add `BaseEncoding` to replace `encode_b58`, `decode_b58`, `encode_multibase`, `decode_multibase` [\#870](https://github.com/iotaledger/identity.rs/pull/870)
- Add `RevocationBitmap2022`, bump MSRV to 1.60 [\#861](https://github.com/iotaledger/identity.rs/pull/861)
- Add Wasm `Credential` and `Presentation` field getters and constructors [\#815](https://github.com/iotaledger/identity.rs/pull/815)
- Add Diffie-Hellman key exchange for encryption to `Account` [\#809](https://github.com/iotaledger/identity.rs/pull/809)

### Added

- Implement `ECDH-ES+A256KW` for `Storage` encryption [\#867](https://github.com/iotaledger/identity.rs/pull/867)
- Add Client option for retry publishing behaviour [\#820](https://github.com/iotaledger/identity.rs/pull/820)
- Implement `Storage` test suite [\#791](https://github.com/iotaledger/identity.rs/pull/791)

### Patch

- Unpin iota-crypto version [\#834](https://github.com/iotaledger/identity.rs/pull/834)

### Removed

- Remove unused resolution code [\#862](https://github.com/iotaledger/identity.rs/pull/862)

## [v0.5.0](https://github.com/iotaledger/identity.rs/tree/v0.5.0) (2022-03-31)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.4.0...v0.5.0)
 
This release introduces multiple breaking changes to the structure of IOTA DID Documents and their Tangle messages, rendering any identity created with a prior version incompatible and unresolvable. A versioning system has been introduced so any new identities should hopefully be forward compatible with any future breaking changes to the message structure. 

 The main feature of this release is the introduction of WebAssembly (Wasm) bindings for the high-level `Account` API for Javascript/Typescript in both Node.js and the browser. This includes preliminary Stronghold storage bindings but only for Node.js, as it was determined that compiling Stronghold to Wasm for the browser would not be sufficiently secure. Stronghold offers best-effort secure software storage for cryptographic keys, written in Rust. To use the Stronghold storage package install `@iota/identity-stronghold-nodejs` and follow the instructions of the package [README](https://github.com/iotaledger/identity.rs/tree/dev/bindings/stronghold-nodejs). 

 Note that all features related to diff chain updates are now marked as deprecated. Diff chains are a useful optimisation when publishing many updates to a DID Document. However, their design may be incompatible with upcoming changes to the IOTA network and should be considered unstable. 

 Another major change is the removal of the `MerkleKeyCollection` verification method type, which provided a compact representation for issuing and revoking Verifiable Credentials with multiple cryptographic keys. The `MerkleKeyCollection` suffered from disadvantages which limited scalability when managing more than a few thousand keys. While these disadvantages could be mitigated somewhat, the decision was made to replace it with one or more alternatives not affected by its fundamental limitations, upcoming in the next major release.

### Changed

- Add Wasm `Proof`, rename `Signature` structs to `Proof` [\#776](https://github.com/iotaledger/identity.rs/pull/776)
- Replace `MethodSecret` with `MethodContent` enum [\#764](https://github.com/iotaledger/identity.rs/pull/764)
- Change document metadata `created`, `updated` to be optional [\#753](https://github.com/iotaledger/identity.rs/pull/753)
- Refactor Storage Signature [\#738](https://github.com/iotaledger/identity.rs/pull/738)
- Add X25519 key and verification method support [\#735](https://github.com/iotaledger/identity.rs/pull/735)
- Refactor `KeyLocation` [\#729](https://github.com/iotaledger/identity.rs/pull/729)
- Move DID Document proof outside metadata [\#728](https://github.com/iotaledger/identity.rs/pull/728)
- Combine resolve\_method functions [\#709](https://github.com/iotaledger/identity.rs/pull/709)
- Add separate `identity-iota-core`, `identity-account-storage` crates [\#693](https://github.com/iotaledger/identity.rs/pull/693)
- Change `IotaDocument::verify_document` from a static function to a method [\#675](https://github.com/iotaledger/identity.rs/pull/675)
- Make Wasm support dependent on `target_arch` rather than feature [\#666](https://github.com/iotaledger/identity.rs/pull/666)
- Refactor `CoreDocument`, `VerificationMethod`, `Service` to use generic DID [\#655](https://github.com/iotaledger/identity.rs/pull/655)
- Remove unused `Account` milestone option [\#645](https://github.com/iotaledger/identity.rs/pull/645)
- Change document controller type to `OneOrSet` [\#638](https://github.com/iotaledger/identity.rs/pull/638)
- Rename `MethodQuery` to `DIDUrlQuery`, move `OrderedSet`, `KeyComparable` [\#634](https://github.com/iotaledger/identity.rs/pull/634)
- Change `also_known_as` type to `OrderedSet` [\#632](https://github.com/iotaledger/identity.rs/pull/632)
- Move verification functionality from `DocumentVerifier` to  `CoreDocument`  [\#606](https://github.com/iotaledger/identity.rs/pull/606)
- Fix dependent diff updates being rejected [\#605](https://github.com/iotaledger/identity.rs/pull/605)
- Change `Account::state` visibility to `pub(crate)` [\#604](https://github.com/iotaledger/identity.rs/pull/604)
- Overhaul `CredentialValidator`, add `PresentationValidator` [\#599](https://github.com/iotaledger/identity.rs/pull/599)
- Remove JSON string escaping in diff messages [\#598](https://github.com/iotaledger/identity.rs/pull/598)
- Replace `ClientMap` with new `Resolver` [\#594](https://github.com/iotaledger/identity.rs/pull/594)
- Replace `ClientMap` with `Client` in `Account` [\#582](https://github.com/iotaledger/identity.rs/pull/582)
- Add signature `created`, `expires`, `challenge`, `domain`, `purpose` [\#548](https://github.com/iotaledger/identity.rs/pull/548)
- Refactor document metadata [\#540](https://github.com/iotaledger/identity.rs/pull/540)
- Replace `chrono` with `time` [\#529](https://github.com/iotaledger/identity.rs/pull/529)
- Enable access to the low-level API from the `Account` [\#522](https://github.com/iotaledger/identity.rs/pull/522)
- Update to `rsa` 0.5 in libjose [\#517](https://github.com/iotaledger/identity.rs/pull/517)
- Rename `DocumentDiff` to `DiffMessage` [\#511](https://github.com/iotaledger/identity.rs/pull/511)
- Deterministic ordering of competing messages [\#506](https://github.com/iotaledger/identity.rs/pull/506)
- Check for existence & duplication of methods in `CoreDocument` [\#504](https://github.com/iotaledger/identity.rs/pull/504)
- Move `dropsave` from `Account` to `Stronghold` [\#500](https://github.com/iotaledger/identity.rs/pull/500)
- Add `ExplorerUrl` to replace `Network` explorer methods [\#496](https://github.com/iotaledger/identity.rs/pull/496)
- Update `ServiceEndpoint` to support sets and maps [\#485](https://github.com/iotaledger/identity.rs/pull/485)
- Enable deep equality in `OrderedSet` [\#481](https://github.com/iotaledger/identity.rs/pull/481)
- Add message compression and versioning [\#466](https://github.com/iotaledger/identity.rs/pull/466)
- Update document signing key constraints and methods [\#458](https://github.com/iotaledger/identity.rs/pull/458)
- Refactor the `Account`: internal state, one identity [\#453](https://github.com/iotaledger/identity.rs/pull/453)

### Added

- Expose Ed25519, X25519 length constants [\#772](https://github.com/iotaledger/identity.rs/pull/772)
- Generify `Account::client` over `Rc`, `Arc` [\#707](https://github.com/iotaledger/identity.rs/pull/707)
- Update Stronghold [\#691](https://github.com/iotaledger/identity.rs/pull/691)
- Add `Duration` for `Timestamp` arithmetic [\#684](https://github.com/iotaledger/identity.rs/pull/684)
- Add `Client` fallback to local PoW option [\#682](https://github.com/iotaledger/identity.rs/pull/682)
- Set `controller`, `alsoKnownAs` fields from Account [\#658](https://github.com/iotaledger/identity.rs/pull/658)
- Implement `FromIterator` for `OneOrMany` [\#602](https://github.com/iotaledger/identity.rs/pull/602)
- Add account synchronization method [\#544](https://github.com/iotaledger/identity.rs/pull/544)
- Filter out DiffMessages updating signing methods [\#519](https://github.com/iotaledger/identity.rs/pull/519)
- Add publish with retry method [\#455](https://github.com/iotaledger/identity.rs/pull/455)

### Patch

- Fix panic when parsing an `IotaDID` with more than 2 method id segments [\#758](https://github.com/iotaledger/identity.rs/pull/758)
- Update iota.rs to include timeout bugfix [\#712](https://github.com/iotaledger/identity.rs/pull/712)
- Support verification methods with the same fragment [\#623](https://github.com/iotaledger/identity.rs/pull/623)
- Fix diff properties \(de\)serialization [\#611](https://github.com/iotaledger/identity.rs/pull/611)
- Enable local proof-of-work fallback [\#579](https://github.com/iotaledger/identity.rs/pull/579)
- Add `identity-diff` derive feature gate [\#516](https://github.com/iotaledger/identity.rs/pull/516)
- Improve client error messages [\#512](https://github.com/iotaledger/identity.rs/pull/512)
- Make `create_signature` and `sign` async for `RemoteEd25519` [\#491](https://github.com/iotaledger/identity.rs/pull/491)
- Fix credential validation failing for documents with diff updates [\#490](https://github.com/iotaledger/identity.rs/pull/490)
- Upgrade to the Rust 2021 edition [\#449](https://github.com/iotaledger/identity.rs/pull/449)

### Deprecated

- Deprecate diff chain features [\#759](https://github.com/iotaledger/identity.rs/pull/759)

### Removed

- Remove `AccountStorage` [\#774](https://github.com/iotaledger/identity.rs/pull/774)
- Remove `MerkleKeyCollection` [\#755](https://github.com/iotaledger/identity.rs/pull/755)
- Remove `Storage::set_password` [\#733](https://github.com/iotaledger/identity.rs/pull/733)
- Remove `publicKeyJwk` [\#732](https://github.com/iotaledger/identity.rs/pull/732)
- Remove `DIDLease` account feature [\#664](https://github.com/iotaledger/identity.rs/pull/664)

## [v0.4.0](https://github.com/iotaledger/identity.rs/tree/v0.4.0) (2021-11-01)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.3.0...v0.4.0)

## [v0.3.0](https://github.com/iotaledger/identity.rs/tree/v0.3.0) (2021-05-10)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.2.0...v0.3.0)
 
This release introduces the high-level `Account` API for creating and managing IOTA identities.

## [v0.2.0](https://github.com/iotaledger/identity.rs/tree/v0.2.0) (2021-02-18)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.1.0...v0.2.0)

## [v0.1.0](https://github.com/iotaledger/identity.rs/tree/v0.1.0) (2020-11-12)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/360bf5ce64a7f418249cdeadccb22b9aea7daeb6...v0.1.0)



\* *This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)*
