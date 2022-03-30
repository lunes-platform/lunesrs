/**
# Hash your message with *`sha2 256`*

- Receive a message in bytes(Vec u8) and return a hash in bytes(Vec u8)

## Example

```rust
use lunesrs::utils::crypto::to_sha256;
```
*/
pub fn to_sha256(message: Vec<u8>) -> Vec<u8> {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(&message);

    hasher.finalize().to_vec()
}

/**
# Hash your message with *`keccak 256`*

- Receive a message in bytes(Vec u8) and return a hash in bytes(Vec u8)

## Example

```rust
use lunesrs::utils::crypto::to_keccak256;

let input = "lunes".to_string().as_bytes().to_vec();
let output = to_keccak256(input);
let response = vec![
    146, 251, 226, 85, 184, 131, 202, 174, 22, 215, 15, 169, 30, 71, 60, 127, 81, 109, 124,
    153, 76, 165, 96, 180, 85, 117, 180, 35, 14, 115, 80, 215,
];

assert_eq!(output, response);
```
*/
pub fn to_keccak256(message: Vec<u8>) -> Vec<u8> {
    use tiny_keccak::{Hasher, Keccak};

    let mut k256 = Keccak::v256();
    let mut result = [0; 32];

    k256.update(&message);
    k256.finalize(&mut result);

    result.to_vec()
}

/**
# Hash your message with *`blake 2b 32bytes`*

- Receive a message in bytes(Vec u8) and return a hash in bytes(Vec u8)

## Example

```rust
use lunesrs::utils::crypto::to_blake2b32b;

let input = "lunes".to_string().as_bytes().to_vec();
let output = to_blake2b32b(input);
let response = vec![
    3, 23, 49, 134, 209, 156, 207, 249, 62, 92, 128, 38, 106, 243, 110, 40, 158, 13, 250,
    196, 204, 215, 253, 94, 96, 66, 18, 101, 5, 53, 212, 227
];

assert_eq!(output, response);
```
*/
pub fn to_blake2b32b(message: Vec<u8>) -> Vec<u8> {
    use blake2::{
        digest::{Update, VariableOutput},
        Blake2bVar,
    };

    match Blake2bVar::new(32) {
        Ok(mut hash) => {
            hash.update(&message);
            hash.finalize_boxed().to_vec()
        }
        Err(e) => panic!("{}", e),
    }
}
