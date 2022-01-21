use lunescrab::{total_supply_mainnet, total_supply_testnet};


#[test]
fn total_supply_of_lunes_mainnet() {
    let response = total_supply_mainnet();
    let result = 150_728_537.61498705;
    assert_eq!(response, result);
}


#[test]
fn total_supply_of_lunes_testnet() {
    let response = total_supply_testnet();
    let result = 800_100_000.0;
    assert_eq!(response, result);
}
