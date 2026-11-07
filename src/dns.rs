#[cfg(any(target_os = "macos", all(test, unix)))]
mod macos;
#[cfg(any(target_os = "linux", all(test, unix)))]
mod linux;
#[cfg(unix)]
mod state;

#[cfg(target_os = "macos")]
pub use macos::DNSManager;
#[cfg(target_os = "linux")]
pub use linux::DNSManager;

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub struct DNSManager;
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
impl DNSManager {
    pub fn new(_: Option<String>) -> Self { Self }
    pub fn set_dns(&mut self, _: Vec<&str>, _: Vec<&str>) -> anyhow::Result<()> { Ok(()) }
    pub fn restore_dns(&self) -> anyhow::Result<()> { Ok(()) }
}
