use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn addr_validation() {
    use lunesrs::account::signatures::validate_address;
    use lunesrs::utils::base58::b58_to_vec;

    let mainnet = 1;
    let addr = b58_to_vec("37o7aY3eZZTXmzrDa5e4Wj3Z4ZZuyV42Aaj".to_string());

    assert_eq!(true, validate_address(mainnet, addr));
}
