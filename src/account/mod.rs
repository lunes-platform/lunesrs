use wasm_bindgen::prelude::wasm_bindgen;
use utils::*;
pub mod utils;


#[wasm_bindgen]
pub fn to_blake2b32b_then_keccak256_then_hex(raw_seed: Vec<u8>) -> String {
    keccak256(from_str_hex(blake2b32b(raw_seed)))
}


#[wasm_bindgen]
pub fn to_sha256_then_hex(raw_hash_seed: Vec<u8>) -> String {
    use sha2::{Sha256, Digest};

    let mut hasher = Sha256::new();
    hasher.update(&raw_hash_seed);

    vec_to_hex(
        hasher.finalize().to_vec()
    )
}


#[wasm_bindgen]
pub fn to_private_key_hex(hash_seed: Vec<u8>) -> String {
    use ed25519_axolotl::KeyPair;

    let prvk = {
        KeyPair::from_seed(
            &hash_seed
            .iter()
            .map(|x| *x as u32)
            .collect()
        ).prvk
        .iter()
        .map(|x| *x as u8)
        .collect()
    };
    vec_to_hex(prvk)
}


#[wasm_bindgen]
pub fn to_public_key_hex(hash_seed: Vec<u8>) -> String {
    use ed25519_axolotl::KeyPair;

    let pubk = {
        KeyPair::from_seed(
            &hash_seed
            .iter()
            .map(|x| *x as u32)
            .collect()
        ).pubk
        .iter()
        .map(|x| *x as u8)
        .collect()
    };
    vec_to_hex(pubk)
}


#[wasm_bindgen]
pub fn to_address_hex(version: u8, chain: u8, public_key: Vec<u8>) -> String {
    use blake2::{Blake2bVar, digest::{Update, VariableOutput}};
    use tiny_keccak::{Hasher, Keccak};

    let raw_addr = {
        let mut pubk = match Blake2bVar::new(32) {
            Ok(mut hash) => {
                hash.update(&public_key);

                let mut k256 = Keccak::v256();
                let mut pubk = [0; 32];

                k256.update(&hash.finalize_boxed().to_vec());
                k256.finalize(&mut pubk);

                pubk[0..20].to_vec()
            },
            Err(e) => panic!("ERROR: {}", e)
        };

        pubk.insert(0, chain.to_string().as_bytes()[0]);
        pubk.insert(0, version);
        pubk
    };

    let checksum = match Blake2bVar::new(32) {
        Ok(mut hash) => {
            hash.update(&raw_addr);

            let mut k256 = Keccak::v256();
            let mut addr = [0; 32];

            k256.update(&hash.finalize_boxed().to_vec());
            k256.finalize(&mut addr);

            addr[0..4].to_vec()
        },
        Err(e) => panic!("ERROR: {}", e)
    };

    vec_to_hex([raw_addr, checksum].concat())
}


