use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::crypto::{to_blake2b32b, to_keccak256};

/**
# Full Signature
## Sign a message with your private key

- Receive a message like bytes and return a signature like bytes
- It is possible to decode the signature back to the message
- (64 + message length) byte signature
- Slow to sign and verify

## In JavaScript 👍

```javascript
import * wasm from "lunesrs"

let privateKey: Uint8Array = wasm.toPrivateKey(new Uint8Array(32))
let publicKey: Uint8Array = wasm.toPublicKey(privateKey)
let massage: Uint8Array = wasm.serializeString("Lunes")
let signature: Uint8Array = fullSignature(privateKey, message)

true == wasm.validateSignature(publicKey, message, signature)
```

## In Rust 🤝

```rust
use lunesrs::account::signatures::{full_signature, validate_signature};
use lunesrs::account::wallet::{to_private_key, to_public_key};

let prvk = to_private_key(vec![1; 32]);
let pubk = to_public_key(prvk.clone());
let msg = "Lunes".as_bytes().to_vec();
let signature = full_signature(prvk, msg.clone());

assert_eq!(true, validate_signature(pubk, msg, signature));
```
*/
#[wasm_bindgen(js_name = "fullSignature")]
pub fn full_signature(private_key: Vec<u8>, msg: Vec<u8>) -> Vec<u8> {
    use crate::utils::vectors::{to_vecu32, to_vecu8};
    use ed25519_axolotl::{crypto::signatures::full_signature, utils::random::random_bytes};

    to_vecu8(full_signature(
        to_vecu32(private_key),
        to_vecu32(msg),
        Some(random_bytes(64)),
    ))
}

/**
# Fast Signature
## Sign a message with your private key

- Receive a message like bytes and return a signature like bytes
- Don't possible to decode signature back to message
- Quick to sign and verify
- 64 byte signature

## In JavaScript 👍

```javascript
import * wasm from "lunesrs"

let privateKey: Uint8Array = wasm.toPrivateKey(new Uint8Array(32))
let publicKey: Uint8Array = wasm.toPublicKey(privateKey)
let massage: Uint8Array = wasm.serializeString("Lunes")
let signature: Uint8Array = fastSignature(privateKey, message)

true == wasm.validateSignature(publicKey, message, signature)
```

## In Rust 🤝

```rust
use lunesrs::account::signatures::{fast_signature, validate_signature};
use lunesrs::account::wallet::{to_private_key, to_public_key};

let prvk = to_private_key(vec![1; 32]);
let pubk = to_public_key(prvk.clone());
let msg = "Lunes".as_bytes().to_vec();
let signature = fast_signature(prvk, msg.clone());

assert_eq!(true, validate_signature(pubk, msg, signature));
```
*/
#[wasm_bindgen(js_name = "fastSignature")]
pub fn fast_signature(private_key: Vec<u8>, msg: Vec<u8>) -> Vec<u8> {
    use crate::utils::vectors::{to_vecu32, to_vecu8};
    use ed25519_axolotl::{crypto::signatures::fast_signature, utils::random::random_bytes};

    to_vecu8(fast_signature(
        to_vecu32(private_key),
        to_vecu32(msg),
        Some(random_bytes(64)),
    ))
}

/**
# Validate Signature
## Validate a signature with a message and your public key

- Receive a public key, message, signature end return bool

## In JavaScript 👍

```javascript
import * wasm from "lunesrs"

let privateKey: Uint8Array = wasm.toPrivateKey(new Uint8Array(32))
let publicKey: Uint8Array = wasm.toPublicKey(privateKey)
let massage: Uint8Array = wasm.serializeString("Lunes")
let signature: Uint8Array = fastSignature(privateKey, message)

true == wasm.validateSignature(publicKey, message, signature)
```

## In Rust 🤝

```rust
use lunesrs::account::signatures::{fast_signature, validate_signature};
use lunesrs::account::wallet::{to_private_key, to_public_key};

let prvk = to_private_key(vec![1; 32]);
let pubk = to_public_key(prvk.clone());
let msg = "Lunes".as_bytes().to_vec();
let signature = fast_signature(prvk, msg.clone());

assert_eq!(true, validate_signature(pubk, msg, signature));
```
*/
#[wasm_bindgen(js_name = "validateSignature")]
pub fn validate_signature(public_key: Vec<u8>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    use crate::utils::vectors::to_vecu32;
    use ed25519_axolotl::crypto::signatures::validate_signature;

    validate_signature(
        to_vecu32(public_key),
        to_vecu32(message),
        to_vecu32(signature),
    )
}

/**
# Validate Address
## Validate an Address of a given Chain id

- Receive an address and a chain id end return bool
- ChainID:
    - MAINNET = 1
    - TESTNET = 0

## In JavaScript 👍

```javascript
import * wasm from "lunesrs"

const addr = "37o7aY3eZZTXmzrDa5e4Wj3Z4ZZuyV42Aaj"
const mainnet = 1

true == wasm.validateAddress(mainnet, addr)
```

## In Rust 🤝

```rust
use lunesrs::account::signatures::validate_address;
use lunesrs::utils::base58::b58_to_vec;

let mainnet = 1;
let addr = b58_to_vec("37o7aY3eZZTXmzrDa5e4Wj3Z4ZZuyV42Aaj".to_string());

assert_eq!(true, validate_address(mainnet, addr));
```
*/
#[wasm_bindgen(js_name = "validateAddress")]
pub fn validate_address(chain_id: u8, address: Vec<u8>) -> bool {
    use crate::account::{ADDRESS_CHECKSUM_LENGTH, ADDRESS_LENGTH, ADDRESS_VERSION};

    let (address_left, checksum) = {
        let x = address.len() - ADDRESS_CHECKSUM_LENGTH as usize;
        (address[..x].to_vec(), address[x..].to_vec())
    };
    let hash_address_left = to_keccak256(to_blake2b32b(address_left.clone()));
    let chain = &hash_address_left[..4];

    if address.len() as u8 != ADDRESS_LENGTH {
        return false;
    } else if address[1] != chain_id.to_string().as_bytes()[0] {
        return false;
    } else if ADDRESS_VERSION.contains(&address[0]) == false {
        return false;
    } else if checksum != chain {
        return false;
    } else {
        return true;
    }
}
