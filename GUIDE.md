```rust
use std::net::{SocketAddr, IpvAddr, Ipv4Addr};
use std::time::Duration;

// shared.rs
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(
  /*host:*/ IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
  /*port:*/ 3000
);

pub const TIMEOUT: Duration = Duration::from_secs(10);
pub const POLL_TIMEOUT: Duration = Duration::from_millis(10);

// client.rs
use nux::prelude::*;
use shared::*;

let mut client = Client::connect(SERVER_ADDR, TIMEOUT).unwrap();

fn message(data: &[u8]) { /* ... */ }
fn close(reason: CloseReason) { /* ... */ }

loop {
  for event in client.poll(POLL_TIMEOUT)? {
    use ClientEvent::*;
    match event? {
      Message(data) => message(data),
      Close(reason) => close(reason),
    }
  }
}

// server.rs
use nux::prelude::*;
use shared::*;

let mut server = Server::bind(SERVER_ADDR).unwrap();

fn message(data: &[u8]) { /* ... */ }
fn open(client: Peer) { /* ... */ }
fn close(client: Peer, reason: CloseReason) { /* ... */ }

loop {
  for event in server.poll(POLL_TIMEOUT)? {
    use ServerEvent::*;
    match event? {
      Message(data) => message(data),
      Open(client) => open(client),
      Close(client, reason) => close(client, reason),
    }
  }
}
```
