extern crate rustty;
extern crate mpd;

mod mpd_client;

fn main() {
    let mut client = mpd_client::connect().unwrap();
    client.conn.volume(50).unwrap();
    client.fill_queue();

    client.conn.switch(1).unwrap();
    client.conn.play().unwrap();
    println!("Status: {:?}", client.conn.status());
}
