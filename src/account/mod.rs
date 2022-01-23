use wasm_bindgen::prelude::*;
use utils::*;
pub mod utils;


#[wasm_bindgen]
pub fn to_blake2b32b_then_keccak256_then_hex(raw_seed: Vec<u8>) -> String {
    keccak256(from_str_hex(blake2b32b(raw_seed)))
}


#[cfg(test)]
mod test {
    use super::*;

    mod blake2b32b_then_keccak256 {
        use super::{to_blake2b32b_then_keccak256_then_hex, from_str_hex};

        #[test]
        fn test_raw_seed_for_0_nonce(){
            let input = from_str_hex("000000007363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let result = "cc872e22459e5c220323651e07097a30252162075fa10152e1de0f9b9c8c358a";
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, result);
        }


        #[test]
        fn test_raw_seed_for_1_nonce() {

            let input = from_str_hex("000000017363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let result = "312a14407453264a7dc508b4daa627f521bac6cd817f4f0d816690d7b5806897";
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, result);
        }

        #[test]
        fn test_raw_seed_for_2_nonce() {

            let input = from_str_hex("000000027363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let result = "29201f71d95566db58d1e3886b32a7da0333217dd6f6f63b6b73790fe8971b9a";
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, result);
        }

        #[test]
        fn test_raw_seed_for_3_nonce() {

            let input = from_str_hex("000000037363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let result = "efdee9564b60ab769e337c12ef27e5d20fa5c2e1951eda41dafa18be779a55c5";
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, result);
        }

        #[test]
        fn test_raw_seed_for_4_nonce() {

            let input = from_str_hex("000000047363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let result = "098d7080d6bfaf9a7b007e2aa91f3731c12498eff1a20c22e71692a008b0ffac";
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, result);
        }
    }
}