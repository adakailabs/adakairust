use std::error::Error;

use serde::{Deserialize, Serialize};

/// AdakaiResult is a generic result type
pub type AdakaiResult<T> = Result<T, Box<dyn Error>>;

/// MAINNET_MAGIC for cardano main network
pub const MAINNET_MAGIC: u32 = 764824073;

/// TESTNET_MAGIC for cardano test network
pub const TESTNET_MAGIC: u32 = 1097911063;

/// NetworkType holds the two different cardano networks supported
#[derive(Clone, Copy,Serialize, Deserialize, Debug)]
pub enum NetworkType {
    /// Mainnet for cardano
    Mainnet,

    /// Testnet for cardano
    TestNet,
}

/// NodeType holds the two different cardano node types used in a cardano staking pool
#[derive(Clone, Copy,Serialize, Deserialize, Debug)]
pub enum NodeType {
    /// Relay node
    Relay,

    /// Producer node
    Producer,
}


impl Default for NetworkType {
    fn default() -> Self { NetworkType::Mainnet }
}

impl Default for NodeType {
    fn default() -> Self { NodeType::Relay }
}