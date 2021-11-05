#[cfg(test)]
mod tests {
    use crate::test::test_initialize;
    use crate::topology::{Topology, TopologyResult};
    use crate::types::NetworkType;

// const JSON_NODE_TEST: &str = r#"
    //     {
    //   "addr": "54.220.20.40",
    //   "port": 3002,
    //   "continent": "Europe",
    //   "state": "IE"
    // }"#;

    const JSON_TOPOLOGY_TEST: &str = r#"
{
  "Producers": [
    {
      "addr": "54.220.20.40",
      "port": 3002,
      "continent": "Europe",
      "state": "IE"
    },
    {
      "addr": "relays.mainnet.stakenuts.com",
      "port": 3001,
      "continent": "Europe",
      "state": "DE"
    },
    {
      "addr": "relay.zenithpool.io",
      "port": 31400,
      "continent": "North America",
      "state": "Arizona"
    }
]}"#;

    #[test]
    fn basic_parse_json() {
        test_initialize();
        let top_result: TopologyResult<Topology> =
            Topology::new_from_json(NetworkType::Mainnet, JSON_TOPOLOGY_TEST.to_string());

        match top_result {
            Ok(mut topology) => {

                topology.ping();

                assert_eq!("54.220.20.40", topology.producers()[0].addr());

                let net = topology.producers()[0].network_type();
                match net  {
                    NetworkType::TestNet => assert!(true),
                    _ => {
                        assert!(true)
                    }
                }

                println!("valency: {}", topology.producers()[0].valency());

                topology.pretty_print();

            }
            Err(e) => {
                panic!("error not expected {} ", e)
            }
        };
    }

    #[test]
    fn basic_all_peers_testnet() {
        test_initialize();
        let top_result: TopologyResult<Topology> = Topology::new_from_online_peers(NetworkType::TestNet);

        match top_result {
            Ok(mut topology) => {
                //topology.resolve_valencies();
                topology.ping();
                topology.sort();
                topology.pretty_print();
            }
            Err(_) => {
                panic!("test failed")
            }
        }
    }

    #[test]
    fn basic_all_peers_mainnet() {
        test_initialize();
        let top_result: TopologyResult<Topology> = Topology::new_from_online_peers(NetworkType::Mainnet);

        match top_result {
            Ok(mut topology) => {
                //topology.resolve_valencies();
                topology.ping();
                topology.sort();
                topology.pretty_print();
            }
            Err(_) => {
                panic!("test failed")
            }
        }
    }
    #[test]
    fn basic_all_testnet_peers_valencies() {
        test_initialize();
        let top_result: TopologyResult<Topology> = Topology::new_from_online_peers(NetworkType::TestNet);

        match top_result {
            Ok(mut topology) => {
                topology.resolve_valencies();

                for node in topology.producers.iter() {
                    let val = node.valency();
                    assert!(val >0);

                    if node.addr() == "relays-new.cardano-testnet.iohkdev.io" {
                        assert_eq!(8,val)
                    }

                }
                topology.pretty_print();
            }
            Err(_) => {
                panic!("test failed")
            }
        }



    }
}
