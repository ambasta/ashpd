//! # Examples
//!
//! Access a [`Device`]
//!
//! ```rust,no_run
//! use ashpd::desktop::device::{AccessDeviceOptions, Device, DeviceProxy};
//! use ashpd::{BasicResponse as Basic, RequestProxy, Response};
//! use zbus::fdo::Result;
//!
//! fn main() -> Result<()> {
//!     let connection = zbus::Connection::new_session()?;
//!     let proxy = DeviceProxy::new(&connection)?;
//!     let request = proxy.access_device(6879, &[Device::Speakers], AccessDeviceOptions::default())?;
//!
//!     request.connect_response(|response: Response<Basic>| {
//!         println!("{}", response.is_ok());
//!         Ok(())
//!     })?;
//!     Ok(())
//! }
//! ```
//! [`Device`]: ./enum.Device.html
use crate::{AsyncRequestProxy, HandleToken, RequestProxy};
use serde::{Deserialize, Serialize, Serializer};
use strum_macros::{AsRefStr, EnumString, IntoStaticStr, ToString};
use zbus::{dbus_proxy, fdo::Result};
use zvariant::Signature;
use zvariant_derive::{DeserializeDict, SerializeDict, TypeDict};

#[derive(SerializeDict, DeserializeDict, TypeDict, Clone, Debug, Default)]
/// Specified options for a `access_device` request.
pub struct AccessDeviceOptions {
    /// A string that will be used as the last element of the handle.
    handle_token: Option<HandleToken>,
}

impl AccessDeviceOptions {
    /// Sets the handle token.
    pub fn handle_token(mut self, handle_token: HandleToken) -> Self {
        self.handle_token = Some(handle_token);
        self
    }
}

#[derive(
    Debug, Clone, Copy, Deserialize, EnumString, AsRefStr, IntoStaticStr, ToString, PartialEq, Eq,
)]
#[strum(serialize_all = "lowercase")]
/// The possible device to request access to.
pub enum Device {
    /// A microphone.
    Microphone,
    /// Speakers.
    Speakers,
    /// A Camera.
    Camera,
}

impl zvariant::Type for Device {
    fn signature() -> Signature<'static> {
        String::signature()
    }
}

impl Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&self.to_string(), serializer)
    }
}

#[dbus_proxy(
    interface = "org.freedesktop.portal.Device",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
/// The interface lets services ask if an application should get access to devices such as microphones, speakers or cameras.
/// Not a portal in the strict sense, since the API is not directly accessible to applications inside the sandbox.
trait Device {
    /// Asks for access to a device.
    ///
    /// # Arguments
    ///
    /// * `pid` - The pid of the application on whose behalf the request is made.
    /// * `devices` - A list of devices to request access to.
    /// * `options` - A [`AccessDeviceOptions`].
    ///
    /// [`AccessDeviceOptions`]: ./struct.AccessDeviceOptions.html
    #[dbus_proxy(object = "Request")]
    fn access_device(&self, pid: u32, devices: &[Device], options: AccessDeviceOptions);

    /// The version of this DBus interface.
    #[dbus_proxy(property, name = "version")]
    fn version(&self) -> Result<u32>;
}
