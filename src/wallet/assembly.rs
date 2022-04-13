use wasm_bindgen::prelude::wasm_bindgen;

/**
# Hidden your seed fist with *`blake 2b`* then *`keccak 256`* finally *`sha 256`*

- The function hash (hidden) your seed and return a array of bytes(Vec u8)

## In JavaScript 👍

```javascript
import * wasm from "lunesrs"

const seed = "scrub guard swim catch range upon dawn ensure segment alpha sentence spend effort bar benefit"
const hiddedSeed: Uint8Array = wasm.hiddenSeed(0, seed)

hiddedSeed == [
    163, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24, 209, 227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96
]
```

## In Rust 🤝

```rust
use lunesrs::wallet::assembly::hidden_seed;

let seed = "scrub guard swim catch range upon dawn ensure segment alpha sentence spend effort bar benefit".to_string();
let hidded_seed = hidden_seed(0, seed);

assert_eq!(
    hidded_seed,
    [163, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24, 209, 227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96]
)
```
*/
#[wasm_bindgen(js_name = "hiddenSeed")]
pub fn hidden_seed(nonce: u32, seed: String) -> Vec<u8> {
    use crate::utils::crypto::{to_blake2b32b, to_keccak256, to_sha256};

    let raw_seed = [nonce.to_be_bytes().to_vec(), seed.as_bytes().to_vec()].concat();
    to_sha256(to_keccak256(to_blake2b32b(raw_seed)))
}

/**
# To Private Key from Hidded Seed

- Receive your hash seed (hidden seed) and return a private key using Ed25519-Axolotl
- For more details about *how to works* click [here](https://crates.io/crates/ed25519-axolotl)

## In JavaScript 👍

```javascript
import * wasm from "lunesrs"

const hiddedSeed: Uint8Array = [
    163, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24, 209,
    227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
]
const privateKey: Uint8Array = wasm.to_privateKey(hiddedSeed)

privateKey == [
    160, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24,
    209, 227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
]
```

## In Rust 🤝

```rust
use lunesrs::wallet::assembly::to_private_key;

let hidden_seed: Vec<u8> = vec![
    163, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24, 209,
    227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
];

assert_eq!(
    to_private_key(hidden_seed),
    [
        160, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24,
        209, 227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
    ]
);
```
*/
#[wasm_bindgen(js_name = "toPrivateKey")]
pub fn to_private_key(hidded_seed: Vec<u8>) -> Vec<u8> {
    use crate::utils::vectors::{to_vecu32, to_vecu8};
    use ed25519_axolotl::crypto::keys::KeyPair;

    to_vecu8(KeyPair::new(Some(to_vecu32(hidded_seed))).prvk)
}

/**
# To Public Key from your Private Key

- Receive your private key and return a public key like a bytes using Ed25519-Axolotl
- For more details about *how to works* click [here](https://crates.io/crates/ed25519-axolotl)

## In JavaScript 👍

```javascript
import * wasm from "lunesrs"

const privateKey: Uint8Array = [
    160, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24,
    209, 227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
]
const publicKey: Uint8Array = wasm.toPublicKey(privateKey)

publicKey == [
    28, 105, 36, 199, 36, 111, 120, 95, 152, 208, 215, 39, 161, 71, 78, 237, 200, 160, 71,
    209, 177, 102, 140, 170, 56, 206, 9, 214, 227, 38, 117, 117,
]
```

## In Rust 🤝

```rust
use lunesrs::wallet::assembly::to_public_key;

let private_key: Vec<u8> = vec![
    160, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24, 209,
    227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
];

assert_eq!(
    to_public_key(private_key),
    [
        28, 105, 36, 199, 36, 111, 120, 95, 152, 208, 215, 39, 161, 71, 78, 237, 200, 160, 71,
        209, 177, 102, 140, 170, 56, 206, 9, 214, 227, 38, 117, 117,
    ]
);
```
*/
#[wasm_bindgen(js_name = "toPublicKey")]
pub fn to_public_key(private_key: Vec<u8>) -> Vec<u8> {
    use crate::utils::vectors::{to_vecu32, to_vecu8};
    use ed25519_axolotl::crypto::keys::KeyPair;

    to_vecu8(KeyPair::new(Some(to_vecu32(private_key))).pubk)
}

/**
# to Lunes Addres from Public Key

- Receive the version, chain and public key and return your lunes address like a bytes
- For more details how to works in [telescope](https://blockchain.lunes.io/telescope)

## In JavaScript 👍

```javascript
import * wasm from "lunesrs"

const versionAddress = 1
const mainnet_id = 1
const publicKey: Uint8Array = [
    28, 105, 36, 199, 36, 111, 120, 95, 152, 208, 215, 39, 161, 71, 78, 237, 200, 160, 71, 209,
    177, 102, 140, 170, 56, 206, 9, 214, 227, 38, 117, 117,
]
const address: Uint8Array = wasm.toAddress(versionAddress, mainnet_id, publicKey)

address == [
    28, 105, 36, 199, 36, 111, 120, 95, 152, 208, 215, 39, 161, 71, 78, 237, 200, 160, 71,
    209, 177, 102, 140, 170, 56, 206, 9, 214, 227, 38, 117, 117,
]
```

## In Rust 🤝

```rust
use lunesrs::wallet::assembly::to_address;

let mainnet_id = 1;
let version_address = 1;
let public_key: Vec<u8> = vec![
    28, 105, 36, 199, 36, 111, 120, 95, 152, 208, 215, 39, 161, 71, 78, 237, 200, 160, 71, 209,
    177, 102, 140, 170, 56, 206, 9, 214, 227, 38, 117, 117,
];

assert_eq!(
    to_address(version_address, mainnet_id, public_key),
    [
        1, 49, 44, 46, 82, 88, 220, 91, 204, 187, 92, 83, 89, 68, 39, 15, 115, 185, 143, 151,
        57, 38, 99, 41, 200, 192,
    ],
);
```
*/
#[wasm_bindgen(js_name = "toAddress")]
pub fn to_address(version: u8, chain: u8, public_key: Vec<u8>) -> Vec<u8> {
    use crate::utils::crypto::{to_blake2b32b, to_keccak256};

    let raw_addr = {
        let mut pubk = to_keccak256(to_blake2b32b(public_key))[0..20].to_vec();
        pubk.insert(0, chain.to_string().as_bytes()[0]);
        pubk.insert(0, version);

        pubk
    };
    let checksum = to_keccak256(to_blake2b32b(raw_addr.clone()))[0..4].to_vec();

    [raw_addr, checksum].concat()
}
