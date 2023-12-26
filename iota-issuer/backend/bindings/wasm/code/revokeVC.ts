// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Credential,
    EdCurve,
    FailFast,
    IJwsVerifier,
    IotaDocument,
    IotaIdentityClient,
    Jwk,
    JwkMemStore,
    JwsAlgorithm,
    JwsSignatureOptions,
    JwtCredentialValidationOptions,
    JwtCredentialValidator,
    KeyIdMemStore,
    Resolver,
    RevocationBitmap,
    Service,
    Storage,
    VerificationMethod,
    verifyEd25519,
} from "@iota/identity-wasm/node";
import { AliasOutput, Client, IRent, MnemonicSecretManager, Utils, SecretManagerType } from "@iota/sdk-wasm/node";
import { API_ENDPOINT } from "./util";

export async function revokeVC(client:Client, issuerSecretManager:SecretManagerType, issuerStorage:Storage, issuerDocument:any, CREDENTIAL_INDEX:number) {
    const didClient = new IotaIdentityClient(client);
    issuerDocument.revokeCredentials("my-revocation-service", CREDENTIAL_INDEX);

    // Publish the changes.
    let aliasOutput: AliasOutput = await didClient.updateDidOutput(issuerDocument);
    let rentStructure: IRent = await didClient.getRentStructure();
    aliasOutput = await client.buildAliasOutput({
        ...aliasOutput,
        amount: Utils.computeStorageDeposit(aliasOutput, rentStructure),
        aliasId: aliasOutput.getAliasId(),
        unlockConditions: aliasOutput.getUnlockConditions(),
    });

    const update2: IotaDocument = await didClient.publishDidOutput(
        issuerSecretManager,
        aliasOutput,
    );
    
    console.log("revoke succ!")
}
