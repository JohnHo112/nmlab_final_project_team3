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
import { AliasOutput, Client, IRent, MnemonicSecretManager, SecretManagerType, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT } from "./util";

export async function addRevocation(client:Client, secretManager:SecretManagerType, issuerStorage:Storage, issuerDocument:any){
    const didClient = new IotaIdentityClient(client);
    // Create a new empty revocation bitmap. No credential is revoked yet.
    const revocationBitmap = new RevocationBitmap();

    // Add the revocation bitmap to the DID Document of the issuer as a service.
    const serviceId = issuerDocument.id().join("#my-revocation-service");
    const service: Service = revocationBitmap.toService(serviceId);
    issuerDocument.insertService(service);

    // Resolve the latest output and update it with the given document.
    let aliasOutput: AliasOutput = await didClient.updateDidOutput(
        issuerDocument,
    );

    // Because the size of the DID document increased, we have to increase the allocated storage deposit.
    // This increases the deposit amount to the new minimum.
    let rentStructure: IRent = await didClient.getRentStructure();
    aliasOutput = await client.buildAliasOutput({
        ...aliasOutput,
        amount: Utils.computeStorageDeposit(aliasOutput, rentStructure),
        aliasId: aliasOutput.getAliasId(),
        unlockConditions: aliasOutput.getUnlockConditions(),
    });

    // Publish the document.
    issuerDocument = await didClient.publishDidOutput(
        secretManager,
        aliasOutput,
    );

    return issuerDocument;

}