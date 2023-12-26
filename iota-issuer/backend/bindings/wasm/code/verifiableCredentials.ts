// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Credential,
    EdDSAJwsVerifier,
    FailFast,
    JwkMemStore,
    JwsSignatureOptions,
    JwtCredentialValidationOptions,
    JwtCredentialValidator,
    KeyIdMemStore,
    Storage,
    RevocationBitmap,
} from "@iota/identity-wasm/node";
import { Client, MnemonicSecretManager, Utils, SecretManagerType } from "@iota/sdk-wasm/node";
import { API_ENDPOINT } from "./util";

function transformString(inputStr: string): string {
    const result: string[] = [];
    for (const char of inputStr) {
        result.push(char);
        result.push(':');
    }
    result.pop();

    return result.join('');
}

export async function createSignedVerifiableCredential(client:Client, secretManager:SecretManagerType, issuerStorage:Storage, issuerDocument:any, issuerFragment:any, name:string, holderDid:string, country:string, birth:string, CREDENTIAL_INDEX:number) {
    const subject = {
        id: holderDid,
        name: name,
        country: country,
        birth: birth
    };

    let holderdid = await transformString(holderDid)
    console.log(holderdid)

    const unsignedVc = new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        credentialStatus: {
            id: issuerDocument.id() + "#my-revocation-service",
            type: RevocationBitmap.type(),
            revocationBitmapIndex: CREDENTIAL_INDEX.toString(),
        },
        issuer: issuerDocument.id(),
        credentialSubject: subject,
    });

    const credentialJwt = await issuerDocument.createCredentialJwt(
        issuerStorage,
        issuerFragment,
        unsignedVc,
        new JwsSignatureOptions(),
    );
    console.log(`Credential JWT > ${credentialJwt.toString()}`);

    return credentialJwt

    // // Before sending this credential to the holder the issuer wants to validate that some properties
    // // of the credential satisfy their expectations.

    // // Validate the credential's signature, the credential's semantic structure,
    // // check that the issuance date is not in the future and that the expiration date is not in the past.
    // // Note that the validation returns an object containing the decoded credential.
    // const decoded_credential = new JwtCredentialValidator(new EdDSAJwsVerifier()).validate(
    //     credentialJwt,
    //     issuerDocument,
    //     new JwtCredentialValidationOptions(),
    //     FailFast.FirstError,
    // );

    // // Since `validate` did not throw any errors we know that the credential was successfully validated.
    // console.log(`VC successfully validated`);

    // // The issuer is now sure that the credential they are about to issue satisfies their expectations.
    // // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
    // console.log(`Issued credential: ${JSON.stringify(decoded_credential.intoCredential(), null, 2)}`);
    // return credentialJwt
}
