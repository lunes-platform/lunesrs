use bs58;
use hex::{decode, encode};
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

pub const ADDRESS_VERSION: [u8; 2] = [1, 11];
pub const ADDRESS_CHECKSUM_LENGTH: u8 = 4;
pub const ADDRESS_HASH_LENGTH: u8 = 20;
pub const ADDRESS_LENGTH: u8 = 1 + 1 + ADDRESS_CHECKSUM_LENGTH + ADDRESS_HASH_LENGTH;

/// # b58ToVec Function
///
/// Use base 58 Decode input into vector (Vec<u8>)
///
/// # Example:
///
/// Basic usage:
///
/// ```rust
/// use lunesrs::account::utils::b58_to_vec;
///
/// let input = "2Ej3vQ".to_string();
/// let output = b58_to_vec(input);
///
/// assert_eq!(output, [48, 97, 48, 97]);
/// ```
#[wasm_bindgen(js_name = "b58ToVec")]
pub fn b58_to_vec(base58: String) -> Vec<u8> {
    match bs58::decode(base58).into_vec() {
        Ok(arr) => arr,
        Err(e) => panic!("\x1b[91m\x1bERRO de base58 para hexadecimal {}\x1b[0m", e),
    }
}

/// # Vec to Base58 Function
///
/// Use base 58 for Encode vec to string base58
///
/// # Example:
///
/// Basic usage:
///
/// ```rust
/// use lunesrs::account::utils::vec_to_b58;
///
/// let input = vec![48, 97, 48, 97];
/// let output = vec_to_b58(input);
///
/// assert_eq!(output, "2Ej3vQ".to_string());
/// ```
#[wasm_bindgen(js_name = "vecToB58")]
pub fn vec_to_b58(vec: Vec<u8>) -> String {
    bs58::encode(vec).into_string()
}

#[wasm_bindgen(js_name = "hexToB58")]
pub fn hex_to_b58(hex: String) -> String {
    bs58::encode(from_str_hex(hex)).into_string()
}

#[wasm_bindgen(js_name = "fromStrHex")]
pub fn from_str_hex(str_hex: String) -> Vec<u8> {
    match decode(str_hex) {
        Ok(string) => string,
        Err(e) => panic!(
            "\x1b[91m\x1bmGot one when converting string to hex: {}\x1b[0m",
            e
        ),
    }
}

/// # Choose a 3 random numbers
///
/// The function pick up a three random numbers in the range from 0 to 2048
///
/// # Example:
///
/// Basic usage:
///
/// ```rust
/// use lunesrs::account::utils::random_triple_number;
///
/// let x = random_triple_number();
/// assert_eq!(x.len(), 3);
/// ```
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

/// # Into a Vector with u32
///
/// Transform any function into a Vector with u32 only using this function
///
/// # Example:
///
/// Basic usage:
///
/// ```rust
/// use lunesrs::account::utils::to_vecu32;
///
/// let input = "lunes".as_bytes();
/// let output = to_vecu32(input.to_vec());
///
/// assert_eq!(output, [108, 117, 110, 101, 115]);
/// ```
#[wasm_bindgen(js_name = "toVecu32")]
pub fn to_vecu32(arr: Vec<u8>) -> Vec<u32> {
    arr.iter().map(|x| *x as u32).collect()
}

/// # Into Vector with u8
///
/// Transform any function into a Vector with u8, only using this function
///
/// # Example:
///
/// Basic Usage:
///
/// ```rust
/// use lunesrs::account::utils::to_vecu8;
///
/// let input = [108, 117, 110, 101, 115];
/// let output = to_vecu8(input.to_vec());
///
/// assert_eq!(output, [108, 117, 110, 101, 115]);
/// ```
#[wasm_bindgen(js_name = "toVecu8")]
pub fn to_vecu8(arr: Vec<u32>) -> Vec<u8> {
    arr.iter().map(|x| *x as u8).collect()
}

/// # Vector with a u32 Function
///
/// The function transform a Vector with u32 into a Hexadecimal number
///
/// # Example:
///
/// Basic usage:
///
/// ```rust
/// use lunesrs::account::utils::vecu32_to_hex;
///
/// let input: [u32; 4] = [9, 10, 11, 12];
/// let output = vecu32_to_hex(input.to_vec());
///
/// assert_eq!(output, "090a0b0c")
/// ```
#[wasm_bindgen(js_name = "vecu32ToHex")]
pub fn vecu32_to_hex(vec: Vec<u32>) -> String {
    encode(to_vecu8(vec))
}

