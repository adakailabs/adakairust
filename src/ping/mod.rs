#![warn(missing_docs)]

extern crate pretty_env_logger;

use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use cardano_ouroboros_network::mux;
use futures::executor::block_on;
use log::debug;

use crate::node::Node;
use crate::types::{MAINNET_MAGIC, NetworkType, TESTNET_MAGIC};

mod ping_tests;

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
        network_magic: u32,

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
        error: String},
}

// pub(crate) type PingResult<T> = Result<T, Box<dyn Error>>;

/// Pinger holds the internal functionality used for pinging multiple nodes concurrently
#[derive(Debug)]
pub(crate) struct Pinger {
    fan_in: Option<Sender<MessageOut>>,
    fan_out: Vec<Sender<MessageIn>>,
    cpus: usize,
    next_cpu: usize,
    msg_vec: Arc<Mutex<Vec<MessageOut>>>,
    size: usize,
}

impl Pinger {

    /// new return a ping with the internal fields initialized to it's default values.
    ///
    /// # Arguments
    /// * `size` - size is the size of the vector of nodes that will be pinged
    pub fn new (size: usize) -> Pinger {
        debug!("input vector size: {}", size);

        let mut p: Pinger = Pinger {
            fan_in: Option::None,
            fan_out: Vec::new(),
            cpus: num_cpus::get(),
            next_cpu : 0,
            msg_vec: Arc::new(Mutex::new(Vec::new())),
            size,
        };

        if p.cpus > 1 {
            p.cpus = p.cpus - 1
        }
        return p;
    }

    /// run: starts the internal workers (threads) in charged of performing pings.
    /// It will start one thread per available cpus - 1.
    pub fn run(&mut self) -> (JoinHandle<()>, Vec<JoinHandle<()>>) {
        let a = self.go_producer();
        let b  = self.go_workers();
        thread::sleep(time::Duration::from_secs(1)); //FIXME
        return (a,b);
    }

    /// fan_out: returns a vector with the sender channel for each worker (pinger).  It is used for
    /// sending the nodes data (MessageIn) for pinging a node
    pub fn fan_out(&self) -> &Vec<Sender<MessageIn>> {
        &self.fan_out
    }

    /// cpus: return the number of cpus that will be used
    pub fn cpus(&mut self) -> usize {
        return self.cpus;
    }

    /// next_cpu: used as an iterator for distributing work among all available cpus
    pub fn next_cpu(&mut self) -> usize {
        self.next_cpu += 1;
        if self.next_cpu == self.cpus {
            self.next_cpu = 0
        }
        self.next_cpu
    }

    /// msg_vec: returns the vector of MessageOut contain the ping results for each node.
    pub fn msg_vec(&self) -> Arc<Mutex<Vec<MessageOut>>> {
        return self.msg_vec.clone();
    }

    fn go_workers(&mut self) -> Vec<JoinHandle<()>> {

        let mut out: Vec<JoinHandle<()>> = Vec::new();

        for i in 0..self.cpus {
            let (tx,out_i) = self.go_worker(self.fan_in.clone(), i);
            self.fan_out.push(tx);
            out.push(out_i);
        }
        thread::sleep(time::Duration::from_secs(1));
        return out
    }

    fn go_producer(&mut self) -> JoinHandle<()>{
        let (tx, rx): (
            std::sync::mpsc::Sender<MessageOut>,
            std::sync::mpsc::Receiver<MessageOut>,
        ) = channel();

        self.fan_in = Some(tx.clone());

        let local_node_vec = self.msg_vec.clone();
        let mut local_counter:usize = 0;
        let size = self.size;


        let _guard = thread::spawn(move || loop {
            let msg = rx.recv();

            match msg {
                Ok(decoded) => match decoded {
                    MessageOut::Quit => {
                        debug!("msg_out: QUIT: {:?}", decoded);
                        break
                    }
                    MessageOut::Latency { .. } => {
                        let mut protected = local_node_vec.lock().unwrap();  // FIXME: mutex probably unneeded ???
                        protected.push(decoded);

                        local_counter += 1;
                        if local_counter == size  {
                            debug!("breaking go_producer");
                            break
                        }
                    }
                },
                Err(e) => {
                    error!("err: {:?}", e);
                    break;
                }
            }
        });



        return _guard

    }

