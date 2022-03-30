use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn full_sign_message() {
    use lunesrs::account::signatures::{full_signature, validate_signature};
    use lunesrs::account::wallet::{to_private_key, to_public_key};

    let prvk = to_private_key(vec![1; 32]);
    let pubk = to_public_key(prvk.clone());
    let msg = "Lunes".as_bytes().to_vec();
    let signature = full_signature(prvk, msg.clone());

    assert_eq!(true, validate_signature(pubk, msg, signature));
}
