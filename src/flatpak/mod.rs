use std::collections::HashMap;
use zbus::{dbus_proxy, fdo::Result};

#[dbus_proxy(
    interface = "org.freedesktop.portal.Flatpak",
    default_path = "/org/freedesktop/portal/Flatpak"
)]
trait Flatpak {
    /// CreateUpdateMonitor method
    fn create_update_monitor(&self, options: HashMap<&str, zvariant::Value>) -> Result<String>;

    /// Spawn method
    fn spawn(
        &self,
        cwd_path: &[u8],
        argv: &[&[u8]],
        fds: HashMap<u32, std::os::unix::io::RawFd>,
        envs: HashMap<&str, &str>,
        flags: u32,
        options: HashMap<&str, zvariant::Value>,
    ) -> Result<u32>;

    /// SpawnSignal method
    fn spawn_signal(&self, pid: u32, signal: u32, to_process_group: bool) -> Result<()>;

    /// supports property
    #[dbus_proxy(property)]
    fn supports(&self) -> Result<u32>;

    /// version property
    #[dbus_proxy(property)]
    fn version(&self) -> Result<u32>;
}