use std::collections::{HashMap, HashSet};
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::node::Node;
use crate::types::NetworkType;
use crate::wget;

mod topology_tests;

#[allow(dead_code)]
const TYPE_TESTNET: bool = false;

pub(crate) type TopologyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize)]
pub(crate) struct Topology {
    #[serde(rename = "Producers")]
    producers: Vec<Node>,

    #[serde(default)]
    map: HashMap<String, usize>,

    #[serde(default)]
    set: HashSet<String>,
}

impl Topology {
    pub fn producers(&self) -> &Vec<Node> {
        &self.producers
    }
}

impl Topology {
    pub fn new_from_json(node_type: NetworkType, json: String) -> TopologyResult<Topology> {
        let top_result: serde_json::Result<Topology> = serde_json::from_str(&json);

        match top_result {
            Ok(mut topology) => {
                topology.map = HashMap::new();

                for (i, node) in topology.producers.iter_mut().enumerate() {
                    node.set_network_type(node_type);
                    let node_name = format!("{}_{}", node.addr(), node.port());
                    topology.map.entry(node_name).or_insert(i);
                    topology.set.insert(node.addr().to_string());
                }
                return Ok(topology);
            }
            Err(e) => TopologyResult::Err(Box::try_from(e).unwrap()),
        }
    }

    pub fn new_from_online_peers(node_type: NetworkType) -> TopologyResult<Topology> {
        let json = wget::mainnet_topology_all_peers();

        let convert_result = String::from_utf8(json);

        match convert_result {
            Ok(json) => {
                return Topology::new_from_json(node_type, json);
            }
            Err(e) => TopologyResult::Err(Box::try_from(e).unwrap()),
        }
    }
}
