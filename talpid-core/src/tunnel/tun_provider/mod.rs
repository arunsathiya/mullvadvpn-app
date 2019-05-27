use std::net::IpAddr;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
use talpid_types::BoxedError;

mod stub;
pub use self::stub::StubTunProvider;

/// Generic tunnel device.
///
/// Must be associated with a platform specific file descriptor representing the device.
#[cfg(unix)]
pub trait Tun: AsRawFd + Send {
    /// Retrieve the tunnel interface name.
    fn interface_name(&self) -> &str;
}

/// Stub tunnel device.
#[cfg(windows)]
pub trait Tun: Send {
    /// Retrieve the tunnel interface name.
    fn interface_name(&self) -> &str;
}

/// Factory of tunnel devices.
pub trait TunProvider: Send + 'static {
    /// Create a tunnel device using the provided configuration.
    fn create_tun(&self, config: TunConfig) -> Result<Box<dyn Tun>, BoxedError>;
}

/// Configuration for creating a tunnel device.
pub struct TunConfig {
    /// IP addresses for the tunnel interface.
    pub addresses: Vec<IpAddr>,
}

impl TunConfig {
    /// Create a new tunnel device configuration using the specified tunnel addresses.
    pub fn new(addresses: impl IntoIterator<Item = IpAddr>) -> Self {
        TunConfig {
            addresses: addresses.into_iter().collect(),
        }
    }
}
