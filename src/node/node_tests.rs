#[cfg(test)]
mod tests {
    // use crate::cardano_topology::{Node, Topology, TopologyResult, TYPE_MAINNET};
    use crate::node::Node;
    use crate::types::{AdakaiResult, NetworkType};

    const JSON_NODE_TEST: &str = r#"
        {
      "addr": "54.220.20.40",
      "port": 3002,
      "continent": "Europe",
      "state": "IE"
    }"#;

    #[test]
    fn basic_deserialize_node() {
        let top_result: AdakaiResult<Node> =
            Node::new_from_json(NetworkType::Mainnet, JSON_NODE_TEST.to_string());

        match top_result {
            Ok(node) => {
                assert_eq!("54.220.20.40", node.addr());
                let network = node.network_type();

                match network {
                    NetworkType::TestNet => {
                        assert_eq!(1,0);
                    }

                    _ => {}
                }


                println!("valency: {}", node.valency());
                println!("node: {}", node.addr());
            }
            Err(e) => {
                panic!("error not expected {} ", e)
            }
        };
    }
}
