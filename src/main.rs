#[derive(PartialEq, Debug)]
enum Chain {
    Mainnet,
    Testnet,
}

pub struct Account {
    seed: String,
    nonce: i32,
    chain: Chain,
    prik: String,
    pubk: String,
    addr: String,
}

fn account_factory(
    seed: String,
    nonce: i32,
    chain: Chain,
    prik: String,
    pubk: String,
    addr: String,
) -> Account {
    Account {
        seed: seed,
        nonce: nonce,
        chain: chain,
        prik: prik,
        pubk: pubk,
        addr: addr,
    }
}

fn main() {
    let w = account_factory(
        String::from("phone dove naive find erase grant right cause garden struggle robust ball"),
        0,
        Chain::Testnet,
        String::from("4ErHkZRfViERqzDn23PaboWqwBJPUzYVZGFaGA3FvZNL"),
        String::from("6ABYrcXPHxrhiq1p7zcLV9EKxP24TPXr7e1LHimfqTz8"),
        String::from("37tXH2QNg15AtDp9rkJ391v2Z3FqpPwLybL"),
    );
    println!("{:?}", w.chain)
}
