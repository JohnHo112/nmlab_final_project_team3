import {
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    MethodScope,
    Storage,
} from "@iota/identity-wasm/node";
import { type Address, AliasOutput, Client, MnemonicSecretManager, SecretManager, Utils, SecretManagerType } from "@iota/sdk-wasm/node";


const fs = require("fs");
const { createDid } = require("./createDid");
const { createSignedVerifiableCredential } = require("./verifiableCredentials")
import { addRevocation } from "./addRevocationBitmap"
const { revokeVC } = require("./revokeVC")

// Init issuer setup. Create did, add verification method and create revocation bitmap
export async function init_issuer(client:Client, secretManager:SecretManagerType, issuerStorage:Storage, verificationMethodFragment:string) {
    try{
        let {address, document, fragment} = await createDid(client, secretManager, issuerStorage, verificationMethodFragment);
        document = await addRevocation(client, secretManager, issuerStorage, document)
        console.log("issuer document:");
        console.log(document);
        return [document, fragment];
    } catch (error) {
        console.error(error);
        throw error;
    }
}

// Create verifiable credential
export async function createVC(client:Client, secretManager:SecretManagerType, issuerStorage:Storage, issuerDocument:any, issuerFragment:any, name:string, holderDid:string, country:string, birth:string, CREDENTIAL_INDEX:number) {

    const VCJson = await createSignedVerifiableCredential(client, secretManager, issuerStorage, issuerDocument, issuerFragment, name, holderDid, country, birth, CREDENTIAL_INDEX)
    return VCJson
}

// Revocation
export async function revocation(client:Client, secretManager:SecretManagerType, issuerStorage:Storage, issuerDocument:any, CREDENTIAL_INDEX:number) {
    await revokeVC(client, secretManager, issuerStorage, issuerDocument, CREDENTIAL_INDEX)
}
