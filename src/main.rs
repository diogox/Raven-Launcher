#![feature(extern_prelude)]
#![allow(dead_code)]

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typed_builder;
#[macro_use]
extern crate downcast_rs;

extern crate gtk;
extern crate relm_core;
extern crate serde;
extern crate serde_json;
extern crate serde_traitobject;
extern crate webbrowser;
extern crate ws;
extern crate port_scanner;
extern crate timer;
extern crate chrono;
extern crate rusqlite;
extern crate gio;
extern crate relm_attributes;
extern crate gdk;
extern crate bindkey;
extern crate fragile;

pub mod api;
pub mod client;
pub mod server;
pub mod search;
pub mod ui;

use relm::Widget;

fn main() {
    ui::windows::launcher::LauncherWindow::run(())
        .expect("Win::run failed");
}