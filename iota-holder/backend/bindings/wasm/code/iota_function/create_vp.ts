// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Duration,
    IotaIdentityClient,
    JwsSignatureOptions,
    JwtPresentationOptions,
    Presentation,
    Resolver,
    Storage,
    Timestamp,
    IotaDocument,
    Jwt,
} from "@iota/identity-wasm/node";
import * as path from 'path'
import { readFileSync, writeFileSync } from 'fs'
import { Client } from "@iota/sdk-wasm/node";

/**
 * This example shows how to create a Verifiable Presentation and validate it.
 * A Verifiable Presentation is the format in which a (collection of) Verifiable Credential(s) gets shared.
 * It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.
 */
export async function createVP(
    client:Client,
    holderStorage:Storage,
    credentialFile:string,
    holderDocument:IotaDocument,
    holderFragment:string,
    challenge:string,
    ) {

    // Holder creates a verifiable presentation from the issued credential for the verifier to validate.
    const didClient = new IotaIdentityClient(client);

    // Create a Verifiable Presentation from the Credential
    console.log("Creating Verifiable Presentation...");
    const filePath = path.join('credentials', credentialFile);
    const verifiableCredential = JSON.parse(readFileSync(filePath, 'utf-8'))
    console.log("read file successfully~");
    const credentialJwt = await new Jwt(verifiableCredential)
    
    const resolver = new Resolver({
        client: didClient,
    });

    const unsignedVp = new Presentation({
        holder: holderDocument.id(),
        verifiableCredential: [credentialJwt],
    });

    const nonce = challenge
    const expires = Timestamp.nowUTC().checkedAdd(Duration.minutes(10));

    const presentationJwt = await holderDocument.createPresentationJwt(
        holderStorage,
        holderFragment,
        unsignedVp,
        new JwsSignatureOptions({ nonce }),
        new JwtPresentationOptions({ expirationDate: expires }),
    );

// // verify vp (for testing)

//     const jwtPresentationValidationOptions = new JwtPresentationValidationOptions(
//         {
//             presentationVerifierOptions: new JwsVerificationOptions({ nonce }),
//         },
//     );

//     // Resolve the presentation holder.
//     const presentationHolderDID: CoreDID = JwtPresentationValidator.extractHolder(presentationJwt);
//     const resolvedHolder = await resolver.resolve(
//         presentationHolderDID.toString(),
//     );

//     // Validate presentation. Note that this doesn't validate the included credentials.
//     let decodedPresentation = new JwtPresentationValidator(new EdDSAJwsVerifier()).validate(
//         presentationJwt,
//         resolvedHolder,
//         jwtPresentationValidationOptions,
//     );

//     // Validate the credentials in the presentation.
//     let credentialValidator = new JwtCredentialValidator(new EdDSAJwsVerifier());
//     let validationOptions = new JwtCredentialValidationOptions({
//         subjectHolderRelationship: [
//             presentationHolderDID.toString(),
//             SubjectHolderRelationship.AlwaysSubject,
//         ],
//     });

//     let jwtCredentials: Jwt[] = decodedPresentation
//         .presentation()
//         .verifiableCredential()
//         .map((credential) => {
//             const jwt = credential.tryIntoJwt();
//             if (!jwt) {
//                 throw new Error("expected a JWT credential");
//             } else {
//                 return jwt;
//             }
//         });

//     // Concurrently resolve the issuers' documents.
//     let issuers: string[] = [];
//     for (let jwtCredential of jwtCredentials) {
//         let issuer = JwtCredentialValidator.extractIssuerFromJwt(jwtCredential);
//         issuers.push(issuer.toString());
//     }
//     let resolvedIssuers = await resolver.resolveMultiple(issuers);

//     // Validate the credentials in the presentation.
//     for (let i = 0; i < jwtCredentials.length; i++) {
//         credentialValidator.validate(
//             jwtCredentials[i],
//             resolvedIssuers[i],
//             validationOptions,
//             FailFast.FirstError,
//         );
//     }

//     // Since no errors were thrown we know that the validation was successful.
//     console.log(`VP successfully validated`);


    const fileNamePresentation = holderFragment + '-presentation.json'
    const presentationFilePath = path.join('presentations', fileNamePresentation)
    writeFileSync(presentationFilePath, JSON.stringify(presentationJwt, null, 4))

    console.log('VP was successfully created. see file:')
    console.log(presentationFilePath.toString())
    return 'VP was successfully created.'
}
