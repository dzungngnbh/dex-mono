#![feature(lazy_cell)]

pub mod constants;
pub mod orderbook;
pub mod subaccount;
pub mod utils;

// capnp
pub mod pubsub_capnp {
    include!(concat!(env!("OUT_DIR"), "/pubsub_capnp.rs"));
}

pub mod pubsub2_capnp {
    include!(concat!(env!("OUT_DIR"), "/pubsub2_capnp.rs"));
}
