const express = require("express");
const app = express();
const cors = require("cors");
//const fs = require('fs');
const { init_issuer, createVC, revocation } = require("./code/index.ts")
import { AliasOutput, Client, MnemonicSecretManager, SecretManager, Utils } from "@iota/sdk-wasm/node";
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

import { API_ENDPOINT, ensureAddressHasFunds} from "./code/util";

app.use(cors());
app.use(express.json());

// connect endpoint & generate some key-pair & vm
const client = new Client({
    primaryNode: API_ENDPOINT,
    localPow: true,
});
const secretManager: MnemonicSecretManager = {
    mnemonic: Utils.generateMnemonic(),
};
let issuerStorage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
const verificationMethodFragment = "key-1"

let issuerDocument:any;
let issuerFragment:any;

let RevocationList:any = {};
let nextRevocationIdx:number = 1;

async function start() {
    [issuerDocument, issuerFragment] = await init_issuer(client, secretManager, issuerStorage, verificationMethodFragment);
}
start()

app.post("/create", async (req:any, res:any) => {
    //console.log(req.body);
    const name = req.body.name
    const holderDid = req.body.did
    const country = req.body.country
    const birth = req.body.birth
    const VC = await createVC(client, secretManager, issuerStorage, issuerDocument, issuerFragment, name, holderDid, country, birth, nextRevocationIdx)
    RevocationList[name] = nextRevocationIdx
    nextRevocationIdx += 1
    console.log(RevocationList)
    res.send(VC)
});

function deleteItemByValue(dict:any, value:any) {
    for (let key in dict) {
        if (dict.hasOwnProperty(key) && dict[key] === value) {
            delete dict[key];
            break;
        }
    }
}

app.get("/showRevocationList", (req:any, res:any) => {
    res.send(RevocationList)
});

app.post("/revocation", async (req:any, res:any) => {
    console.log(req.body);
    const revocationIdx = req.body.revocationIdx
    await revocation(client, secretManager, issuerStorage, issuerDocument, parseInt(revocationIdx));
    deleteItemByValue(RevocationList, parseInt(revocationIdx));
    console.log(RevocationList)
});

app.listen(5001, () => {
    console.log("Yey, your server is running on port 5001");
});

