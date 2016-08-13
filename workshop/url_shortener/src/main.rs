extern crate rustc_serialize;
extern crate rusqlite;
extern crate time;
#[macro_use]
extern crate nickel;

mod server;

use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::Newline::*;
use rustc_serialize::base64::CharacterSet::*;
use rustc_serialize::base64::Config;

use rusqlite::SqliteConnection;
use time::Timespec;
use server::start_server;

//
// DATA MODELS
//
#[derive(Debug)]
struct Link {
    id: u64,
    target: String,
    use_count: u64,
    created_at: Timespec
}

#[derive(Debug)]
struct LinkUse {
    id: u64,
    link_id: u64,
    browser_info: String,
    created_at: Timespec
}

//
// WEB SERVER
//
fn main() {
    let c = Config {
        char_set: UrlSafe,
        newline: LF,
        pad: false,
        line_length: None
    };
    for s in 0..255 {
        println!("B64: {:?}", [s].to_base64(c));
    }

    start_server();
}
