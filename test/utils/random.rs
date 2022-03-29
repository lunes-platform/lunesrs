use lunesrs::utils::random::{random_bytes, random_triple_number};
use wasm_bindgen_test::wasm_bindgen_test;

#[test]
fn verify_bytes() {
    assert_eq!(32, random_bytes(32).len());
    assert_eq!(
        true,
        random_bytes(10000).iter().all(|x| x.ge(&&0) && x.le(&&255))
    );
}

#[test]
#[wasm_bindgen_test]
fn verify_triple() {
    assert_eq!(3, random_triple_number().len());
    assert_eq!(
        true,
        random_triple_number()
            .iter()
            .all(|x| x.ge(&&0) && x.le(&&2048))
    );
}