    fn go_worker(&mut self, output_opt: Option<Sender<MessageOut>>, i: usize) -> (Sender<MessageIn>, JoinHandle<()>) {
        let output = output_opt.unwrap();
        debug!("starting worker {}", i);
        let (tx, rx): (
            std::sync::mpsc::Sender<MessageIn>,
            std::sync::mpsc::Receiver<MessageIn>,
        ) = channel();

        let _guard = thread::spawn(move || loop {
            let msg = rx.recv();
            match msg {
                Ok(decoded) => match decoded {
                    MessageIn::Quit => {
                        debug!("msg: QUIT: {:?} --> worker: {:?}", decoded, i);
                        break;
                    }
                    MessageIn::Node { name,port,network_magic, id} => {
                        debug!("msg: NODE: {} --> worker: {} - {} - {} ",i, name, port, network_magic);
                        let (conn_duration, total_duration, is_error, error) = ping(name, port, NetworkType::TestNet); //fixme
                        output.send(MessageOut::Latency {
                            conn_latency: conn_duration,
                            total_latency: total_duration,
                            id,
                            online: !is_error,
                            is_error,
                            error,
                        }).unwrap();
                    }
                },
                Err(e) => {
                    error!("err: {:?} --> worker: {:?}", e, i);
                    return;
                }
            }
        });

        return (tx.clone(),_guard);
    }
}

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
                        debug!("ping: connect elapsed: {}, total elapsed: {} -- {}",
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
/// use crate::ping;
/// let (conn_duration, total_duration, is_error, error) = ping(name, port, NetworkType::TestNet); //fixme
/// ```
pub fn ping(host: String, port: u16, net_type: NetworkType) -> (Duration, Duration, bool, String) {
    debug!("ping node: {}:{}", host,port);
    let mut network_magic= MAINNET_MAGIC;

    match net_type {
        NetworkType::TestNet => {
            network_magic = TESTNET_MAGIC;
            debug!("network type: testnet")
        }
        NetworkType::Mainnet => {
            debug!("network type: mainnet")
        }
    }
    let future_ping = call_ping(host.to_string(), port, network_magic);
    block_on(future_ping)
}

/// ping_vec: sends a ping message to each of the nodes contained in the passed vector.
/// It returns the same vector with the connection and total latencies updated.
/// # Arguments:
///
pub fn ping_vec(mut in_node_vec: Vec<Node>,net_type: NetworkType) -> Vec<Node> {

    let mut network_magic= MAINNET_MAGIC;
    match net_type {
        NetworkType::TestNet => {
            network_magic = TESTNET_MAGIC;
        }
        NetworkType::Mainnet => {
        }
    }

    let mut pinger : Pinger = Pinger::new( in_node_vec.len());
    let (a, _) = pinger.run();

    for (id, node) in in_node_vec.iter().enumerate() {

        debug!("node to ping: {} --> {}:{}",id, node.addr().to_string(),node.port());

        let cpu = pinger.next_cpu();
        pinger.fan_out()[cpu]
            .send(MessageIn::Node {
                name: node.addr().to_string(),
                port: node.port(),
                network_magic,
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
            MessageOut::Latency { conn_latency, total_latency, online, id,is_error, error  } => {
                let n_id = id.clone();
                in_node_vec[n_id].set_total_latency(total_latency.clone());
                in_node_vec[n_id].set_con_latency(conn_latency.clone());
                in_node_vec[n_id].set_online(online.clone());
                in_node_vec[n_id].set_online_error(error.clone());

                if !online {
                    assert_ne!("", error);
                    assert_eq!(true, *is_error);
                }
            },
            _ => {}
        }
    }

    debug!("joing done, continuing");

    for i in 0..pinger.cpus() {
        pinger.fan_out()[i].send(MessageIn::Quit).unwrap();
    }

    debug!("all done");

    return in_node_vec

}
