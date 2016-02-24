extern crate mpd;

use mpd::Client;
use mpd::error::Result;
use mpd::search::Term;
use std::net::TcpStream;
use std::env;

fn connection_string() -> String {
    let host = env::var("MPD_HOST").unwrap_or("localhost".into());
    let port = env::var("MPD_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(6600);
    format!("{}:{}", host, port)
}

pub struct MpdClient {
    pub conn: Client,
}

impl MpdClient {
    pub fn fill_queue(&mut self) {
        let queue = self.conn.queue().unwrap_or(Vec::new());
        let needs = 10 - queue.len();
        if needs > 0 {
            let mut query = self.conn.query();
            // query.and(Term::Tag("Artist".into()), "Tocotronic");
            // query.limit(0, needs as u32);
            let foo = query.find(false, false).err();
            println!("{:?}", foo);
        }
    }
}

pub fn connect() -> Result<MpdClient> {
    let conn_str = connection_string();
    Client::connect(conn_str.as_str()).map(|c| MpdClient {
        conn: c
    })
}

