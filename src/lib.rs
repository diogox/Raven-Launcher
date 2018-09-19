#![feature(extern_prelude)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typed_builder;

extern crate serde;
extern crate serde_json;
extern crate serde_traitobject;
extern crate webbrowser;
extern crate ws;
extern crate port_scanner;

pub mod api;
pub mod client;
pub mod server;