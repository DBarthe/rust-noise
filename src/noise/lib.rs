#![crate_id = "noise#0.0.0"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

pub use noises::perlin;

pub mod noise;
pub mod noises;