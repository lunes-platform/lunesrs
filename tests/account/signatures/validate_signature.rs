use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn verify_validation() {
    use lunesrs::wallet::signatures::{fast_signature, validate_signature};
    use lunesrs::wallet::assembly::{to_private_key, to_public_key};

    let prvk = to_private_key(vec![1; 32]);
    let pubk = to_public_key(prvk.clone());
    let msg = "Lunes".as_bytes().to_vec();
    let signature = fast_signature(prvk, msg.clone());

    assert_eq!(true, validate_signature(pubk, msg, signature));
}
