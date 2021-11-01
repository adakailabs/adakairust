#![warn(missing_docs)]

//! Adakairust is a collection of functions useful for implementing functionality that facilitates
//! the administration of a Cardano Blockchain staking pool.
//!
//! # Summary
//!
//!
//! # Features
//!
//!
//! # Usage
//!
//!
#[macro_use] extern crate log;

/// ping module provides functions for performing a ping to a single cardano node or
/// pinging a vector of nodes
pub mod ping;

/// node module holds all the functions related to the configuration properties of a Cardano node:
/// * address
/// * port
/// * valency
///
/// It also offers certain additional data useful for nodes management and maintenance of a staking
/// pool.
/// * connection latency
/// * online availability
/// (refer to module documentation for all features description)
pub mod node;

/// types moduls holds multiple helper types related to all of the adakairust crate functionality
pub mod types;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
