//! This is the main library interface for this project
#![deny(missing_docs)]

mod capability;
mod config;
mod cri_service;
mod criapi;
pub mod error;
mod image_service;
mod network;
mod oci_spec;
mod runtime_service;
mod sandbox;
mod seccomp;
mod server;
mod storage;
mod unix_stream;

pub use config::Config;
pub use server::Server;

#[macro_use]
extern crate bitflags;
