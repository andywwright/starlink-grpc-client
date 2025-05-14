// Include the generated protobuf/tonic code exactly once here
include!(concat!(env!("OUT_DIR"), "/mod.rs"));

// Re-export it under `space_x`
// pub use space_x;
pub mod client;
pub mod models;
pub mod error;
