#[test]
fn verify_blake2b32b() {
    use lunesrs::utils::crypto::to_blake2b32b;

    let input = "lunes".to_string().as_bytes().to_vec();
    let output = to_blake2b32b(input);
    let response = vec![
        3, 23, 49, 134, 209, 156, 207, 249, 62, 92, 128, 38, 106, 243, 110, 40, 158, 13, 250, 196,
        204, 215, 253, 94, 96, 66, 18, 101, 5, 53, 212, 227,
    ];
    assert_eq!(output, response);
}

#[test]
fn verify_keccak256() {
    use lunesrs::utils::crypto::to_keccak256;

    let input = "lunes".to_string().as_bytes().to_vec();
    let output = to_keccak256(input);
    let response = vec![
        146, 251, 226, 85, 184, 131, 202, 174, 22, 215, 15, 169, 30, 71, 60, 127, 81, 109, 124,
        153, 76, 165, 96, 180, 85, 117, 180, 35, 14, 115, 80, 215,
    ];

    assert_eq!(output, response);
}

#[test]
fn verify_sha256() {
    use lunesrs::utils::crypto::to_sha256;

    let input = "lunes".to_string().as_bytes().to_vec();
    let output = to_sha256(input);
    let response = vec![
        146, 251, 226, 85, 184, 131, 202, 174, 22, 215, 15, 169, 30, 71, 60, 127, 81, 109, 124,
        153, 76, 165, 96, 180, 85, 117, 180, 35, 14, 115, 80, 215,
    ];

    assert_eq!(output, response);
}
