use std::collections::{HashMap, HashSet};
use std::error::Error;

use prettytable::{cell, row, Table};
use serde::{Deserialize, Serialize};

use crate::{ping, wget};
use crate::node::Node;
use crate::types::NetworkType;

mod topology_tests;

#[allow(dead_code)]
const TYPE_TESTNET: bool = false;

pub type TopologyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize)]
pub(crate) struct Topology {
    #[serde(rename = "Producers")]
    producers: Vec<Node>,

    #[serde(default)]
    map: HashMap<String, usize>,

    #[serde(default)]
    set: HashSet<String>,

    #[serde(default)]
    network_type: NetworkType,

}

impl Topology {
    #[allow(dead_code)]
    pub fn producers(&self) -> &Vec<Node> {
        &self.producers
    }
}

impl Topology {
    #[allow(dead_code)]
    pub fn new_from_json(network_type: NetworkType, json: String) -> TopologyResult<Topology> {
        let top_result: serde_json::Result<Topology> = serde_json::from_str(&json);

        match top_result {
            Ok(mut topology) => {
                topology.network_type = network_type;
                topology.map = HashMap::new();

                for (i, node) in topology.producers.iter_mut().enumerate() {
                    node.set_network_type(network_type);
                    let node_name = format!("{}_{}", node.addr(), node.port());
                    topology.map.entry(node_name).or_insert(i);
                    topology.set.insert(node.addr().to_string());
                }
                return Ok(topology);
            }
            Err(e) => TopologyResult::Err(Box::try_from(e).unwrap()),
        }
    }

    pub fn new_from_online_peers(network_type: NetworkType) -> TopologyResult<Topology> {
        let json:Vec<u8> ; //= Vec::new();

        match network_type {
            NetworkType::Mainnet => {
                json = wget::mainnet_topology_all_peers();
            }
            NetworkType::TestNet => {
                json = wget::testnet_topology_all_peers();
            }
        }

        let convert_result = String::from_utf8(json);

        match convert_result {
            Ok(json) => {
                return Topology::new_from_json(network_type, json);
            }
            Err(e) => TopologyResult::Err(Box::try_from(e).unwrap()),
        }
    }

    pub fn ping(&mut self) {
        let new_producers =
            ping::ping_vec(self.producers.clone(), self.network_type);
        self.producers = new_producers;
    }

    pub fn resolve_valencies(&mut self) {
        for node in self.producers.iter_mut() {
            node.resolve_valency();
        }
    }

    pub fn sort(&mut self) {
        self.producers.sort_by(|b, a| b.total_latency().as_millis().cmp(&a.total_latency().as_millis()));
    }

    pub fn pretty_print(&self) {
        let mut table = Table::new();
        table.add_row(row!["ID", "addr", "port", "valency", "conn_latency", "total latency", "online"]);

        for (i, node) in self.producers().iter().enumerate() {
            let addr = node.addr();
            let port = node.port().to_string();
            let total_latency = node.total_latency().as_millis().to_string();
            let con_latency = node.con_latency().as_millis().to_string();
            let valency = node.valency();
            let mut online = "YES";
            if !node.online() {
                online = "NO"
            }

            table.add_row(row![i, addr, port ,valency, con_latency, total_latency, online]);
        }
        table.printstd();
    }

}
