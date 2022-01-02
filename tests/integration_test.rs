use lunes_wasm;

#[test]
fn total_suplay_of_lunes() {
    let response = lunes_wasm::total_suplay_mainnet();
    let result = 150_728_537.61498705;
    assert_eq!(response, result);
}