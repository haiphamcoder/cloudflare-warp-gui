#[derive(Clone, PartialEq)]
pub enum WarpStatus {
    NotInstalled,
    Disconnected,
    Connected,
    Installing,
    Registering,
    Connecting,
    Error(String),
}
