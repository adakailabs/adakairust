#[cfg(test)]
mod tests {
    use crate::wget::{MAINNET_PEERS_URI, wget_cardano_file};

// use crate::cardano_wget::MAINNET_PEERS_URI;

    #[test]
    fn basic() {
        let json_vec = wget_cardano_file(MAINNET_PEERS_URI);
        let json = String::from_utf8(json_vec).expect("found invalid utf-8");

        println!("asdf:{}", json)
    }
}
