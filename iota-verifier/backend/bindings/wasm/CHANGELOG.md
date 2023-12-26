# Changelog

## [wasm-v1.0.0](https://github.com/iotaledger/identity.rs/tree/wasm-v1.0.0) (2023-11-02)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.6.0...wasm-v1.0.0)

This version introduces a new DID method targeting the IOTA UTXO ledger. This method works fundamentally differently from the previous method and introduces new capabilities to interact with Layer 1 assets like Native Tokens, NFTs and various Output types.

This version changes the credential and presentation format to JWT, as specified by the [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

Note: Identities and credentials created with the earlier versions cannot be resolved with this version of the library.

### Changed 

- Add dedicated EdDSA verifier crate [#1238](https://github.com/iotaledger/identity.rs/pull/1238)
- Use `VC Data Model v1.1` JWT encoding instead of `VC-JWT` [#1234](https://github.com/iotaledger/identity.rs/pull/1234)
- Change `verifiable_credential` to type `Vec<CRED>` in `Presentation` [#1231](https://github.com/iotaledger/identity.rs/pull/1231)
- Polish Wasm bindings [#1206](https://github.com/iotaledger/identity.rs/pull/1206)
- Polish `identity_credential` [#1205](https://github.com/iotaledger/identity.rs/pull/1205)
- Polish `identity_iota_core` [#1203](https://github.com/iotaledger/identity.rs/pull/1203)
- Upgrade `client-wasm` to `sdk-wasm` [#1202](https://github.com/iotaledger/identity.rs/pull/1202)
- Rename `JwtPresentation` to `Presentation` [#1200](https://github.com/iotaledger/identity.rs/pull/1200)
- Remove legacy signing and verification APIs [#1194](https://github.com/iotaledger/identity.rs/pull/1194)
- Remove old `Presentation` type [#1190](https://github.com/iotaledger/identity.rs/pull/1190)
- Remove reexported `Resolver` validation APIs [#1183](https://github.com/iotaledger/identity.rs/pull/1183)
- Use JWT credentials for Domain Linkage [#1180](https://github.com/iotaledger/identity.rs/pull/1180)
- Remove stronghold nodejs bindings [#1178](https://github.com/iotaledger/identity.rs/pull/1178)
- JwkStorageDocument & JwtCredential validation [#1152](https://github.com/iotaledger/identity.rs/pull/1152)
- Add initial PublicKeyJwk support [#1143](https://github.com/iotaledger/identity.rs/pull/1143)
- Refactor `MethodType` to make it extensible [#1112](https://github.com/iotaledger/identity.rs/pull/1112)
- Remove generics in `CoreDocument`, `VerificationMethod`, `Service`, `DIDUrl` and `LinkedDomainService` [#1110](https://github.com/iotaledger/identity.rs/pull/1110)
- Use official client-wasm dependency in examples [#1097](https://github.com/iotaledger/identity.rs/pull/1097)
- More identifier checks in `CoreDocument` [#1067](https://github.com/iotaledger/identity.rs/pull/1067)
- Update to `iota-client` 2.0.1-rc.4 and `iota-client-wasm` 0.5.0-alpha.6 [#1088](https://github.com/iotaledger/identity.rs/pull/1088)
- Use Bech32-encoded state controller and governor addresses [\#1044](https://github.com/iotaledger/identity.rs/pull/1044)
- Expose iteration over verification relationship fields [\#1024](https://github.com/iotaledger/identity.rs/pull/1024)
- Chore/rename mixed resolver [\#1026](https://github.com/iotaledger/identity.rs/pull/1026)
- Add length prefix to DID Document payloads [\#1010](https://github.com/iotaledger/identity.rs/pull/1010)
- Update Wasm credential, presentation validators for Stardust [\#1004](https://github.com/iotaledger/identity.rs/pull/1004)
- Rename `Stardust` types to `Iota` [\#1000](https://github.com/iotaledger/identity.rs/pull/1000)
- Change Stardust DID method to IOTA [\#982](https://github.com/iotaledger/identity.rs/pull/982)
- Add Wasm Stardust Client [\#975](https://github.com/iotaledger/identity.rs/pull/975)
- Generalized Resolver [\#970](https://github.com/iotaledger/identity.rs/pull/970)
- Change `Storage` to handle `CoreDID` [\#968](https://github.com/iotaledger/identity.rs/pull/968)
- Change `Storage` to store arbitrary blobs [\#953](https://github.com/iotaledger/identity.rs/pull/953)
- Change `Service` `type` field to allow sets [\#944](https://github.com/iotaledger/identity.rs/pull/944)
- Generalise `CredentialValidator`, `PresentationValidator` to support arbitrary DID Documents [\#935](https://github.com/iotaledger/identity.rs/pull/935)

### Added

- Allow arbitrary JWS header parameters [#1245](https://github.com/iotaledger/identity.rs/pull/1245)
- Allow custom JWT claims for presentations [#1244](https://github.com/iotaledger/identity.rs/pull/1244)
- Allow custom `kid` to be set in JWS [#1239](https://github.com/iotaledger/identity.rs/pull/1239)
- Allow custom JWT claims for credentials [#1237](https://github.com/iotaledger/identity.rs/pull/1237)
- Improve `Proof` [#1209](https://github.com/iotaledger/identity.rs/pull/1209)
- Add `resolve_multiple` to Resolver [#1189](https://github.com/iotaledger/identity.rs/pull/1189)
- Move jwk_storage and key_id_storage to Wasm lib [#1181](https://github.com/iotaledger/identity.rs/pull/1181)
- Wasm Bindings for JWT Presentations [#1179](https://github.com/iotaledger/identity.rs/pull/1179)
- Polish JWK thumbprint and document extension API [#1173](https://github.com/iotaledger/identity.rs/pull/1173)
- Wasm bindings for `KeyIdStorage` [#1147](https://github.com/iotaledger/identity.rs/pull/1147)
- Introduce `IToCoreDocument` and document locks in the bindings [#1120](https://github.com/iotaledger/identity.rs/pull/1120)
- Add Wasm Bindings for Domain Linkage [#1115](https://github.com/iotaledger/identity.rs/pull/1115)
- Add wasm credentials and presentations examples [#1075](https://github.com/iotaledger/identity.rs/pull/1075)
- Add revocation examples [#1076](https://github.com/iotaledger/identity.rs/pull/1076)
- Add `IotaDID.fromAliasId` to the Wasm bindings [\#1048](https://github.com/iotaledger/identity.rs/pull/1048)
- Expose Controller and Governor Addresses in metadata [\#1023](https://github.com/iotaledger/identity.rs/pull/1023)
- Add Wasm bindings for `CoreDocument` [\#994](https://github.com/iotaledger/identity.rs/pull/994)
- Add initial Wasm Stardust bindings [\#967](https://github.com/iotaledger/identity.rs/pull/967)

### Patch

- Fix wasm panic caused by a race condition in `IotaDocument` and `CoreDocument` [#1258](https://github.com/iotaledger/identity.rs/pull/1258)
- Fix holder claim check in VP [#1236](https://github.com/iotaledger/identity.rs/pull/1236)
- Fix issuer claim check in VC [#1235](https://github.com/iotaledger/identity.rs/pull/1235)
- Fix clippy's issue `uninlined-format-args` [#1109](https://github.com/iotaledger/identity.rs/pull/1109)
- Update iota.js peer dependency [#1107](https://github.com/iotaledger/identity.rs/pull/1107)
- Fix unresolved import in TS artifacts [\#1066](https://github.com/iotaledger/identity.rs/pull/1066)
- Recommend unique `credentialStatus.id` in `RevocationBitmap2022` [\#1039](https://github.com/iotaledger/identity.rs/pull/1039)
- Support case insensitive serialization of `RentStructure` [\#1012](https://github.com/iotaledger/identity.rs/pull/1012)
- Fix broken wasm bindings compilation [\#995](https://github.com/iotaledger/identity.rs/pull/995)
- Fix DID TypeScript references [\#977](https://github.com/iotaledger/identity.rs/pull/977)

## [wasm-v1.0.0-rc.1](https://github.com/iotaledger/identity.rs/tree/wasm-v1.0.0-rc.1) (2023-09-29)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.6.0...wasm-v1.0.0-rc.1)

This version introduces a new DID method targeting the IOTA UTXO ledger. This method works fundamentally differently from the previous method and introduces new capabilities to interact with Layer 1 assets like Native Tokens, NFTs and various Output types.

This version changes the credential and presentation format to JWT, as specified by the [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

Note: Identities and credentials created with the earlier versions cannot be resolved with this version of the library.

### Changed 

- Add dedicated EdDSA verifier crate [#1238](https://github.com/iotaledger/identity.rs/pull/1238)
- Use `VC Data Model v1.1` JWT encoding instead of `VC-JWT` [#1234](https://github.com/iotaledger/identity.rs/pull/1234)
- Change `verifiable_credential` to type `Vec<CRED>` in `Presentation` [#1231](https://github.com/iotaledger/identity.rs/pull/1231)
- Polish Wasm bindings [#1206](https://github.com/iotaledger/identity.rs/pull/1206)
- Polish `identity_credential` [#1205](https://github.com/iotaledger/identity.rs/pull/1205)
- Polish `identity_iota_core` [#1203](https://github.com/iotaledger/identity.rs/pull/1203)
- Upgrade `client-wasm` to `sdk-wasm` [#1202](https://github.com/iotaledger/identity.rs/pull/1202)
- Rename `JwtPresentation` to `Presentation` [#1200](https://github.com/iotaledger/identity.rs/pull/1200)
- Remove legacy signing and verification APIs [#1194](https://github.com/iotaledger/identity.rs/pull/1194)
- Remove old `Presentation` type [#1190](https://github.com/iotaledger/identity.rs/pull/1190)
- Remove reexported `Resolver` validation APIs [#1183](https://github.com/iotaledger/identity.rs/pull/1183)
- Use JWT credentials for Domain Linkage [#1180](https://github.com/iotaledger/identity.rs/pull/1180)
- Remove stronghold nodejs bindings [#1178](https://github.com/iotaledger/identity.rs/pull/1178)
- JwkStorageDocument & JwtCredential validation [#1152](https://github.com/iotaledger/identity.rs/pull/1152)
- Add initial PublicKeyJwk support [#1143](https://github.com/iotaledger/identity.rs/pull/1143)
- Refactor `MethodType` to make it extensible [#1112](https://github.com/iotaledger/identity.rs/pull/1112)
- Remove generics in `CoreDocument`, `VerificationMethod`, `Service`, `DIDUrl` and `LinkedDomainService` [#1110](https://github.com/iotaledger/identity.rs/pull/1110)
- Use official client-wasm dependency in examples [#1097](https://github.com/iotaledger/identity.rs/pull/1097)
- More identifier checks in `CoreDocument` [#1067](https://github.com/iotaledger/identity.rs/pull/1067)
- Update to `iota-client` 2.0.1-rc.4 and `iota-client-wasm` 0.5.0-alpha.6 [#1088](https://github.com/iotaledger/identity.rs/pull/1088)
- Use Bech32-encoded state controller and governor addresses [\#1044](https://github.com/iotaledger/identity.rs/pull/1044)
- Expose iteration over verification relationship fields [\#1024](https://github.com/iotaledger/identity.rs/pull/1024)
- Chore/rename mixed resolver [\#1026](https://github.com/iotaledger/identity.rs/pull/1026)
- Add length prefix to DID Document payloads [\#1010](https://github.com/iotaledger/identity.rs/pull/1010)
- Update Wasm credential, presentation validators for Stardust [\#1004](https://github.com/iotaledger/identity.rs/pull/1004)
- Rename `Stardust` types to `Iota` [\#1000](https://github.com/iotaledger/identity.rs/pull/1000)
- Change Stardust DID method to IOTA [\#982](https://github.com/iotaledger/identity.rs/pull/982)
- Add Wasm Stardust Client [\#975](https://github.com/iotaledger/identity.rs/pull/975)
- Generalized Resolver [\#970](https://github.com/iotaledger/identity.rs/pull/970)
- Change `Storage` to handle `CoreDID` [\#968](https://github.com/iotaledger/identity.rs/pull/968)
- Change `Storage` to store arbitrary blobs [\#953](https://github.com/iotaledger/identity.rs/pull/953)
- Change `Service` `type` field to allow sets [\#944](https://github.com/iotaledger/identity.rs/pull/944)
- Generalise `CredentialValidator`, `PresentationValidator` to support arbitrary DID Documents [\#935](https://github.com/iotaledger/identity.rs/pull/935)

### Added

- Allow arbitrary JWS header parameters [#1245](https://github.com/iotaledger/identity.rs/pull/1245)
- Allow custom JWT claims for presentations [#1244](https://github.com/iotaledger/identity.rs/pull/1244)
- Allow custom `kid` to be set in JWS [#1239](https://github.com/iotaledger/identity.rs/pull/1239)
- Allow custom JWT claims for credentials [#1237](https://github.com/iotaledger/identity.rs/pull/1237)
- Improve `Proof` [#1209](https://github.com/iotaledger/identity.rs/pull/1209)
- Add `resolve_multiple` to Resolver [#1189](https://github.com/iotaledger/identity.rs/pull/1189)
- Move jwk_storage and key_id_storage to Wasm lib [#1181](https://github.com/iotaledger/identity.rs/pull/1181)
- Wasm Bindings for JWT Presentations [#1179](https://github.com/iotaledger/identity.rs/pull/1179)
- Polish JWK thumbprint and document extension API [#1173](https://github.com/iotaledger/identity.rs/pull/1173)
- Wasm bindings for `KeyIdStorage` [#1147](https://github.com/iotaledger/identity.rs/pull/1147)
- Introduce `IToCoreDocument` and document locks in the bindings [#1120](https://github.com/iotaledger/identity.rs/pull/1120)
- Add Wasm Bindings for Domain Linkage [#1115](https://github.com/iotaledger/identity.rs/pull/1115)
- Add wasm credentials and presentations examples [#1075](https://github.com/iotaledger/identity.rs/pull/1075)
- Add revocation examples [#1076](https://github.com/iotaledger/identity.rs/pull/1076)
- Add `IotaDID.fromAliasId` to the Wasm bindings [\#1048](https://github.com/iotaledger/identity.rs/pull/1048)
- Expose Controller and Governor Addresses in metadata [\#1023](https://github.com/iotaledger/identity.rs/pull/1023)
- Add Wasm bindings for `CoreDocument` [\#994](https://github.com/iotaledger/identity.rs/pull/994)
- Add initial Wasm Stardust bindings [\#967](https://github.com/iotaledger/identity.rs/pull/967)

### Patch

- Fix holder claim check in VP [#1236](https://github.com/iotaledger/identity.rs/pull/1236)
- Fix issuer claim check in VC [#1235](https://github.com/iotaledger/identity.rs/pull/1235)
- Fix clippy's issue `uninlined-format-args` [#1109](https://github.com/iotaledger/identity.rs/pull/1109)
- Update iota.js peer dependency [#1107](https://github.com/iotaledger/identity.rs/pull/1107)
- Fix unresolved import in TS artifacts [\#1066](https://github.com/iotaledger/identity.rs/pull/1066)
- Recommend unique `credentialStatus.id` in `RevocationBitmap2022` [\#1039](https://github.com/iotaledger/identity.rs/pull/1039)
- Support case insensitive serialization of `RentStructure` [\#1012](https://github.com/iotaledger/identity.rs/pull/1012)
- Fix broken wasm bindings compilation [\#995](https://github.com/iotaledger/identity.rs/pull/995)
- Fix DID TypeScript references [\#977](https://github.com/iotaledger/identity.rs/pull/977)

## [wasm-v0.7.0-alpha.7](https://github.com/iotaledger/identity.rs/tree/wasm-v0.7.0-alpha.7) (2023-09-28)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.7.0-alpha.6...wasm-v0.7.0-alpha.7)

### Changed
- Use `VC Data Model v1.1` JWT encoding instead of `VC-JWT` [#1234](https://github.com/iotaledger/identity.rs/pull/1234)
- Change `verifiable_credential` to type `Vec<CRED>` in `Presentation` [#1231](https://github.com/iotaledger/identity.rs/pull/1231)


### Added
- Allow arbitrary JWS header parameters [#1245](https://github.com/iotaledger/identity.rs/pull/1245)
- Allow custom JWT claims for presentations [#1244](https://github.com/iotaledger/identity.rs/pull/1244)
- Allow custom `kid` to be set in JWS [#1239](https://github.com/iotaledger/identity.rs/pull/1239)
- Allow custom JWT claims for credentials [#1237](https://github.com/iotaledger/identity.rs/pull/1237)


### Patch
- Fix holder claim check in VP [#1236](https://github.com/iotaledger/identity.rs/pull/1236)
- Fix issuer claim check in VC [#1235](https://github.com/iotaledger/identity.rs/pull/1235)

## [wasm-v0.7.0-alpha.6](https://github.com/iotaledger/identity.rs/tree/wasm-v0.7.0-alpha.6) (2023-08-15)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.7.0-alpha.5...wasm-v0.7.0-alpha.6)

### Added

- Improve `Proof` [#1209](https://github.com/iotaledger/identity.rs/pull/1209)
- Add `resolve_multiple` to Resolver [#1189](https://github.com/iotaledger/identity.rs/pull/1189)
- Move jwk_storage and key_id_storage to Wasm lib [#1181](https://github.com/iotaledger/identity.rs/pull/1181)
- Wasm Bindings for JWT Presentations [#1179](https://github.com/iotaledger/identity.rs/pull/1179)
- Polish JWK thumbprint and document extension API [#1173](https://github.com/iotaledger/identity.rs/pull/1173)
- Wasm bindings for `KeyIdStorage` [#1147](https://github.com/iotaledger/identity.rs/pull/1147)
- Introduce `IToCoreDocument` and document locks in the bindings [#1120](https://github.com/iotaledger/identity.rs/pull/1120)
- Add Wasm Bindings for Domain Linkage [#1115](https://github.com/iotaledger/identity.rs/pull/1115)

### Changed 

- Polish Wasm bindings [#1206](https://github.com/iotaledger/identity.rs/pull/1206)
- Polish `identity_credential` [#1205](https://github.com/iotaledger/identity.rs/pull/1205)
- Polish `identity_iota_core` [#1203](https://github.com/iotaledger/identity.rs/pull/1203)
- Upgrade `client-wasm` to `sdk-wasm` [#1202](https://github.com/iotaledger/identity.rs/pull/1202)
- Rename `JwtPresentation` to `Presentation` [#1200](https://github.com/iotaledger/identity.rs/pull/1200)
- Remove legacy signing and verification APIs [#1194](https://github.com/iotaledger/identity.rs/pull/1194)
- Remove old `Presentation` type [#1190](https://github.com/iotaledger/identity.rs/pull/1190)
- Remove reexported `Resolver` validation APIs [#1183](https://github.com/iotaledger/identity.rs/pull/1183)
- Use JWT credentials for Domain Linkage [#1180](https://github.com/iotaledger/identity.rs/pull/1180)
- Remove stronghold nodejs bindings [#1178](https://github.com/iotaledger/identity.rs/pull/1178)
- JwkStorageDocument & JwtCredential validation [#1152](https://github.com/iotaledger/identity.rs/pull/1152)
- Add initial PublicKeyJwk support [#1143](https://github.com/iotaledger/identity.rs/pull/1143)
- Refactor `MethodType` to make it extensible [#1112](https://github.com/iotaledger/identity.rs/pull/1112)
- Remove generics in `CoreDocument`, `VerificationMethod`, `Service`, `DIDUrl` and `LinkedDomainService` [#1110](https://github.com/iotaledger/identity.rs/pull/1110)

### Patch

- Fix clippy's issue `uninlined-format-args` [#1109](https://github.com/iotaledger/identity.rs/pull/1109)
- Update iota.js peer dependency [#1107](https://github.com/iotaledger/identity.rs/pull/1107)

## [wasm-v0.7.0-alpha.5](https://github.com/iotaledger/identity.rs/tree/wasm-v0.7.0-alpha.5) (2023-01-24)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.7.0-alpha.4...wasm-v0.7.0-alpha.5)

### Changed 
- Use official client-wasm dependency in examples [#1097](https://github.com/iotaledger/identity.rs/pull/1097)

## [wasm-v0.7.0-alpha.4](https://github.com/iotaledger/identity.rs/tree/wasm-v0.7.0-alpha.4) (2022-11-24)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.7.0-alpha.3...wasm-v0.7.0-alpha.4)

### Added
- Add wasm credentials and presentations examples [#1075](https://github.com/iotaledger/identity.rs/pull/1075)
- Add revocation examples [#1076](https://github.com/iotaledger/identity.rs/pull/1076)

### Changed 
- More identifier checks in `CoreDocument` [#1067](https://github.com/iotaledger/identity.rs/pull/1067)
- Update to `iota-client` 2.0.1-rc.4 and `iota-client-wasm` 0.5.0-alpha.6 [#1088](https://github.com/iotaledger/identity.rs/pull/1088)

## [wasm-v0.7.0-alpha.3](https://github.com/iotaledger/identity.rs/tree/wasm-v0.7.0-alpha.3) (2022-11-01)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.7.0-alpha.2...wasm-v0.7.0-alpha.3)

### Added

- Add `IotaDID.fromAliasId` to the Wasm bindings [\#1048](https://github.com/iotaledger/identity.rs/pull/1048)

### Patch

- Fix unresolved import in TS artifacts [\#1066](https://github.com/iotaledger/identity.rs/pull/1066)


## [wasm-v0.7.0-alpha.2](https://github.com/iotaledger/identity.rs/tree/wasm-v0.7.0-alpha.2) (2022-09-30)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.7.0-alpha.1...wasm-v0.7.0-alpha.2)
 
### Changed

- Use Bech32-encoded state controller and governor addresses [\#1044](https://github.com/iotaledger/identity.rs/pull/1044)
- Expose iteration over verification relationship fields [\#1024](https://github.com/iotaledger/identity.rs/pull/1024)

### Added

- Expose Controller and Governor Addresses in metadata [\#1023](https://github.com/iotaledger/identity.rs/pull/1023)

### Patch

- Recommend unique `credentialStatus.id` in `RevocationBitmap2022` [\#1039](https://github.com/iotaledger/identity.rs/pull/1039)

## [wasm-v0.7.0-alpha.1](https://github.com/iotaledger/identity.rs/tree/wasm-v0.7.0-alpha.1) (2022-09-16)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.6.0...wasm-v0.7.0-alpha.1)
 
This version introduces a new DID method targeting the IOTA UTXO ledger. This method works fundamentally differently from the previous method and introduces new capabilities to interact with Layer 1 entities like native tokens, NFTs and smart contracts. 

 This is an early alpha release, so there may be breaking changes in upcoming versions that invalidate DIDs created with this version. The method at this point is only intended for experimentation. 

 Note: Identities created with the earlier versions cannot be resolved with this version of the library. 



### Changed

- Chore/rename mixed resolver [\#1026](https://github.com/iotaledger/identity.rs/pull/1026)
- Add length prefix to DID Document payloads [\#1010](https://github.com/iotaledger/identity.rs/pull/1010)
- Update Wasm credential, presentation validators for Stardust [\#1004](https://github.com/iotaledger/identity.rs/pull/1004)
- Rename `Stardust` types to `Iota` [\#1000](https://github.com/iotaledger/identity.rs/pull/1000)
- Change Stardust DID method to IOTA [\#982](https://github.com/iotaledger/identity.rs/pull/982)
- Add Wasm Stardust Client [\#975](https://github.com/iotaledger/identity.rs/pull/975)
- Generalized Resolver [\#970](https://github.com/iotaledger/identity.rs/pull/970)
- Change `Storage` to handle `CoreDID` [\#968](https://github.com/iotaledger/identity.rs/pull/968)
- Change `Storage` to store arbitrary blobs [\#953](https://github.com/iotaledger/identity.rs/pull/953)
- Change `Service` `type` field to allow sets [\#944](https://github.com/iotaledger/identity.rs/pull/944)
- Generalise `CredentialValidator`, `PresentationValidator` to support arbitrary DID Documents [\#935](https://github.com/iotaledger/identity.rs/pull/935)

### Added

- Add Wasm bindings for `CoreDocument` [\#994](https://github.com/iotaledger/identity.rs/pull/994)
- Add initial Wasm Stardust bindings [\#967](https://github.com/iotaledger/identity.rs/pull/967)

### Patch

- Support case insensitive serialization of `RentStructure` [\#1012](https://github.com/iotaledger/identity.rs/pull/1012)
- Fix broken wasm bindings compilation [\#995](https://github.com/iotaledger/identity.rs/pull/995)
- Fix DID TypeScript references [\#977](https://github.com/iotaledger/identity.rs/pull/977)

## [wasm-v0.6.0](https://github.com/iotaledger/identity.rs/tree/wasm-v0.6.0) (2022-06-15)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.5.0...wasm-v0.6.0)
 
The main feature of this release is the addition of the `RevocationBitmap2022` specification, offering efficient credential revocation on-Tangle. This is the replacement for the `MerkleKeyCollection` removed in v0.5.0, which offered similar functionality but fundamentally failed to scale beyond a few thousand revocations. 

 Other changes include encryption support using Elliptic Curve Diffie-Hellman (ECDH) and quality of life improvements for verifiable credential and presentation types in the Wasm bindings. 

 DID Documents created with v0.5.0 remain compatible with v0.6.0. This will be the last major release prior to changes for the Stardust update. 



### Changed

- Change `remove_service` to return boolean [\#877](https://github.com/iotaledger/identity.rs/pull/877)
- Change `DIDUrl::join` to borrow self [\#871](https://github.com/iotaledger/identity.rs/pull/871)
- Add `RevocationBitmap2022`, bump MSRV to 1.60 [\#861](https://github.com/iotaledger/identity.rs/pull/861)
- Add Wasm `Credential` and `Presentation` field getters and constructors [\#815](https://github.com/iotaledger/identity.rs/pull/815)
- Add Diffie-Hellman key exchange for encryption to `Account` [\#809](https://github.com/iotaledger/identity.rs/pull/809)

### Added

- Implement `ECDH-ES+A256KW` for `Storage` encryption [\#867](https://github.com/iotaledger/identity.rs/pull/867)
- Add Client option for retry publishing behaviour [\#820](https://github.com/iotaledger/identity.rs/pull/820)
- Implement `Storage` test suite [\#791](https://github.com/iotaledger/identity.rs/pull/791)

### Patch

- Fix Wasm `Account.createService` endpoint type [\#819](https://github.com/iotaledger/identity.rs/pull/819)
- Fix omitting `Resolver.verifyPresentation`, `Document.resolveMethod` optional parameters [\#807](https://github.com/iotaledger/identity.rs/pull/807)
- Fix Account `create_signed_*` function return types [\#794](https://github.com/iotaledger/identity.rs/pull/794)
- Fix musl-libc target for Stronghold Node.js bindings [\#789](https://github.com/iotaledger/identity.rs/pull/789)

## [wasm-v0.5.0](https://github.com/iotaledger/identity.rs/tree/wasm-v0.5.0) (2022-03-31)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.4.0...wasm-v0.5.0)
 
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
- Change Wasm key types to `UInt8Array` [\#734](https://github.com/iotaledger/identity.rs/pull/734)
- Refactor `KeyLocation` [\#729](https://github.com/iotaledger/identity.rs/pull/729)
- Move DID Document proof outside metadata [\#728](https://github.com/iotaledger/identity.rs/pull/728)
- Replace Wasm getters and setters with methods [\#706](https://github.com/iotaledger/identity.rs/pull/706)
- Replace Wasm `Config` with `ClientConfig` interface [\#696](https://github.com/iotaledger/identity.rs/pull/696)
- Change `IotaDocument::verify_document` from a static function to a method [\#675](https://github.com/iotaledger/identity.rs/pull/675)
- Make Wasm support dependent on `target_arch` rather than feature [\#666](https://github.com/iotaledger/identity.rs/pull/666)
- Refactor `CoreDocument`, `VerificationMethod`, `Service` to use generic DID [\#655](https://github.com/iotaledger/identity.rs/pull/655)
- Change `also_known_as` type to `OrderedSet` [\#632](https://github.com/iotaledger/identity.rs/pull/632)
- Add union type parameters [\#616](https://github.com/iotaledger/identity.rs/pull/616)
- Fix dependent diff updates being rejected [\#605](https://github.com/iotaledger/identity.rs/pull/605)
- Overhaul `CredentialValidator`, add `PresentationValidator` [\#599](https://github.com/iotaledger/identity.rs/pull/599)
- Remove JSON string escaping in diff messages [\#598](https://github.com/iotaledger/identity.rs/pull/598)
- Replace `ClientMap` with new `Resolver` [\#594](https://github.com/iotaledger/identity.rs/pull/594)
- Rename Wasm `VerifiableCredential`, `VerifiablePresentation`  [\#551](https://github.com/iotaledger/identity.rs/pull/551)
- Add signature `created`, `expires`, `challenge`, `domain`, `purpose` [\#548](https://github.com/iotaledger/identity.rs/pull/548)
- Refactor document metadata [\#540](https://github.com/iotaledger/identity.rs/pull/540)
- Replace `chrono` with `time` [\#529](https://github.com/iotaledger/identity.rs/pull/529)
- Rename `DocumentDiff` to `DiffMessage` [\#511](https://github.com/iotaledger/identity.rs/pull/511)
- Deterministic ordering of competing messages [\#506](https://github.com/iotaledger/identity.rs/pull/506)
- Check for existence & duplication of methods in `CoreDocument` [\#504](https://github.com/iotaledger/identity.rs/pull/504)
- Annotate Wasm async function return types [\#501](https://github.com/iotaledger/identity.rs/pull/501)
- Add `ExplorerUrl` to replace `Network` explorer methods [\#496](https://github.com/iotaledger/identity.rs/pull/496)
- Update `ServiceEndpoint` to support sets and maps [\#485](https://github.com/iotaledger/identity.rs/pull/485)
- Add message compression and versioning [\#466](https://github.com/iotaledger/identity.rs/pull/466)
- Update document signing key constraints and methods [\#458](https://github.com/iotaledger/identity.rs/pull/458)

### Added

- Expose Ed25519, X25519 length constants [\#772](https://github.com/iotaledger/identity.rs/pull/772)
- Add deep clone function in Wasm [\#705](https://github.com/iotaledger/identity.rs/pull/705)
- Add `Duration` for `Timestamp` arithmetic [\#684](https://github.com/iotaledger/identity.rs/pull/684)
- Add `Client` fallback to local PoW option [\#682](https://github.com/iotaledger/identity.rs/pull/682)
- Add Wasm `Service` constructor and field getters [\#680](https://github.com/iotaledger/identity.rs/pull/680)
- Complete `Document` Wasm bindings [\#679](https://github.com/iotaledger/identity.rs/pull/679)
- Add `Document.signDocument` for Wasm [\#674](https://github.com/iotaledger/identity.rs/pull/674)
- Add Wasm bindings for `set_controller` and `set_also_known_as` in the `Account` [\#668](https://github.com/iotaledger/identity.rs/pull/668)
- Add NodeJs bindings for Stronghold `Storage` [\#660](https://github.com/iotaledger/identity.rs/pull/660)
- Add Wasm `Account` `Storage` interface [\#597](https://github.com/iotaledger/identity.rs/pull/597)
- Add Wasm bindings for the `Account` [\#574](https://github.com/iotaledger/identity.rs/pull/574)
- Filter out DiffMessages updating signing methods [\#519](https://github.com/iotaledger/identity.rs/pull/519)
- Add publish with retry method [\#455](https://github.com/iotaledger/identity.rs/pull/455)

### Patch

- Fix stronghold.ts key types [\#763](https://github.com/iotaledger/identity.rs/pull/763)
- Fix `Uint8Array` references [\#760](https://github.com/iotaledger/identity.rs/pull/760)
- Enable Wasm weak references for automatic garbage collection [\#694](https://github.com/iotaledger/identity.rs/pull/694)
- Fix `WasmTimestamp` JSON serialization [\#688](https://github.com/iotaledger/identity.rs/pull/688)
- Fix Wasm `DID` conversion error names [\#651](https://github.com/iotaledger/identity.rs/pull/651)
- Support verification methods with the same fragment [\#623](https://github.com/iotaledger/identity.rs/pull/623)
- Use node-fetch \>= 2.6.7 [\#617](https://github.com/iotaledger/identity.rs/pull/617)
- Fix diff properties \(de\)serialization [\#611](https://github.com/iotaledger/identity.rs/pull/611)
- Fix incorrect names for `ResolvedDocument.integrationMessageId` & `mergeDiff`  [\#600](https://github.com/iotaledger/identity.rs/pull/600)
- Fix node-fetch conflict when multiple versions are included [\#587](https://github.com/iotaledger/identity.rs/pull/587)
- Enable local proof-of-work fallback [\#579](https://github.com/iotaledger/identity.rs/pull/579)
- Fix `Timestamp` in the Wasm bindings [\#541](https://github.com/iotaledger/identity.rs/pull/541)
- Improve client error messages [\#512](https://github.com/iotaledger/identity.rs/pull/512)
- Fix credential validation failing for documents with diff updates [\#490](https://github.com/iotaledger/identity.rs/pull/490)

### Deprecated

- Deprecate diff chain features [\#759](https://github.com/iotaledger/identity.rs/pull/759)

### Removed

- Remove `MerkleKeyCollection` [\#755](https://github.com/iotaledger/identity.rs/pull/755)
- Remove `Storage::set_password` [\#733](https://github.com/iotaledger/identity.rs/pull/733)
- Remove `publicKeyJwk` [\#732](https://github.com/iotaledger/identity.rs/pull/732)

## [wasm-v0.4.0](https://github.com/iotaledger/identity.rs/tree/wasm-v0.4.0) (2021-11-01)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/360bf5ce64a7f418249cdeadccb22b9aea7daeb6...wasm-v0.4.0)



\* *This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)*
