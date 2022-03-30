use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn verify_base58_decode() {
    use lunesrs::utils::base58::b58_to_vec;

    let input = "DEiWH5L".to_string();
    let output = b58_to_vec(input);

    assert_eq!(output, [108, 117, 110, 101, 115]);
}

#[test]
#[wasm_bindgen_test]
fn verify_base58_encode() {
    use lunesrs::utils::base58::vec_to_b58;

    let input = "lunes".as_bytes().to_vec();
    let output = vec_to_b58(input.clone());

    assert_eq!(output, "DEiWH5L".to_string());
}
