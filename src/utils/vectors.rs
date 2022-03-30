use wasm_bindgen::prelude::wasm_bindgen;

/**
# Convert a Vec u8 to Vec u32

- Convert a vector of u8 in a new vector of u32

## In JavaScript 👍

```javascript
import * as wasm from "lunesrs"

const arrayu8: Uint8Array = new Uint8Array()
const arrayu32: Uint32Array = wasm.toVecu32(arrayu8)
```

## In Rust 🤝

```rust
use lunesrs::utils::vectors::to_vecu32;
use std::any::{Any, TypeId};

let output = to_vecu32([1u8;100].to_vec());

assert_eq!(
    true,
    output.iter().all(|x|
        TypeId::of::<u32>() == x.type_id()
    )
);
```
*/
#[wasm_bindgen(js_name = "toVecu32")]
pub fn to_vecu32(arr: Vec<u8>) -> Vec<u32> {
    arr.iter().map(|x| *x as u32).collect()
}

/**
# Convert a Vec u32 to Vec u8

- Convert a vector of u32 in a new vector of u8

## In JavaScript 👍

```javascript
import * as wasm from "lunesrs"

const arrayu32: Uint32Array = new Uint32Array()
const arrayu8: Uint8Array = wasm.toVecu8(arrayu32)
```

## In Rust 🤝

```rust
use lunesrs::utils::vectors::to_vecu8;
use std::any::{Any, TypeId};

let output = to_vecu8([1u32;100].to_vec());

assert_eq!(
    true,
    output.iter().all(|x|
        TypeId::of::<u8>() == x.type_id()
    )
);
```
*/
#[wasm_bindgen(js_name = "toVecu8")]
pub fn to_vecu8(arr: Vec<u32>) -> Vec<u8> {
    arr.iter().map(|x| *x as u8).collect()
}
