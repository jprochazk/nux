#![allow(clippy::needless_lifetimes)]

use crate::peer::Address;
use crate::CloseReason;
use std::marker::PhantomData;
use std::time::Duration;

pub struct Client {
  remote_addr: Address,
  events: Vec<Event<'static>>,
}

impl Client {
  pub fn connect(addr: Address, timeout: Duration) -> Result<Self, ConnectError> {
    Ok(Client {
      remote_addr: addr,
      events: Vec::with_capacity(16),
    })
  }

  pub fn remote_addr(&self) -> Address {
    self.remote_addr
  }

  pub fn poll<'conn>(
    &'conn mut self,
    timeout: Duration,
  ) -> Result<impl Iterator<Item = Event<'conn>> + 'conn, Error> {
    Ok(self.events.drain(..).map(|e| {
      // bind the lifetime of the event to that of this client,
      // so that the user can't drop the client while the event
      // is still in use.
      // this is one of the requirements of exposing data from the reference-counted
      // buffer.
      unsafe { std::mem::transmute::<Event<'static>, Event<'conn>>(e) }
    }))
  }

  pub fn send(&mut self, data: &()) {
    todo!()
  }
}

pub enum Event<'conn> {
  Message(MessageEvent<'conn>),
  Close(CloseEvent),
}

pub struct MessageEvent<'conn> {
  // TODO: `data` should be a pointer to the ref-counted recv buffer
  data: (),
  phantom: PhantomData<&'conn ()>,
}

impl<'conn> MessageEvent<'conn> {
  pub fn data(&self) -> &() {
    // TODO: convert the data pointer to a slice
    &self.data
  }
}

pub struct CloseEvent {
  reason: CloseReason,
}

impl CloseEvent {
  pub fn reason(&self) -> CloseReason {
    self.reason
  }
}

#[derive(Clone, Debug)]
pub enum ConnectError {
  ConnectionFailure,
}

impl std::fmt::Display for ConnectError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "connect error")
  }
}

impl std::error::Error for ConnectError {}

#[derive(Clone, Debug)]
pub enum Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "client error")
  }
}

impl std::error::Error for Error {}
