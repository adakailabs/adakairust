


#[cfg(test)]
mod tests {
    use std::{thread, time};

    use crate::node::Node;
    use crate::ping::{MessageIn, ping, ping_vec, Pinger};
    use crate::test::test_initialize;
    use crate::types::NetworkType;

    extern crate pretty_env_logger;

    const JSON_TESTNET_NODE_TEST_BAD_0: &str = r#"
    {
      "addr": "costa-rica.adakailabs.com",
      "port": 2323,
      "continent": "North America",
      "state": "CR"
    }"#;



    const JSON_TESTNET_NODE_TEST_GOOD_0: &str = r#"
    {
      "addr": "costa-rica.adakailabs.com",
      "port": 5001,
      "continent": "North America",
      "state": "CR"
    }"#;

    const JSON_TESTNET_NODE_TEST_GOOD_1: &str = r#"
        {
      "addr": "north-america.relays-new.cardano-testnet.iohkdev.io",
      "port": 3001,
      "continent": "Europe",
      "state": "IE"
    }"#;

    #[test]
    fn test_ping_node_bad_0() {
        test_initialize();
        let node = Node::new_from_json(NetworkType::Mainnet, JSON_TESTNET_NODE_TEST_BAD_0.to_string()).unwrap();
        let host = node.addr().to_string();
        let port = node.port();
        let (_,_,is_error, the_error) = ping(host, port, NetworkType::TestNet);

        assert_eq!(true, is_error);

        info!("the error: {}", the_error)

    }


    #[test]
    fn test_ping_node_good_0() {
        test_initialize();
        let mut node = Node::new_from_json(NetworkType::Mainnet, JSON_TESTNET_NODE_TEST_GOOD_0.to_string()).unwrap();
        let host = node.addr().to_string();
        let port = node.port();

        let (con_duration,total_duration,is_error, the_error) = ping(host, port, NetworkType::TestNet);

        let x = con_duration.as_millis();
        let y = total_duration.as_millis();
        assert!(x > 10, "a = {}, b = {} ", x.to_string(), y.to_string());
        assert!(y > 10, "a = {}, b = {} ", x, y);

        if is_error  {
            panic!("error: {}", the_error)
        }

        node.resolve_valency();

        info!("valency: {}", node.valency());


    }

    #[test]
    fn test_ping_node_good_1() {
        test_initialize();
        let node = Node::new_from_json(NetworkType::Mainnet, JSON_TESTNET_NODE_TEST_GOOD_1.to_string()).unwrap();

        let host = node.addr().to_string();
        let port = node.port();

        let (con_duration,total_duration,is_error, the_error) = ping(host, port, NetworkType::TestNet);

        let x = con_duration.as_millis();
        let y = total_duration.as_millis();
        assert!(x > 10, "a = {}, b = {} ", x.to_string(), y.to_string());
        assert!(y > 10, "a = {}, b = {} ", x, y);

        if is_error  {
            panic!("error: {}", the_error)
        }
    }

    #[test]
    fn test_channels() {
        //Arc<RwLock<&mut StompSession>>
        let mut pinger = Pinger::new(num_cpus::get());
        let (_, _) = pinger.run();

        for i in 0..pinger.cpus() {
            pinger.fan_out()[i]
                .send(MessageIn::Node {
                    name: "adakai".to_string(),
                    port: 2,
                    network_magic: NetworkType::TestNet,
                    id:0
                })
                .unwrap();
        }

        for i in 0..pinger.cpus() {
            pinger.fan_out()[i].send(MessageIn::Quit).unwrap();
        }

        thread::sleep(time::Duration::from_secs(5));
    }

    #[test]
    fn ping_vector() {
        test_initialize();
        const VEC_SIZE: usize= 20;
        let mut node_vec = Vec::new();

        for _ in 0..VEC_SIZE {
            node_vec.push(Node::new_from_json(NetworkType::TestNet, JSON_TESTNET_NODE_TEST_GOOD_1.to_string()).unwrap());
        }

        let new_vec = ping_vec(node_vec, NetworkType::TestNet);

        assert_eq!(new_vec.len(), VEC_SIZE);

        for (_,node) in new_vec.iter().enumerate() {
            info!("node: {} {} -->", node.addr().to_string(), node.con_latency().as_millis());

            //assert_eq!(true,node.online() );

            if node.online() {
                assert!(node.con_latency().as_millis() > 10, "con_latency: {}", node.con_latency().as_millis());
                assert!(node.total_latency().as_millis() > 10);
            }else {
                error!("offline: {} ",node.addr() )
            }
        }

    }

    #[test]
    fn ping_vector_with_error() {
        test_initialize();
        const VEC_SIZE: usize= 20;
        let mut node_vec = Vec::new();

        // Inject correct nodes
        for _ in 0..VEC_SIZE-1 {
            node_vec.push(Node::new_from_json(NetworkType::TestNet, JSON_TESTNET_NODE_TEST_GOOD_1.to_string()).unwrap());
        }

        // Inject bad node
        node_vec.push(Node::new_from_json(NetworkType::TestNet, JSON_TESTNET_NODE_TEST_BAD_0.to_string()).unwrap());


        let new_vec = ping_vec(node_vec, NetworkType::TestNet);

        assert_eq!(new_vec.len(), VEC_SIZE);

        let mut error_count = 0;

        for (_,node) in new_vec.iter().enumerate() {
            if node.online() {
                assert!(node.con_latency().as_millis() > 10, "con_latency: {}", node.con_latency().as_millis());
                assert!(node.total_latency().as_millis() > 10);
            }else {
                error_count += 1;
                info!("expected error: {}" , node.online_error());
                assert_ne!("", node.online_error());
            }
        }
        assert_eq!(1, error_count);
    }

}
