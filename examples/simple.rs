use std::time::Duration;

use nux::client;
use nux::client::Client;
use nux::peer::Address;

fn handle(data: &()) {
  todo!()
}

fn main() {
  let addr = Address::localhost(3000);
  let mut client = Client::connect(addr, Duration::from_secs(10)).unwrap();

  loop {
    for event in client.poll(Duration::from_millis(10)).unwrap() {
      use client::Event::*;
      match event {
        Message(msg) => {
          handle(msg.data());
        }
        Close(_) => todo!(),
      }
    }

    client.send(&());
  }
}
