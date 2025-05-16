#![allow(clippy::large_enum_variant)]

// If we're generating protobuf bindings, skip all library code.
#[cfg(feature = "build-protos")]
// A dummy symbol so that the crate compiles in build-protos mode
pub const _BUILD_PROTOS_ONLY: () = ();

#[cfg(not(feature = "build-protos"))]
include!("../proto_bindings/mod.rs");

// Re-export it under `space_x`
// pub use space_x;
#[cfg(not(feature = "build-protos"))]
pub mod client;
#[cfg(not(feature = "build-protos"))]
pub mod models;
#[cfg(not(feature = "build-protos"))]
pub mod error;
