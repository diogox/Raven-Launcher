#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate serde;
extern crate webbrowser;

pub mod api;
pub mod client;
pub mod server;