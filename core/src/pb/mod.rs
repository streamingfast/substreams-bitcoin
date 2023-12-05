//! This module contains the protobuf generated code for the Substreams Bitcoin block
//! model.
//!
//! This is the raw Protbuf code, types in here can be used without problem.

mod generated;

/// Re-export the protobuf generated code directly, at some point we might
pub mod sf {
    pub use crate::pb::generated::sf::*;
}

pub mod btc {
    pub mod v1 {
        pub use crate::pb::generated::sf::bitcoin::r#type::v1::*;
    }
}
