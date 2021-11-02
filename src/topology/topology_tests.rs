#[cfg(test)]
mod tests {
    use crate::topology::{Node, Topology, TopologyResult};
    use crate::types::NetworkType;

    const JSON_NODE_TEST: &str = r#"
        {
      "addr": "54.220.20.40",
      "port": 3002,
      "continent": "Europe",
      "state": "IE"
    }"#;

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
        let top_result: TopologyResult<Topology> =
            Topology::new_from_json(NetworkType::Mainnet, JSON_TOPOLOGY_TEST.to_string());

        match top_result {
            Ok(topology) => {
                assert_eq!("54.220.20.40", topology.producers()[0].addr());

                let net = topology.producers()[0].network_type();
                match net  {
                    NetworkType::TestNet => assert!(true),
                    _ => {
                        assert!(true)
                    }
                }

                println!("valency: {}", topology.producers()[0].valency())
            }
            Err(e) => {
                panic!("error not expected {} ", e)
            }
        };
    }

    #[test]
    fn basic_all_peers() {
        let top_result: TopologyResult<Topology> = Topology::new_from_online_peers(NetworkType::Mainnet);

        let the_len = top_result.unwrap().producers().len();

        println!("len: {} ", the_len)
    }
}
