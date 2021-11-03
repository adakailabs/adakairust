#[cfg(test)]
mod tests {
    // use crate::cardano_topology::{Node, Topology, TopologyResult, TYPE_MAINNET};
    use crate::node::Node;
    use crate::test::test_initialize;
    use crate::types::{AdakaiResult, NetworkType};

// relays-new.cardano-testnet.iohkdev.io

    const JSON_NODE_TEST_IOHK_VALENCY_8: &str = r#"
        {
      "addr": "relays-new.cardano-testnet.iohkdev.io",
      "port": 3001
    }"#;

    const JSON_NODE_TEST_IP: &str = r#"
        {
      "addr": "54.220.20.40",
      "port": 3002,
      "continent": "Europe",
      "state": "IE"
    }"#;

    const JSON_NODE_TEST_DNS_NAME: &str = r#"
        {
      "addr": "costa-rica.adakailabs.com",
      "port": 3002
    }"#;


    #[test]
    fn basic_deserialize_node_ip() {
        test_initialize();
        let top_result: AdakaiResult<Node> =
            Node::new_from_json(NetworkType::Mainnet, JSON_NODE_TEST_IP.to_string());

        match top_result {
            Ok(mut node) => {

                node.node_ping();

                assert_eq!("54.220.20.40", node.addr());
                let network = node.network_type();

                match network {
                    NetworkType::TestNet => {
                        assert_eq!(1,0);
                    }

                    _ => {}
                }

                node.resolve_valency();

                info!("valency     :  {}", node.valency());
                info!("addr        :  {}", node.addr());
                info!("con latency :  {}", node.con_latency().as_millis() );
                info!("tot latency :  {}", node.total_latency().as_millis() );

                assert!(10 < node.con_latency().as_millis());
                assert!(10 < node.total_latency().as_millis());
                assert_eq!(1, node.valency());

            }
            Err(e) => {
                panic!("error not expected {} ", e)
            }
        };
    }

    #[test]
    fn basic_deserialize_node_dns_name() {
        test_initialize();
        let top_result: AdakaiResult<Node> =
            Node::new_from_json(NetworkType::Mainnet, JSON_NODE_TEST_DNS_NAME.to_string());

        match top_result {
            Ok(mut node) => {

                node.node_ping();

                assert_eq!("costa-rica.adakailabs.com", node.addr());
                let network = node.network_type();

                match network {
                    NetworkType::TestNet => {
                        assert_eq!(1,0);
                    }

                    _ => {}
                }

                node.resolve_valency();

                info!("valency     :  {}", node.valency());
                info!("addr        :  {}", node.addr());
                info!("con latency :  {}", node.con_latency().as_millis() );
                info!("tot latency :  {}", node.total_latency().as_millis() );

                assert!(10 < node.con_latency().as_millis());
                assert!(10 < node.total_latency().as_millis());
                assert_eq!(1, node.valency());

            }
            Err(e) => {
                panic!("error not expected {} ", e)
            }
        };
    }

    #[test]
    fn basic_deserialize_node_iohk_valency_8() {
        test_initialize();
        let top_result: AdakaiResult<Node> =
            Node::new_from_json(NetworkType::TestNet, JSON_NODE_TEST_IOHK_VALENCY_8.to_string());

        match top_result {
            Ok(mut node) => {

                node.node_ping();

                assert_eq!("relays-new.cardano-testnet.iohkdev.io", node.addr());
                let network = node.network_type();

                match network {
                    NetworkType::Mainnet => {
                        assert!(true);
                    }

                    _ => {}
                }

                node.resolve_valency();

                info!("valency     :  {}", node.valency());
                info!("addr        :  {}", node.addr());
                info!("con latency :  {}", node.con_latency().as_millis() );
                info!("tot latency :  {}", node.total_latency().as_millis() );

                assert!(10 < node.con_latency().as_millis());
                assert!(10 < node.total_latency().as_millis());
                assert_eq!(8, node.valency());

            }
            Err(e) => {
                panic!("error not expected {} ", e)
            }
        };
    }


}
