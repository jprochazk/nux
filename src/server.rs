#![allow(clippy::needless_lifetimes)]

use crate::peer::Address;
use crate::peer::Peer;
use crate::CloseReason;
use std::marker::PhantomData;
use std::time::Duration;

pub struct Server {
  local_addr: Address,
  events: Vec<Event<'static>>,
}

impl Server {
  pub fn bind(addr: Address) -> Result<Self, BindError> {
    Ok(Server {
      local_addr: addr,
      events: Vec::with_capacity(1024),
    })
  }

  pub fn local_addr(&self) -> Address {
    self.local_addr
  }

  pub fn poll<'conn>(
    &'conn mut self,
    timeout: Duration,
  ) -> Result<impl Iterator<Item = Event<'conn>> + 'conn, Error> {
    Ok(self.events.drain(..).map(|e| {
      // bind the lifetime of the event to that of this server, so that the user can't
      // drop the server while the event is still in use. this is one of the
      // requirements of exposing data from the reference-counted buffer.
      unsafe { std::mem::transmute::<Event<'static>, Event<'conn>>(e) }
    }))
  }
}

pub enum Event<'conn> {
  Message(MessageEvent<'conn>),
  Open(OpenEvent),
  Close(CloseEvent),
}

pub struct MessageEvent<'conn> {
  // TODO: `data` should be a pointer to the ref-counted recv buffer
  data: (),
  peer: Peer,
  phantom: PhantomData<&'conn mut ()>,
}

impl<'conn> MessageEvent<'conn> {
  pub fn data(&self) -> &() {
    // TODO: convert the data pointer to a slice
    &self.data
  }

  pub fn peer(&self) -> Peer {
    self.peer
  }
}

pub struct OpenEvent {
  peer: Peer,
}

impl OpenEvent {
  pub fn peer(&self) -> Peer {
    self.peer
  }
}

pub struct CloseEvent {
  peer: Peer,
  reason: CloseReason,
}

impl CloseEvent {
  pub fn peer(&self) -> Peer {
    self.peer
  }

  pub fn reason(&self) -> CloseReason {
    self.reason
  }
}

#[derive(Clone, Debug)]
pub enum BindError {
  AlreadyInUse,
}

impl std::fmt::Display for BindError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "bind address error")
  }
}

impl std::error::Error for BindError {}

#[derive(Clone, Debug)]
pub enum Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "server error")
  }
}

impl std::error::Error for Error {}
