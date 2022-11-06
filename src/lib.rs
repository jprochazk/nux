pub mod client;
pub mod peer;
pub mod server;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseReason {
  /// The peer's protocol version did not ours.
  VersionMismatch,
  /// The peer sent a malformed packet.
  MalformedPacket,
  /// The peer has not sent any packets for too long.
  TimedOut,
}

impl std::fmt::Display for CloseReason {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CloseReason::VersionMismatch => write!(f, "peer's protocol version did not match ours"),
      CloseReason::MalformedPacket => write!(f, "peer sent a malformed packet"),
      CloseReason::TimedOut => write!(
        f,
        "too much time has elapsed since the last time the peer sent a packet"
      ),
    }
  }
}
