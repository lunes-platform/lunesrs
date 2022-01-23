use wasm_bindgen::prelude::wasm_bindgen;
use hex::{encode, decode};



#[wasm_bindgen]
pub fn vec_to_hex(vec: Vec<u8>) -> String {
    encode(vec)
}


#[wasm_bindgen]
pub fn str_to_hex(string: String) -> String {
    encode(string)
}


#[wasm_bindgen]
pub fn from_str_hex(str_hex: String) -> Vec<u8> {
    match decode(str_hex) {
        Ok(string) => string,
        Err(e) => panic!("ERROR: {}", e)
    }
}


#[wasm_bindgen]
pub fn blake2b32b(data: Vec<u8>) -> String {
    use blake2::{Blake2bVar, digest::{Update, VariableOutput}};

    match Blake2bVar::new(32) {
        Ok(mut hash) => {
            hash.update(&data);
            vec_to_hex(hash.finalize_boxed().to_vec())
        },
        Err(e) => panic!("ERROR: {}", e)
    }
}


#[wasm_bindgen]
pub fn keccak256(data: Vec<u8>) -> String {
    use tiny_keccak::{Hasher, Keccak};

    let mut k256 = Keccak::v256();
    let mut result = [0; 32];

    k256.update(&data);
    k256.finalize(&mut result);

    vec_to_hex(result.to_vec())
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blake2b_with_digest_size_of_32bytes() {
        let input = "lunes".to_string().as_bytes().to_vec();
        let result = "03173186d19ccff93e5c80266af36e289e0dfac4ccd7fd5e604212650535d4e3";
        let output = blake2b32b(input);
        assert_eq!(output, result);
    }

    #[test]
    fn test_keccak_with_digest_size_of_256bytes() {
        let input = "lunes".to_string().as_bytes().to_vec();
        let result = "92fbe255b883caae16d70fa91e473c7f516d7c994ca560b45575b4230e7350d7";
        let output = keccak256(input);
        assert_eq!(output, result);
    }

}