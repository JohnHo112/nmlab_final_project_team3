import * as ed from "@noble/ed25519";
import { decodeB64, encodeB64, Jwk, JwkGenOutput, JwkStorage } from "~identity_wasm";
import { EdCurve, JwkType, JwsAlgorithm } from "./jose";

type Ed25519PrivateKey = Uint8Array;
type Ed25519PublicKey = Uint8Array;

export class JwkMemStore implements JwkStorage {
    /** The map from key identifiers to Jwks. */
    private _keys: Map<string, Jwk>;

    /** Creates a new, empty `MemStore` instance. */
    constructor() {
        this._keys = new Map();
    }

    public static ed25519KeyType(): string {
        return "Ed25519";
    }

    public async generate(keyType: string, algorithm: JwsAlgorithm): Promise<JwkGenOutput> {
        if (keyType !== JwkMemStore.ed25519KeyType()) {
            throw new Error(`unsupported key type ${keyType}`);
        }

        if (algorithm !== JwsAlgorithm.EdDSA) {
            throw new Error(`unsupported algorithm`);
        }

        const keyId = randomKeyId();
        const privKey: Ed25519PrivateKey = ed.utils.randomPrivateKey();
        const jwk = await encodeJwk(privKey, algorithm);

        this._keys.set(keyId, jwk);

        const publicJWK = jwk.toPublic();
        if (!publicJWK) {
            throw new Error(`JWK is not a public key`);
        }

        return new JwkGenOutput(keyId, publicJWK);
    }

    public async sign(keyId: string, data: Uint8Array, publicKey: Jwk): Promise<Uint8Array> {
        if (publicKey.alg() !== JwsAlgorithm.EdDSA) {
            throw new Error("unsupported JWS algorithm");
        } else {
            if (publicKey.paramsOkp()?.crv !== (EdCurve.Ed25519 as string)) {
                throw new Error("unsupported Okp parameter");
            }
        }

        const jwk = this._keys.get(keyId);

        if (jwk) {
            const [privateKey, _] = decodeJwk(jwk);
            return ed.sign(data, privateKey);
        } else {
            throw new Error(`key with id ${keyId} not found`);
        }
    }

    public async insert(jwk: Jwk): Promise<string> {
        const keyId = randomKeyId();

        if (!jwk.isPrivate) {
            throw new Error("expected a JWK with all private key components set");
        }

        if (!jwk.alg()) {
            throw new Error("expected a Jwk with an `alg` parameter");
        }

        this._keys.set(keyId, jwk);

        return keyId;
    }

    public async delete(keyId: string): Promise<void> {
        this._keys.delete(keyId);
    }

    public async exists(keyId: string): Promise<boolean> {
        return this._keys.has(keyId);
    }

    public count(): number {
        return this._keys.size;
    }
}

// Encodes a Ed25519 keypair into a Jwk.
async function encodeJwk(privateKey: Ed25519PrivateKey, alg: JwsAlgorithm): Promise<Jwk> {
    const publicKey = await ed.getPublicKey(privateKey);
    let x = encodeB64(publicKey);
    let d = encodeB64(privateKey);

    return new Jwk({
        "kty": JwkType.Okp,
        "crv": "Ed25519",
        d,
        x,
        alg,
    });
}

function decodeJwk(jwk: Jwk): [Ed25519PrivateKey, Ed25519PublicKey] {
    if (jwk.alg() !== JwsAlgorithm.EdDSA) {
        throw new Error("unsupported `alg`");
    }

    const paramsOkp = jwk.paramsOkp();
    if (paramsOkp) {
        const d = paramsOkp.d;

        if (d) {
            let textEncoder = new TextEncoder();
            const privateKey = decodeB64(textEncoder.encode(d));
            const publicKey = decodeB64(textEncoder.encode(paramsOkp.x));
            return [privateKey, publicKey];
        } else {
            throw new Error("missing private key component");
        }
    } else {
        throw new Error("expected Okp params");
    }
}

// Returns a random number between `min` and `max` (inclusive).
// SAFETY NOTE: This is not cryptographically secure randomness and thus not suitable for production use.
// It suffices for our testing implementation however and avoids an external dependency.
function getRandomNumber(min: number, max: number): number {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

// Returns a random key id.
function randomKeyId(): string {
    const randomness = new Uint8Array(20);
    for (let index = 0; index < randomness.length; index++) {
        randomness[index] = getRandomNumber(0, 255);
    }

    return encodeB64(randomness);
}