/// # String into Vector with u32
///
/// The function transform a String into Vector with u32
///
/// # Example:
///
/// Basic Usage:
///
/// ```rust
/// use lunesrs::account::utils::str_to_vecu32;
///
/// let input = "lunes".to_string();
/// let output = str_to_vecu32(input);
///
/// assert_eq!(output, [108, 117, 110, 101, 115])
/// ```
#[wasm_bindgen(js_name = "stringToVecu32")]
pub fn str_to_vecu32(message: String) -> Vec<u32> {
    // message.as_bytes().iter().map(|x| *x as u32).collect()
    ed25519_axolotl::str_to_vec32(message)
}

/// # Vector u32 into String
///
/// The function transform a Vector with u32 into string
///
/// # Example:
///
/// Basic usage:
///
/// ```rust
/// use lunesrs::account::utils::vecu32_to_str;
///
/// let input: [u32; 5] = [108, 117, 110, 101, 115];
/// let output = vecu32_to_str(input.to_vec());
///
/// assert_eq!(output, "lunes")
/// ```
#[wasm_bindgen(js_name = "vecu32ToString")]
pub fn vecu32_to_str(message: Vec<u32>) -> String {
    ed25519_axolotl::vec32_to_str(&message)
}

/// # Serialize Unsigned Integer
///
/// The function transform a u64 in Array of bytes
///
/// # Example:
///
/// Basic usage:
///
/// ```rust
/// use lunesrs::account::utils::serialize_integer;
/// let uinteger = 1528077600000;
///
/// assert_eq!(serialize_integer(uinteger), [0, 0, 1, 99, 200, 133, 197, 0]);
/// assert_eq!(serialize_integer(1), [0, 0, 0, 0, 0, 0, 0, 1]);
/// ```
#[wasm_bindgen(js_name = "serializeUInteger")]
pub fn serialize_integer(number: u64) -> Vec<u8> {
    number.to_be_bytes().to_vec()
}

/// # Serialize String
///
/// The function transform a u64 in Array of bytes
///
/// # Example:
///
/// Basic usage:
///
/// ```rust
/// use lunesrs::account::utils::serialize_string;
/// let string = "E3ZpxkM2kvS78aFYG2xFfngchMgik4ogLLRa6CBJvVgz";
///
/// assert_eq!(
///     serialize_string(string),
///     [69, 51, 90, 112, 120, 107, 77, 50, 107, 118, 83, 55, 56, 97, 70, 89, 71, 50, 120, 70, 102, 110, 103, 99, 104, 77, 103, 105, 107, 52, 111, 103, 76, 76, 82, 97, 54, 67, 66, 74, 118, 86, 103, 122]
/// );
/// assert_eq!(
///     serialize_string(""),
///     [0]
/// );
/// ```
#[wasm_bindgen(js_name = "serializeString")]
pub fn serialize_string(token: &str) -> Vec<u8> {
    if token == "" {
        0u8.to_be_bytes().to_vec()
    } else {
        token.as_bytes().to_vec()
    }
}

#[wasm_bindgen(js_name = "stringToB58")]
pub fn str_to_b58(string: String) -> String {
    bs58::encode(string).into_string()
}

pub fn int_to_hex(int: u32) -> String {
    hex::encode(int.to_be_bytes())
}

pub fn vecu8_to_hex(vec: Vec<u8>) -> String {
    encode(vec)
}

pub fn str_to_hex(string: String) -> String {
    encode(string)
}

pub fn blake2b32b(data: Vec<u8>) -> String {
    use blake2::{
        digest::{Update, VariableOutput},
        Blake2bVar,
    };

    match Blake2bVar::new(32) {
        Ok(mut hash) => {
            hash.update(&data);
            vecu8_to_hex(hash.finalize_boxed().to_vec())
        }
        Err(e) => panic!("ERROR: {}", e),
    }
}

pub fn keccak256(data: Vec<u8>) -> String {
    use tiny_keccak::{Hasher, Keccak};

    let mut k256 = Keccak::v256();
    let mut result = [0; 32];

    k256.update(&data);
    k256.finalize(&mut result);

    vecu8_to_hex(result.to_vec())
}

pub fn random_bytes(size: usize) -> Vec<u32> {
    let mut seed: Vec<u32> = vec![0; size];
    let mut rng = rand::thread_rng();
    for i in 0..seed.len() {
        seed[i] = rng.gen_range(0..=255);
    }
    return seed;
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

    #[test]
    fn test_min_to_hex() {
        let output = int_to_hex(0u32);

        assert_eq!(output, "00000000");
    }

    #[test]
    fn test_random_to_hex() {
        let output = int_to_hex(86587u32);

        assert_eq!(output, "0001523b");
    }

    #[test]
    fn test_max_to_hex() {
        let output = int_to_hex(4_294_967_295u32);

        assert_eq!(output, "ffffffff");
    }
}
