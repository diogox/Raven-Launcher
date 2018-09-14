#![feature(extern_prelude)]
#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_traitobject;
extern crate webbrowser;

pub mod api;
pub mod client;
pub mod server;