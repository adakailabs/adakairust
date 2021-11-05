use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::JoinHandle;
use std::time::Duration;

use dns_lookup::lookup_host;

use crate::ping::{MessageIn, MessageOut, ping};

/// Pinger holds the internal functionality used for pinging multiple nodes concurrently
#[derive(Debug)]
pub(crate) struct Pinger {
    fan_in: Option<Sender<MessageOut>>,
    fan_out: Vec<Sender<MessageIn>>,
    worker_threads: usize,
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
            worker_threads: num_cpus::get(),
            next_cpu : 0,
            msg_vec: Arc::new(Mutex::new(Vec::new())),
            size,
        };

        if p.worker_threads > 1 {
            p.worker_threads = p.worker_threads * 4
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
    pub fn worker_threads(&mut self) -> usize {
        return self.worker_threads;
    }

    /// next_cpu: used as an iterator for distributing work among all available workers
    pub fn next_worker(&mut self) -> usize {
        self.next_cpu += 1;
        if self.next_cpu == self.worker_threads {
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

        for i in 0..self.worker_threads {
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
            let msg = rx.recv_timeout(Duration::from_secs(120));
            match msg {
                Ok(decoded) => match decoded {
                    MessageIn::Quit => {
                        debug!("msg: QUIT: {:?} --> worker: {:?}", decoded, i);
                        break;
                    }
                    MessageIn::Node { name,port,network_magic, id} => {
                        // debug!("msg: NODE: {} --> worker: {} - {} ",i, name, port);

                        let name_a = name.clone();

                        debug!("ping a: {}", name);
                        let (conn_duration, total_duration, is_error, error) = ping(name_a, port, network_magic); //fixme
                        debug!("ping b: {}", name);

                        let name_copy = name.clone();

                        let valency = valency(name_copy);

                        output.send(MessageOut::Latency {
                            conn_latency: conn_duration,
                            total_latency: total_duration,
                            id,
                            online: !is_error,
                            is_error,
                            error,
                            valency,
                        }).unwrap();
                    }
                },
                Err(e) => {
                    error!("err: {:?} --> worker: {:?}", e, i);

                    output.send(MessageOut::Latency {
                        conn_latency: Duration::from_secs(120),
                        total_latency: Duration::from_secs(120),
                        id: 0,
                        is_error: true,
                        online: false,
                        error: "405".to_string(),
                        valency: 0
                    }).unwrap();
                    break;
                }
            }
        });
        return (tx.clone(),_guard);
    }
}

fn valency(hostname: String) -> u16 {
    let mut valency_count = 0;

    if ipaddress::IPAddress::is_valid(hostname.to_string()) {
        return 1;
    }

    let ips_result = lookup_host(&*hostname);
    match ips_result {
        Ok(resolved_addresses) => {
            for _ in resolved_addresses {
                valency_count += 1;
            }
        }
        Err(_) => {
            valency_count = 0
        }
    }
    return valency_count;

}