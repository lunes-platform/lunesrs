use wasm_bindgen_test::wasm_bindgen_test;

#[test]
fn verify_bytes() {
    use lunesrs::utils::random::random_bytes;

    let x = [1; 32];
    assert_eq!(x.len(), random_bytes(32).len());
    assert_eq!(
        true,
        random_bytes(10000).iter().all(|x| x.ge(&&0) && x.le(&&255))
    );
}

#[test]
#[wasm_bindgen_test]
fn verify_triple() {
    use lunesrs::utils::random::random_triple_number;

    let x = [1, 2, 3];
    assert_eq!(random_triple_number().len(), x.len());
    assert_eq!(
        true,
        random_triple_number()
            .iter()
            .all(|x| x.ge(&&0) && x.le(&&2048))
    );
}
