#![warn(missing_docs)]

extern crate pretty_env_logger;


use std::thread;
use std::time::{Duration, Instant};

use cardano_ouroboros_network::mux;
use futures::executor::block_on;
use log::debug;

use crate::node::Node;
use crate::ping::pinger::Pinger;
use crate::types::{MAINNET_MAGIC, NetworkType, TESTNET_MAGIC};

mod ping_tests;
mod pinger;

/// MessageOut holds the message crafted with the information of the node that wants to be pinged.
/// It is sent to a worker that can process it and respond by sending a MessageOut.
#[derive(Debug)]
pub enum MessageIn {
    /// Quit signal the worker thread to quit
    Quit,

    /// Node contains the message body
    Node {
        /// name: IP or valid DNS address of the node
        name: String,

        /// port: TCP port of the node
        port: u16 ,

        /// network_magic of the cardano network the node belongs to
        network_magic: NetworkType,

        /// id is the position in the input vector
        id:usize},
}

/// MessageOut holds the message crafted with the information return by the ping function.
/// It is sent to a worker the response vector of nodes with calculated latencies.
#[derive(Debug)]
pub enum MessageOut {
    #[allow(dead_code)]
    /// Quit signal the worker thread to quit
    Quit,
    /// Latency is the message body of the ping response
    Latency {

        /// conn_latency is the elapsed time measured from the start of the request until a connection
        /// was stablished
        conn_latency: Duration,

        /// total_latency is the total elapsed time taken by the node to respond
        total_latency: Duration,

        /// online is true if the node responded to the ping connection request
        online: bool,

        /// id is the position of the node in the input vector of nodes
        id: usize,

        /// is_error is true if there was any kind of connection error
        is_error: bool,

        /// error is the description of the error detected
        error: String,

        /// valency:  node valency
        valency: u16,

    },

}

// pub(crate) type PingResult<T> = Result<T, Box<dyn Error>>;

async fn call_ping(host: String, port: u16, network_magic: u32) -> (Duration, Duration, bool, String) {

    const RETRY_WAIT_NS: u32 = 100000000;

    for i in 0..4 {
        let start = Instant::now();
        match mux::connection::connect(&*host, port).await {
            Ok(channel) => {
                let connect_duration = start.elapsed();
                match channel.handshake(network_magic).await {
                    Ok(_) => {
                        let total_duration = start.elapsed();
                        debug!("ping: addr: {}, connect elapsed: {}, total elapsed: {} -- {}",
                            host,
                            connect_duration.as_millis(), total_duration.as_millis(), port);
                        return (connect_duration, total_duration, false, "".to_string());
                    }
                    Err(e) => {
                        debug!("error 1: {}", e.to_string());
                        return (Duration::new(10, 0), Duration::new(10, 0), true, e.to_string());
                    }
                }
            }
            Err(e) => {
                debug!("error 2: {}", e.to_string());
                if i >= 2 {
                    debug!("retry failed");
                    return (Duration::new(10, 0), Duration::new(10, 0), true, e.to_string());
                }else {
                    debug!("retry: {}", i);
                    thread::sleep(Duration::new(0, RETRY_WAIT_NS))
                }
            },
        }
    }
    return (Duration::new(0,0), Duration::new(0,0), true, "".to_string());
}

/// ping: sends a ping message to a node, return the ping result
/// # Arguments:
/// * `host:` the node IP address or DNS name
/// * `port:` the TCP port of the node to ping.
/// * `net_type:` network magic of the cardano network that the node belongs to
///
/// # Example:
/// ```
/// use crate::adakairust::ping::{ping};
/// use crate::adakairust::types::{NetworkType};
/// let name = "costa-rica.adakailabs.com".to_string();
/// let port = 5002;
///
/// let (conn_duration, total_duration, is_error, error) =
/// self::adakairust::ping::ping(name, port, self::adakairust::types::NetworkType::TestNet);
/// ```
pub fn ping(host: String, port: u16, net_type: NetworkType) -> (Duration, Duration, bool, String) {
    debug!("ping node: {}:{}", host,port);
    let mut network_magic= MAINNET_MAGIC;

    match net_type {
        NetworkType::TestNet => {
            network_magic = TESTNET_MAGIC;
        }
        NetworkType::Mainnet => {}
    }
    let future_ping = call_ping(host.to_string(), port, network_magic);
    block_on(future_ping)
}

/// ping_vec: sends a ping message to each of the nodes contained in the passed vector.
/// It returns the same vector with the connection and total latencies updated.
/// # Arguments:
///
pub fn ping_vec(mut in_node_vec: Vec<Node>,net_type: NetworkType) -> Vec<Node> {
    let mut pinger : Pinger = Pinger::new( in_node_vec.len());
    let (a, _) = pinger.run();

    for (id, node) in in_node_vec.iter().enumerate() {
        let cpu = pinger.next_worker();
        pinger.fan_out()[cpu]
            .send(MessageIn::Node {
                name: node.addr().to_string(),
                port: node.port(),
                network_magic: net_type,
                id,
            })
            .unwrap();
    }

    a.join().unwrap();

    let arc_msg_vec = pinger.msg_vec().clone();
    let msg_vec = arc_msg_vec.lock().unwrap();

    assert_eq!(in_node_vec.len(), msg_vec.len());

    for msg in msg_vec.iter() {
        match msg {
            MessageOut::Latency { conn_latency, total_latency, online, id,is_error, error, valency  } => {
                if *is_error && (*error == "405".to_string()) {
                    continue
                }
                let n_id = id.clone();
                in_node_vec[n_id].set_total_latency(total_latency.clone());
                in_node_vec[n_id].set_con_latency(conn_latency.clone());
                in_node_vec[n_id].set_online(online.clone());
                in_node_vec[n_id].set_online_error(error.clone());
                in_node_vec[n_id].set_valency(valency.clone());

                if !online {
                    assert_ne!("", error);
                    assert_eq!(true, *is_error);
                }
            },
            _ => {}
        }
    }

    debug!("joing done, continuing");

    for i in 0..pinger.worker_threads() {
        let quit_msg = pinger.fan_out()[i].send(MessageIn::Quit);

        match quit_msg {
            Ok(_) => {
                debug!("finalized worker: {}", i)
            }
            Err(e) => {
                debug!("could not finalize worker: {} --> {}", i, e.to_string())
            }
        }

    }

    debug!("all done");

    return in_node_vec

}
