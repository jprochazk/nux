#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Peer {
  addr: Address,
}

impl Peer {
  pub(crate) fn new(addr: Address) -> Self {
    Self { addr }
  }

  pub fn addr(&self) -> Address {
    self.addr
  }
}

impl std::fmt::Debug for Peer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Peer").field("addr", &self.addr).finish()
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(pub std::net::SocketAddr);

impl Address {
  pub fn localhost(port: u16) -> Self {
    Self(std::net::SocketAddr::new(
      std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
      port,
    ))
  }

  pub fn any(port: u16) -> Self {
    Self(std::net::SocketAddr::new(
      std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
      port,
    ))
  }
}

impl From<std::net::SocketAddr> for Address {
  fn from(v: std::net::SocketAddr) -> Self {
    Self(v)
  }
}

impl std::fmt::Debug for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Address")
      .field("ip", &self.0.ip())
      .field("port", &self.0.port())
      .finish()
  }
}

impl std::fmt::Display for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.0.ip(), self.0.port())
  }
}
