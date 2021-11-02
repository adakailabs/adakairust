use curl::easy::Easy;

mod wget_tests;

pub const MAINNET_PEERS_URI: &str = "https://explorer.cardano.org/relays/topology.json";
pub const TESTNET_PEERS_URI: &str =
    "https://explorer.cardano-testnet.iohkdev.io/relays/topology.json";

pub fn mainnet_topology_all_peers() -> Vec<u8> {
    wget_cardano_file(MAINNET_PEERS_URI)
}

pub fn testnet_topology_all_peers() -> Vec<u8> {
    wget_cardano_file(TESTNET_PEERS_URI)
}

pub fn wget_cardano_file(url: &str) -> Vec<u8> {
    // Write the contents of rust-lang.org to stdout
    let mut buf = Vec::new();
    let mut handle = Easy::new();

    {
        handle.url(url).unwrap();

        let mut transfer = handle.transfer();

        transfer
            .write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    return buf;
}
