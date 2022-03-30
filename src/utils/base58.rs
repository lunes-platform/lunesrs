use wasm_bindgen::prelude::wasm_bindgen;

/**
# Encoding your message with *`base58`*

- Receive a message in bytes(Vec u8) and return string base58 encoded

## In JavaScript 👍

```javascript
import * as wasm from "lunesrs"

const result: string = wasm.arrayToBase58([108, 117, 110, 101, 115])
result === "DEiWH5L"
```

## In Rust 🤝

```rust
use lunesrs::utils::base58::vec_to_b58;

let input = "lunes".as_bytes().to_vec();
let output = vec_to_b58(input.clone());

assert_eq!(output, "DEiWH5L".to_string());
```
*/
#[wasm_bindgen(js_name = "arrayToBase58")]
pub fn vec_to_b58(vec: Vec<u8>) -> String {
    use bs58::encode;

    encode(vec).into_string()
}

/**
# Dencoding your message with *`base58`*

- Receive a message in bytes(Vec u8) and return string base58 decoded

## In JavaScript 👍

```javascript
import * as wasm from "lunesrs"

const result: Uint8Array = wasm.base58ToArray("DEiWH5L")
result === [108, 117, 110, 101, 115]
```

## In Rust 🤝

```rust
use lunesrs::utils::base58::b58_to_vec;

let input = "DEiWH5L".to_string();
let output = b58_to_vec(input);

assert_eq!(output, [108, 117, 110, 101, 115]);
```
*/
#[wasm_bindgen(js_name = "base58ToArray")]
pub fn b58_to_vec(message: String) -> Vec<u8> {
    use bs58::decode;

    match decode(message).into_vec() {
        Ok(arr) => arr,
        Err(e) => panic!("{}", e),
    }
}
