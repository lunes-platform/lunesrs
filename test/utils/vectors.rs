use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn verify_type_u32() {
    use lunesrs::utils::vectors::to_vecu32;
    use std::any::{Any, TypeId};

    let output = to_vecu32([1u8; 100].to_vec());

    assert_eq!(
        true,
        output.iter().all(|x| TypeId::of::<u32>() == x.type_id())
    );
}

#[test]
#[wasm_bindgen_test]
fn verify_type_u8() {
    use lunesrs::utils::vectors::to_vecu8;
    use std::any::{Any, TypeId};

    let output = to_vecu8([1u32; 100].to_vec());

    assert_eq!(
        true,
        output.iter().all(|x| TypeId::of::<u8>() == x.type_id())
    );
}
