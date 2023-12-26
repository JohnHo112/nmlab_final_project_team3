// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IotaDocument,
    IotaIdentityClient,
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    MethodScope,
    Storage,
} from "@iota/identity-wasm/node";
import { type Address, AliasOutput, Client, MnemonicSecretManager, SecretManager, Utils } from "@iota/sdk-wasm/node";
import { ensureAddressHasFunds} from "../util";

/** Demonstrate how to create a DID Document and publish it in a new Alias Output. */
export async function createDid(verificationMethodFragment:string, client:Client): Promise<{
    address: Address;
    document: IotaDocument;
    fragment: string;
    storage: Storage;
}> {
    const didClient = new IotaIdentityClient(client);
    const networkHrp: string = await didClient.getNetworkHrp();

    const secretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };
    const secretManagerInstance = new SecretManager(secretManager);
    let holderStorage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());

    const walletAddressBech32 = (await secretManagerInstance.generateEd25519Addresses({
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
        bech32Hrp: networkHrp,
    }))[0];

    console.log("Wallet address Bech32:", walletAddressBech32);

    await ensureAddressHasFunds(client, walletAddressBech32);

    const address: Address = Utils.parseBech32Address(walletAddressBech32);

    // Create a new DID document with a placeholder DID.
    // The DID will be derived from the Alias Id of the Alias Output after publishing.
    let document = new IotaDocument(networkHrp);

    const fragment = await document.generateMethod(
        holderStorage,
        JwkMemStore.ed25519KeyType(),
        JwsAlgorithm.EdDSA,
        verificationMethodFragment,
        MethodScope.AssertionMethod(),
    );

    // Construct an Alias Output containing the DID document, with the wallet address
    // set as both the state controller and governor.
    let aliasOutput: AliasOutput = await didClient.newDidOutput(address, document);

    // Publish the Alias Output and get the published DID document.
    const published = await didClient.publishDidOutput(secretManager, aliasOutput);

    return { address, document: published, fragment, storage:holderStorage};
}