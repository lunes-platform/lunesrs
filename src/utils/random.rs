use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

/**
# Return a vector with *`n`* random numbers

- The function takes three random numbers in the range from 0 to 255

## Example

```rust
use lunesrs::utils::random::random_bytes;

let x = [1;32];
assert_eq!(x.len(), random_bytes(32).len());

assert_eq!(
    true,
    // 0 <= x <= 255
    random_bytes(10000).iter().all(|x| x.ge(&&0) && x.le(&&255))
);
```
*/

pub fn random_bytes(size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(0..255))
        .collect::<Vec<u32>>()
}

/**
# Return a vector with 3 random numbers

- The function takes three random numbers in the range from 0 to 2048


## In JavaScript 👍

```javascript
import * as wasm from "lunesrs"

const vec = wasm.randomTripleNumber()

```
## In Rust 🤝

```rust
use lunesrs::utils::random::random_triple_number;

let x = [1,2,3];
assert_eq!(random_triple_number().len(), x.len());

assert_eq!(
    random_triple_number()
        .iter()
        // 0 <= x <= 2048
        .all(|x| x.le(&&2048) && x.gt(&&0)),
    true
);


```
*/
#[wasm_bindgen(js_name = "randomTripleNumber")]
pub fn random_triple_number() -> Vec<u32> {
    let word_count = 2048 - 1;
    let random = random_bytes(4);
    let x = random[3] + (random[2] << 8) + (random[1] << 16) + (random[0] << 24);
    let w1 = x % word_count;
    let w2 = (((x / word_count) >> 0) + w1) % word_count;
    let w3 = (((((x / word_count) >> 0) / word_count) >> 0) + w2) % word_count;
    vec![w1, w2, w3]
}
