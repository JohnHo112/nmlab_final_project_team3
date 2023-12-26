# IOTA-DID
## Introduction
This repository is an implementation of digital passport in iota-DID framework. The project includes three websites, each represents Holder(resident), Issuer(government) and Verifier(customs). 

- `Holder`: Create a DID, add Verification Method and Create Verifiable Presentation.
- `Issuer`: Create a DID, add Verification Method, Create Revocation list, Sign Verifiable Credential and Revoke Verifiable Credential.
- `Verifier`: Generate challenge, Verify Verifiable Presentation.

## Prerequisites
- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Nodejs](https://nodejs.org/en)

## Install Packages
```
npm run install-all-packages
```
## Holder
```
cd iota-holder
```
#### Run frontend
```
npm run holder-frontend
```
#### Run backend
```
npm run holder-backend
```

## Issuer
```
cd iota-issuer
```
#### Run frontend
```
npm run issuer-frontend
```
#### Run backend
```
npm run issuer-backend
```

## Verifier
```
cd iota-verifier
```
#### Run frontend
```
npm run verifier-frontend
```
#### Run backend
```
npm run verifier-backend
```

## Reference
- identity.rs: https://github.com/iotaledger/identity.rs.git
- IOTA Identity Framework: https://wiki.iota.org/identity.rs/welcome/