#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    mod blake2b32b_then_keccak256 {
        use super::{wasm_bindgen_test, to_blake2b32b_then_keccak256_then_hex, from_str_hex};

        #[test]
        #[wasm_bindgen_test]
        fn test_raw_seed_for_0_nonce(){
            let input = from_str_hex("000000007363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, "cc872e22459e5c220323651e07097a30252162075fa10152e1de0f9b9c8c358a");
        }


        #[test]
        #[wasm_bindgen_test]
        fn test_raw_seed_for_1_nonce() {

            let input = from_str_hex("000000017363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, "312a14407453264a7dc508b4daa627f521bac6cd817f4f0d816690d7b5806897");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_raw_seed_for_2_nonce() {

            let input = from_str_hex("000000027363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, "29201f71d95566db58d1e3886b32a7da0333217dd6f6f63b6b73790fe8971b9a");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_raw_seed_for_3_nonce() {

            let input = from_str_hex("000000037363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, "efdee9564b60ab769e337c12ef27e5d20fa5c2e1951eda41dafa18be779a55c5");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_raw_seed_for_4_nonce() {

            let input = from_str_hex("000000047363727562206775617264207377696d2063617463682072616e67652075706f6e206461776e20656e73757265207365676d656e7420616c7068612073656e74656e6365207370656e64206566666f7274206261722062656e65666974".to_string());
            let output = to_blake2b32b_then_keccak256_then_hex(input);
            assert_eq!(output, "098d7080d6bfaf9a7b007e2aa91f3731c12498eff1a20c22e71692a008b0ffac");
        }
    }


    mod sha256 {
        use super::{wasm_bindgen_test, to_sha256_then_hex, from_str_hex};

        #[test]
        #[wasm_bindgen_test]
        fn test_sha256_for_0_nonce() {
            let raw_hash_seed = from_str_hex("cc872e22459e5c220323651e07097a30252162075fa10152e1de0f9b9c8c358a".to_string());
            let output = to_sha256_then_hex(raw_hash_seed);
            assert_eq!(output, "a34211e1159080cbf115cdd1108adb9b323018d1e34f2368fc66d54a3fa51460");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_sha256_for_1_nonce() {
            let raw_hash_seed = from_str_hex("312a14407453264a7dc508b4daa627f521bac6cd817f4f0d816690d7b5806897".to_string());
            let output = to_sha256_then_hex(raw_hash_seed);
            assert_eq!(output, "9ec39e2bebaf5171478e8675e2f78cbd0956c1363b28643bd5ab087197f42b74");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_sha256_for_2_nonce() {
            let raw_hash_seed = from_str_hex("29201f71d95566db58d1e3886b32a7da0333217dd6f6f63b6b73790fe8971b9a".to_string());
            let output = to_sha256_then_hex(raw_hash_seed);
            assert_eq!(output, "3287a10f344eeab1ea6543c044ae687c1c9c17215176d2ff7f7f3b1894d7198d");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_sha256_for_3_nonce() {
            let raw_hash_seed = from_str_hex("efdee9564b60ab769e337c12ef27e5d20fa5c2e1951eda41dafa18be779a55c5".to_string());
            let output = to_sha256_then_hex(raw_hash_seed);
            assert_eq!(output, "44bcf98e997b77bb868b8ee090e960db764f03b3ac91bfbdebcde877b0374cc5");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_sha256_for_4_nonce() {
            let raw_hash_seed = from_str_hex("098d7080d6bfaf9a7b007e2aa91f3731c12498eff1a20c22e71692a008b0ffac".to_string());
            let output = to_sha256_then_hex(raw_hash_seed);
            assert_eq!(output, "b0844296190762a600795411a184cc3a13049ea11acd3fc6e6abdac7c7d91a66");
        }
    }


    mod private_key {
        use super::{wasm_bindgen_test, to_private_key_hex, from_str_hex};

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_private_key_for_0_nonce() {
            let hash_seed = from_str_hex("a34211e1159080cbf115cdd1108adb9b323018d1e34f2368fc66d54a3fa51460".to_string());
            let output = to_private_key_hex(hash_seed);
            assert_eq!(output, "a04211e1159080cbf115cdd1108adb9b323018d1e34f2368fc66d54a3fa51460");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_private_key_for_1_nonce() {
            let hash_seed = from_str_hex("9ec39e2bebaf5171478e8675e2f78cbd0956c1363b28643bd5ab087197f42b74".to_string());
            let output = to_private_key_hex(hash_seed);
            assert_eq!(output, "98c39e2bebaf5171478e8675e2f78cbd0956c1363b28643bd5ab087197f42b74");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_private_key_for_2_nonce() {
            let hash_seed = from_str_hex("3287a10f344eeab1ea6543c044ae687c1c9c17215176d2ff7f7f3b1894d7198d".to_string());
            let output = to_private_key_hex(hash_seed);
            assert_eq!(output, "3087a10f344eeab1ea6543c044ae687c1c9c17215176d2ff7f7f3b1894d7194d");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_private_key_for_3_nonce() {
            let hash_seed = from_str_hex("44bcf98e997b77bb868b8ee090e960db764f03b3ac91bfbdebcde877b0374cc5".to_string());
            let output = to_private_key_hex(hash_seed);
            assert_eq!(output, "40bcf98e997b77bb868b8ee090e960db764f03b3ac91bfbdebcde877b0374c45");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_private_key_for_4_nonce() {
            let hash_seed = from_str_hex("b0844296190762a600795411a184cc3a13049ea11acd3fc6e6abdac7c7d91a66".to_string());
            let output = to_private_key_hex(hash_seed);
            assert_eq!(output, "b0844296190762a600795411a184cc3a13049ea11acd3fc6e6abdac7c7d91a66");
        }
    }


    mod publick_key {
        use super::{wasm_bindgen_test, from_str_hex, to_public_key_hex};

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_public_key_for_0_nonce() {
            let prvk = from_str_hex("a04211e1159080cbf115cdd1108adb9b323018d1e34f2368fc66d54a3fa51460".to_string());
            let output = to_public_key_hex(prvk);
            assert_eq!(output, "1c6924c7246f785f98d0d727a1474eedc8a047d1b1668caa38ce09d6e3267575");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_to_public_key_hex_for_1_nonce() {
            let prvk = from_str_hex("98c39e2bebaf5171478e8675e2f78cbd0956c1363b28643bd5ab087197f42b74".to_string());
            let output = to_public_key_hex(prvk);
            assert_eq!(output, "8afbb187cc11d78b6b6ea39f4542e67d2e5a9bfb704c50e2f69f00f718ccee7f");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_public_key_for_2_nonce() {
            let prvk = from_str_hex("3087a10f344eeab1ea6543c044ae687c1c9c17215176d2ff7f7f3b1894d7194d".to_string());
            let output = to_public_key_hex(prvk);
            assert_eq!(output, "538f37cfbc714c62bcbb150679ed72573877f77b6beb7f5d6f7db1feea07b666");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_public_key_for_3_nonce() {
            let prvk = from_str_hex("40bcf98e997b77bb868b8ee090e960db764f03b3ac91bfbdebcde877b0374c45".to_string());
            let output = to_public_key_hex(prvk);
            assert_eq!(output, "18111dd232ddce7cf1a96d74cae4f10a42eb1fb34a3ddc726e111909a14e1873");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_generate_public_key_for_4_nonce() {
            let prvk = from_str_hex("b0844296190762a600795411a184cc3a13049ea11acd3fc6e6abdac7c7d91a66".to_string());
            let output = to_public_key_hex(prvk);
            assert_eq!(output, "c7331af1e72a2ea9019be355a04c7bbfb59f3042d19ca24feb42c7d32315a138");
    }
    }


    mod address {
        use super::{wasm_bindgen_test, to_address_hex, from_str_hex};

        mod mainnet {
            use super::*;

            mod version_1 {
                use super::*;

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_0_nonce() {
                    let pubk = from_str_hex("1c6924c7246f785f98d0d727a1474eedc8a047d1b1668caa38ce09d6e3267575".to_string());
                    let output = to_address_hex(1, 1, pubk);
                    assert_eq!(output, "01312c2e5258dc5bccbb5c535944270f73b98f9739266329c8c0");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_1_nonce() {
                    let pubk = from_str_hex("8afbb187cc11d78b6b6ea39f4542e67d2e5a9bfb704c50e2f69f00f718ccee7f".to_string());
                    let output = to_address_hex(1, 1, pubk);
                    assert_eq!(output, "0131640f230f396c4cf3f6ce7a6156387d52929902bff77423d8");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_2_nonce() {
                    let pubk = from_str_hex("538f37cfbc714c62bcbb150679ed72573877f77b6beb7f5d6f7db1feea07b666".to_string());
                    let output = to_address_hex(1, 1, pubk);
                    assert_eq!(output, "013146cc1229797733630bfa38be72ca6df585e8521fd44b5738");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_3_nonce() {
                    let pubk = from_str_hex("18111dd232ddce7cf1a96d74cae4f10a42eb1fb34a3ddc726e111909a14e1873".to_string());
                    let output = to_address_hex(1, 1, pubk);
                    assert_eq!(output, "0131842e3a128fd462b51805798d36909dac78ff9d43abb4d3b3");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_4_nonce() {
                    let pubk = from_str_hex("c7331af1e72a2ea9019be355a04c7bbfb59f3042d19ca24feb42c7d32315a138".to_string());
                    let output = to_address_hex(1, 1, pubk);
                    assert_eq!(output, "01317d211450834bfc6e0024549218833debf377968a4eca4b2d");
                }
            }

            mod version_2 {
                use super::*;

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_0_nonce() {
                    let pubk = from_str_hex("1c6924c7246f785f98d0d727a1474eedc8a047d1b1668caa38ce09d6e3267575".to_string());
                    let output = to_address_hex(11, 1, pubk);
                    assert_eq!(output, "0b312c2e5258dc5bccbb5c535944270f73b98f973926fe567f22");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_1_nonce() {
                    let pubk = from_str_hex("8afbb187cc11d78b6b6ea39f4542e67d2e5a9bfb704c50e2f69f00f718ccee7f".to_string());
                    let output = to_address_hex(11, 1, pubk);
                    assert_eq!(output, "0b31640f230f396c4cf3f6ce7a6156387d52929902bfabb54a49");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_2_nonce() {
                    let pubk = from_str_hex("538f37cfbc714c62bcbb150679ed72573877f77b6beb7f5d6f7db1feea07b666".to_string());
                    let output = to_address_hex(11, 1, pubk);
                    assert_eq!(output, "0b3146cc1229797733630bfa38be72ca6df585e8521f152bafba");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_3_nonce() {
                    let pubk = from_str_hex("18111dd232ddce7cf1a96d74cae4f10a42eb1fb34a3ddc726e111909a14e1873".to_string());
                    let output = to_address_hex(11, 1, pubk);
                    assert_eq!(output, "0b31842e3a128fd462b51805798d36909dac78ff9d43f2b8e319");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_mainnet_version_1_address_for_4_nonce() {
                    let pubk = from_str_hex("c7331af1e72a2ea9019be355a04c7bbfb59f3042d19ca24feb42c7d32315a138".to_string());
                    let output = to_address_hex(11, 1, pubk);
                    assert_eq!(output, "0b317d211450834bfc6e0024549218833debf377968ab695eaf7");
                }
            }
        }

        mod testnet {
            use super::*;

            mod version_1 {
                use super::*;

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_0_nonce() {
                    let pubk = from_str_hex("1c6924c7246f785f98d0d727a1474eedc8a047d1b1668caa38ce09d6e3267575".to_string());
                    let output = to_address_hex(1, 0, pubk);
                    assert_eq!(output, "01302c2e5258dc5bccbb5c535944270f73b98f973926d12b5dc0");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_1_nonce() {
                    let pubk = from_str_hex("8afbb187cc11d78b6b6ea39f4542e67d2e5a9bfb704c50e2f69f00f718ccee7f".to_string());
                    let output = to_address_hex(1, 0, pubk);
                    assert_eq!(output, "0130640f230f396c4cf3f6ce7a6156387d52929902bfdc19cb02");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_2_nonce() {
                    let pubk = from_str_hex("538f37cfbc714c62bcbb150679ed72573877f77b6beb7f5d6f7db1feea07b666".to_string());
                    let output = to_address_hex(1, 0, pubk);
                    assert_eq!(output, "013046cc1229797733630bfa38be72ca6df585e8521f42414610");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_3_nonce() {
                    let pubk = from_str_hex("18111dd232ddce7cf1a96d74cae4f10a42eb1fb34a3ddc726e111909a14e1873".to_string());
                    let output = to_address_hex(1, 0, pubk);
                    assert_eq!(output, "0130842e3a128fd462b51805798d36909dac78ff9d43e66f1ea7");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_4_nonce() {
                    let pubk = from_str_hex("c7331af1e72a2ea9019be355a04c7bbfb59f3042d19ca24feb42c7d32315a138".to_string());
                    let output = to_address_hex(1, 0, pubk);
                    assert_eq!(output, "01307d211450834bfc6e0024549218833debf377968a9619b6bb");
                }

            }

            mod version_2 {
                use super::*;

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_0_nonce() {
                    let pubk = from_str_hex("1c6924c7246f785f98d0d727a1474eedc8a047d1b1668caa38ce09d6e3267575".to_string());
                    let output = to_address_hex(11, 0, pubk);
                    assert_eq!(output, "0b302c2e5258dc5bccbb5c535944270f73b98f97392650423846");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_1_nonce() {
                    let pubk = from_str_hex("8afbb187cc11d78b6b6ea39f4542e67d2e5a9bfb704c50e2f69f00f718ccee7f".to_string());
                    let output = to_address_hex(11, 0, pubk);
                    assert_eq!(output, "0b30640f230f396c4cf3f6ce7a6156387d52929902bfbb9f09d2");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_2_nonce() {
                    let pubk = from_str_hex("538f37cfbc714c62bcbb150679ed72573877f77b6beb7f5d6f7db1feea07b666".to_string());
                    let output = to_address_hex(11, 0, pubk);
                    assert_eq!(output, "0b3046cc1229797733630bfa38be72ca6df585e8521fb809f734");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_3_nonce() {
                    let pubk = from_str_hex("18111dd232ddce7cf1a96d74cae4f10a42eb1fb34a3ddc726e111909a14e1873".to_string());
                    let output = to_address_hex(11, 0, pubk);
                    assert_eq!(output, "0b30842e3a128fd462b51805798d36909dac78ff9d43136b6231");
                }

                #[test]
                #[wasm_bindgen_test]
                fn test_generate_testnet_version_1_address_for_4_nonce() {
                    let pubk = from_str_hex("c7331af1e72a2ea9019be355a04c7bbfb59f3042d19ca24feb42c7d32315a138".to_string());
                let output = to_address_hex(11, 0, pubk);    
                    assert_eq!(output, "0b307d211450834bfc6e0024549218833debf377968ab7dbdb69");
                }
            }
        }
    }
}