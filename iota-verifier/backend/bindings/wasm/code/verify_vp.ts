import {
    CoreDID,
    EdDSAJwsVerifier,
    FailFast,
    IotaIdentityClient,
    JwsVerificationOptions,
    Jwt,
    JwtCredentialValidationOptions,
    JwtCredentialValidator,
    JwtPresentationValidationOptions,
    JwtPresentationValidator,
    Resolver,
    SubjectHolderRelationship,
} from "@iota/identity-wasm/node";

import { Client } from "@iota/sdk-wasm/node";
import { API_ENDPOINT } from "./util";

export async function verify(presentationFile: string, challenge: string) {
    // Verifier sends the holder a challenge and requests a signed Verifiable Presentation.
    try {
        console.log("Verifying...")
        console.log("presentationFile: ", presentationFile)
        console.log("challenge: ", challenge)
        const client = new Client({
            primaryNode: API_ENDPOINT,
            localPow: true,
        });
        const didClient = new IotaIdentityClient(client);
    
        const verifiablePresentation = JSON.parse(presentationFile)
    
        const presentationJwt = new Jwt(verifiablePresentation);

        // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
        const nonce = challenge;
    
        // ===========================================================================
        // Step 7: Verifier receives the Verifiable Presentation and verifies it.
        // ===========================================================================
    
        // The verifier wants the following requirements to be satisfied:
        // - JWT verification of the presentation (including checking the requested challenge to mitigate replay attacks)
        // - JWT verification of the credentials.
        // - The presentation holder must always be the subject, regardless of the presence of the nonTransferable property
        // - The issuance date must not be in the future.
    
        const jwtPresentationValidationOptions = new JwtPresentationValidationOptions(
            {
                presentationVerifierOptions: new JwsVerificationOptions({ nonce }),
            },
        );
    
        const resolver = new Resolver({
            client: didClient,
        });
        // Resolve the presentation holder.
        const presentationHolderDID: CoreDID = JwtPresentationValidator.extractHolder(presentationJwt);
        const resolvedHolder = await resolver.resolve(
            presentationHolderDID.toString(),
        );
    
        // Validate presentation. Note that this doesn't validate the included credentials.
        let decodedPresentation = new JwtPresentationValidator(new EdDSAJwsVerifier()).validate(
            presentationJwt,
            resolvedHolder,
            jwtPresentationValidationOptions,
        );
    
        // Validate the credentials in the presentation.
        let credentialValidator = new JwtCredentialValidator(new EdDSAJwsVerifier());
        let validationOptions = new JwtCredentialValidationOptions({
            subjectHolderRelationship: [
                presentationHolderDID.toString(),
                SubjectHolderRelationship.AlwaysSubject,
            ],
        });
    
        let jwtCredentials: Jwt[] = decodedPresentation
            .presentation()
            .verifiableCredential()
            .map((credential) => {
                const jwt = credential.tryIntoJwt();
                if (!jwt) {
                    throw new Error("expected a JWT credential");
                } else {
                    return jwt;
                }
            });
    
        // Concurrently resolve the issuers' documents.
        let issuers: string[] = [];
        for (let jwtCredential of jwtCredentials) {
            let issuer = JwtCredentialValidator.extractIssuerFromJwt(jwtCredential);
            issuers.push(issuer.toString());
        }
        let resolvedIssuers = await resolver.resolveMultiple(issuers);
    
        // Validate the credentials in the presentation.
        for (let i = 0; i < jwtCredentials.length; i++) {
            credentialValidator.validate(
                jwtCredentials[i],
                resolvedIssuers[i],
                validationOptions,
                FailFast.FirstError,
            );
        }
    
        // Since no errors were thrown we know that the validation was successful.
        console.log(`VP successfully validated`);
        return 'VP successfully validated';
    }
    catch (error) {
        console.log("Error: ", error)
        return "VP validation unsuccessful";
    }    
}
