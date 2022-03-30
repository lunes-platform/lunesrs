use wasm_bindgen::prelude::wasm_bindgen;

/**
# Serialize Unsigned Integer

- The function transform a u64 in Array of bytes

## In JavaScript 👍

```javascript
```

## In Rust 🤝

```rust
use lunesrs::utils::serialize::serialize_integer;

let uinteger = 1528077600000;

assert_eq!(serialize_integer(uinteger), [0, 0, 1, 99, 200, 133, 197, 0]);
assert_eq!(serialize_integer(1), [0, 0, 0, 0, 0, 0, 0, 1]);
```
*/
#[wasm_bindgen(js_name = "serializeUInteger")]
pub fn serialize_integer(number: u64) -> Vec<u8> {
    number.to_be_bytes().to_vec()
}

/**
# Serialize Strings

- The function transform a string in Array of bytes
- 🖐 If the string is empty *`""`* will returned *`[0]`*

## In JavaScript 👍

```javascript
```

## In Rust 🤝

```rust
use lunesrs::utils::serialize::serialize_string;

let string = "E3ZpxkM2kvS78aFYG2xFfngchMgik4ogLLRa6CBJvVgz".to_string();

assert_eq!(serialize_string("".to_string()), [0]);
assert_eq!(
    serialize_string(string),
    [
        69, 51, 90, 112, 120, 107, 77, 50, 107, 118, 83, 55, 56, 97, 70, 89, 71, 50, 120, 70,
        102, 110, 103, 99, 104, 77, 103, 105, 107, 52, 111, 103, 76, 76, 82, 97, 54, 67, 66,
        74, 118, 86, 103, 122
    ]
);
```
*/
#[wasm_bindgen(js_name = "serializeString")]
pub fn serialize_string(token: String) -> Vec<u8> {
    if token == "" {
        0u8.to_be_bytes().to_vec()
    } else {
        token.as_bytes().to_vec()
    }
}
