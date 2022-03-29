use wasm_bindgen::prelude::wasm_bindgen;

/**
# Hash your seed with *`blake 2b`*, *`keccak 256`* and *`sha 256`*

- The function hidden your seed and return a array of bytes(Vec u8)

# Example

```rust
let x = 0;
```
*/
#[wasm_bindgen(js_name = "hiddenSeed")]
pub fn hidden_seed(nonce: u32, seed: String) -> Vec<u8> {
    use crate::utils::crypto::{to_blake2b32b, to_keccak256, to_sha256};

    let raw_seed = [nonce.to_be_bytes().to_vec(), seed.as_bytes().to_vec()].concat();
    to_sha256(to_blake2b32b(to_keccak256(raw_seed)))
}

#[wasm_bindgen(js_name = "toPrivateKey")]
pub fn to_private_key(hash_seed: Vec<u8>) -> Vec<u8> {
    use crate::utils::vectors::{to_vecu32, to_vecu8};
    use ed25519_axolotl::KeyPair;

    to_vecu8(KeyPair::new(Some(to_vecu32(hash_seed))).prvk())
}

#[wasm_bindgen(js_name = "toPublicKey")]
pub fn to_public_key(private_key: Vec<u8>) -> Vec<u8> {
    use crate::utils::vectors::{to_vecu32, to_vecu8};
    use ed25519_axolotl::KeyPair;

    to_vecu8(KeyPair::new(Some(to_vecu32(private_key))).pubk())
}

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
