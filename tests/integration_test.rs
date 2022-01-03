use lunes_wasm;

#[test]
fn total_suplay_of_lunes_mainnet() {
    let response = lunes_wasm::total_supply_mainnet();
    let result = 150_728_537.61498705;
    assert_eq!(response, result);
}


#[test]
fn total_suplay_of_lunes_testnet() {
    let response = lunes_wasm::total_supply_testnet();
    let result = 800_100_000.0;
    assert_eq!(response, result);
}