use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn multiple_hidden_seed() {
    use lunesrs::account::wallet::hidden_seed;

    let seed = "scrub guard swim catch range upon dawn ensure segment alpha sentence spend effort bar benefit".to_string();
    let hash_seed = [
        [
            163, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24,
            209, 227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
        ],
        [
            158, 195, 158, 43, 235, 175, 81, 113, 71, 142, 134, 117, 226, 247, 140, 189, 9, 86,
            193, 54, 59, 40, 100, 59, 213, 171, 8, 113, 151, 244, 43, 116,
        ],
        [
            50, 135, 161, 15, 52, 78, 234, 177, 234, 101, 67, 192, 68, 174, 104, 124, 28, 156, 23,
            33, 81, 118, 210, 255, 127, 127, 59, 24, 148, 215, 25, 141,
        ],
        [
            68, 188, 249, 142, 153, 123, 119, 187, 134, 139, 142, 224, 144, 233, 96, 219, 118, 79,
            3, 179, 172, 145, 191, 189, 235, 205, 232, 119, 176, 55, 76, 197,
        ],
        [
            176, 132, 66, 150, 25, 7, 98, 166, 0, 121, 84, 17, 161, 132, 204, 58, 19, 4, 158, 161,
            26, 205, 63, 198, 230, 171, 218, 199, 199, 217, 26, 102,
        ],
    ];

    for (nonce, response) in (0..hash_seed.len()).zip(hash_seed) {
        assert_eq!(response.to_vec(), hidden_seed(nonce as u32, seed.clone()));
    }
}

#[test]
#[wasm_bindgen_test]
fn single_hidden_seed() {
    use lunesrs::account::wallet::hidden_seed;

    let seed = "scrub guard swim catch range upon dawn ensure segment alpha sentence spend effort bar benefit".to_string();
    let hidded_seed = hidden_seed(0, seed);

    assert_eq!(
        hidded_seed,
        [
            163, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24,
            209, 227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96
        ]
    )
}
