import uploadFileRoute from "./uploadFile";
import downloadVPRoute from "./downloadVP";
import removeFileRoute from "./removeFile";
import { createDid } from "../iota_function/create_did";
import { createVP } from "../iota_function/create_vp";

import { Client } from "@iota/sdk-wasm/node";
import { API_ENDPOINT} from "../util";

const wrap =
  (fn: any) =>
  (...args: any) =>
    fn(...args).catch(args[2]);

const client = new Client({
  primaryNode: API_ENDPOINT,
  localPow: true,
});

let holderStorage:any;
let holderSecretManager:any;
let holderDocument:any;

function main(app: any) {
  app.post("/api/createDID", async (req: any, res: any) =>  {
    const VM = req.body.verificationMethod;
    const {address, document, fragment, storage} = await createDid(VM, client);
    holderStorage = storage;
    holderDocument = document;
    console.log("----create holder did succ----");
    console.log(holderDocument.id().toString());
    const holderDid = holderDocument.id().toString();
    res.send(holderDid);
  });
  app.get("/api/VP", async (req: any, res: any) => {
    const data = req.query;
    const holderDid = data.holderDID;
    const credentialFile = data.credentialFile;
    const holderFragment = data.fragment;
    const challenge = data.challenge;

    const message = await createVP(client, holderStorage, credentialFile, holderDocument, holderFragment, challenge);
    console.log(message);
    res.send(message);
  });
  app.post("/api/uploadFile", wrap(uploadFileRoute));
  app.get("/api/downloadVP", wrap(downloadVPRoute));
  app.post("/api/removeFile", wrap(removeFileRoute));
}

export default main;
