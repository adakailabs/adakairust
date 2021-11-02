use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::types::{AdakaiResult, NetworkType, NodeType};

///
mod node_tests;

/// Node contains data for describing a cardano node configuration:
/// * addr
/// * port
/// * continent
/// * velency
///
/// Additional to normally available configuration fields it can hold information useful for
/// better managing chain nodes:
/// * connection latency
/// * round trip latency
/// * node type (testnet or mainnet, potencially others)
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Node {
    addr: String,
    port: u16,

    #[serde(default)]
    continent: String,

    #[serde(default)]
    state: String,

    #[serde(default)]
    valency: u16,

    #[serde(default)]
    con_latency: Duration,

    #[serde(default)]
    total_latency: Duration,

    #[serde(default)]
    network_type: NetworkType,

    #[serde(default)]
    node_type: NodeType,


    #[serde(default)]
    online: bool,

    #[serde(default)]
    online_error: String,

}

impl Node {
    /// set_addr: sets the cardano node IP or valid DNS address
    #[allow(dead_code)]
    pub fn set_addr(&mut self, addr: String) {
        self.addr = addr;
    }

    /// set_port: sets the cardano node TCP port
    #[allow(dead_code)]
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// set_continent: sets the continent where the node is physically located
    #[allow(dead_code)]
    pub fn set_continent(&mut self, continent: String) {
        self.continent = continent;
    }

    /// set_state: sets the state where the node is physically located
    #[allow(dead_code)]
    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    /// set_valency:  sets the node valency
    #[allow(dead_code)]
    pub fn set_valency(&mut self, valency: u16) {
        self.valency = valency;
    }

    /// set_con_latency:  sets the node connection latency (see ping module for available functions
    /// for calculating this latency)
    #[allow(dead_code)]
    pub fn set_con_latency(&mut self, latency: Duration) {
        self.con_latency = latency;
    }

    /// set_total_latency:  sets the node connection latency (see ping module for available functions
    /// for calculating this latency)
    #[allow(dead_code)]
    pub fn set_total_latency(&mut self, latency: Duration) {
        self.total_latency = latency;
    }

    /// set_online: set online status
    #[allow(dead_code)]
    pub fn set_online(&mut self, online: bool) {
        self.online = online;
    }

    /// set_online_error: sets the error that explains non-online status
    #[allow(dead_code)]
    pub fn set_online_error(&mut self, online_error: String) {
        self.online_error = online_error;
    }

    /// set_node_type:: sets type of node, (RELAY or PRODUCER)
    #[allow(dead_code)]
    pub fn set_network_type(&mut self, ntype: NetworkType) {
        self.network_type = ntype;
    }

    /// addr: returns the IP address or DNS name
    #[allow(dead_code)]
    pub fn addr(&self) -> &str {
        &self.addr
    }

    /// port: returns the TCP port used by the node
    #[allow(dead_code)]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// continent: returns the continent the node is physically located at
    /// default: ""
    #[allow(dead_code)]
    pub fn continent(&self) -> &str {
        &self.continent
    }

    /// continent: returns the state the node is physically located at
    /// /// default: ""
    #[allow(dead_code)]
    pub fn state(&self) -> &str {
        &self.state
    }

    /// valency: returns the valency of the node
    /// default: 1
    #[allow(dead_code)]
    pub fn valency(&self) -> u16 {
        self.valency
    }

    /// con_latency: returns the connection latency is previously set
    #[allow(dead_code)]
    pub fn con_latency(&self) -> Duration {
        self.con_latency
    }

    /// total_latency: returns the total latency is previously set
    #[allow(dead_code)]
    pub fn total_latency(&self) -> Duration {
        self.total_latency
    }

    /// online: returns the online state, true for online, false for offline.
    /// default: offline
    #[allow(dead_code)]
    pub fn online(&self) -> bool {
        self.online
    }

    /// online_error: returns the error associated to the online status.
    #[allow(dead_code)]
    pub fn online_error(&self) -> String {
        self.online_error.clone()
    }

    /// **network_type**: returns the cardano network type node (TESTNET or MAINNET)
    #[allow(dead_code)]
    pub fn network_type(&self) -> NetworkType {
        self.network_type
    }

    /// **node_type**: returns the cardano node type (relay or producer)
    #[allow(dead_code)]
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    /// new_from_json:  takes a json encoded string and deserializes it into a Node struct.
    /// # Arguments:
    /// - **network_type**: TESTNET or MAINNET type.
    /// - **json**: a json encoded string with the node configuration as deliver by the official cardano explorer
    /// - **example**:
    ///  ``` [json]
    /// {
    ///  "addr": "costa-rica.adakailabs.com",
    ///  "port": 5000,
    ///  "continent": "North America",
    ///  "state": "CR"
    ///  }
    /// ```
    pub fn new_from_json(network_type: NetworkType, json: String) -> AdakaiResult<Node> {
        let top_result: serde_json::Result<Node> = serde_json::from_str(&json);
        match top_result {
            Ok(mut node) => {
                node.network_type = network_type;
                return Ok(node);
            }
            Err(e) => AdakaiResult::Err(Box::try_from(e).unwrap()),
        }
    }
}
