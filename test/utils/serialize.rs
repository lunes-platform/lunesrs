use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn verify_integer_serialization() {
    use lunesrs::utils::serialize::serialize_integer;

    let uinteger = 1528077600000;

    assert_eq!(serialize_integer(uinteger), [0, 0, 1, 99, 200, 133, 197, 0]);
    assert_eq!(serialize_integer(1), [0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
#[wasm_bindgen_test]
fn verify_string_serialization() {
    use lunesrs::utils::serialize::serialize_string;

    let string = "E3ZpxkM2kvS78aFYG2xFfngchMgik4ogLLRa6CBJvVgz";

    assert_eq!(serialize_string(""), [0]);
    assert_eq!(
        serialize_string(string),
        [
            69, 51, 90, 112, 120, 107, 77, 50, 107, 118, 83, 55, 56, 97, 70, 89, 71, 50, 120, 70,
            102, 110, 103, 99, 104, 77, 103, 105, 107, 52, 111, 103, 76, 76, 82, 97, 54, 67, 66,
            74, 118, 86, 103, 122
        ]
    );
}
