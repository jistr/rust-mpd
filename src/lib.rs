#![feature(slicing_syntax, unsafe_destructor)]
#![feature(old_orphan_check)]
#![allow(unstable)]

extern crate libc;
extern crate time;
extern crate "rustc-serialize" as rustc_serialize;

pub mod client;
mod utils;
pub mod error;
pub mod queue;
pub mod status;
pub mod stats;
pub mod outputs;
pub mod songs;
pub mod playlists;
//pub mod idle;

