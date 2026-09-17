use ::std::num::ParseIntError;

#[derive(Copy, Clone, Debug)]
pub struct Version {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
}

impl From<(u32, u32, u32)> for Version {
  fn from((major, minor, patch): (u32, u32, u32)) -> Self {
    Version {
      major,
      minor,
      patch,
    }
  }
}

impl Version {
  pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
    Version {
      major,
      minor,
      patch,
    }
  }
}

impl TryFrom<(&str, &str, &str)> for Version {
  type Error = ParseIntError;

  fn try_from(
    (major, minor, patch): (&str, &str, &str),
  ) -> ::core::result::Result<Self, ParseIntError> {
    Ok(Version::new(major.parse()?, minor.parse()?, patch.parse()?))
  }
}

pub struct StrVersion<'s>(pub &'s str, pub &'s str, pub &'s str);

impl From<StrVersion<'_>> for Version {
  fn from(StrVersion(major, minor, patch): StrVersion) -> Self {
    Version::try_from((major, minor, patch)).unwrap()
  }
}

pub trait VersionToVk {
  fn to_vk_api_version(self) -> u32;
}

impl VersionToVk for Version {
  fn to_vk_api_version(self) -> u32 {
    ::ash::vk::make_api_version(0, self.major, self.minor, self.patch)
  }
}

use {
  crate::raw_window_handle::{
    DisplayHandlePlatform as _, HasDisplayAndWindowHandle,
  },
  crate::{Platform, PresentSupport, Result},
  ::core::ffi::c_ulong,
  ::raw_window_handle::{
    HasDisplayHandle, RawDisplayHandle, RawWindowHandle, XcbDisplayHandle,
    XcbWindowHandle, XlibDisplayHandle, XlibWindowHandle,
  },
};

pub trait PresentSupportFromRawWindowHandle {
  fn from_window(
    window: &impl HasDisplayAndWindowHandle,
  ) -> Result<PresentSupport> {
    let platform = window.platform()?;
    let window_handle = window
      .window_handle()
      .map_err(|_| "window handle not available or unsupported")?
      .as_raw();
    let display_handle = window
      .display_handle()
      .map_err(|_| "display handle not available or unsupported")?
      .as_raw();

    let (mut xcb_display, mut xcb_visual_id) = (None, None);
    let (mut xlib_display, mut xlib_visual_id) = (None, None);
    match (platform, display_handle, window_handle) {
      (
        Platform::Xcb,
        RawDisplayHandle::Xcb(XcbDisplayHandle {
          connection: Some(ptr),
          ..
        }),
        RawWindowHandle::Xcb(XcbWindowHandle {
          visual_id: Some(visual_id),
          ..
        }),
      ) => {
        xcb_display = Some(ptr);
        xcb_visual_id = Some(visual_id.get())
      },
      (
        Platform::Xlib,
        RawDisplayHandle::Xlib(XlibDisplayHandle {
          display: Some(display),
          ..
        }),
        RawWindowHandle::Xlib(XlibWindowHandle { visual_id, .. }),
      ) => {
        xlib_display = Some(display);
        xlib_visual_id = Some(visual_id as c_ulong);
      },
      _ => (),
    }

    Ok(match platform {
      Platform::Win32 => PresentSupport::Win32,
      Platform::Xcb => PresentSupport::Xcb {
        connection: xcb_display.unwrap(),
        visual_id: xcb_visual_id.unwrap(),
      },
      Platform::Xlib => PresentSupport::Xlib {
        display: xlib_display.unwrap(),
        visual_id: xlib_visual_id.unwrap(),
      },
      _ => return Err(format!("unsupported platform: {platform:?}").into()),
    })
  }

  fn from_display(display: &impl HasDisplayHandle) -> Result<PresentSupport> {
    let platform = display.platform()?;
    let display_handle = display.display_handle()?.as_raw();

    match (platform, display_handle) {
      (Platform::Win32, RawDisplayHandle::Windows(_)) => {
        Ok(PresentSupport::Win32)
      },
      #[cfg(all(
        feature = "common-present-support-xcb",
        not(target_os = "windows")
      ))]
      (
        Platform::Xcb,
        RawDisplayHandle::Xcb(XcbDisplayHandle {
          connection: connection_t,
          screen,
          ..
        }),
      ) => {
        let connection_t = connection_t.expect("connection was null");

        let visual_id = {
          let connection = unsafe {
            let connection_t_raw: *mut ::xcb::ffi::xcb_connection_t =
              connection_t.cast().as_ptr();

            ::xcb::Connection::from_raw_conn(connection_t_raw)
          };

          let screen =
            connection.get_setup().roots().nth(screen as usize).unwrap();

          screen.root_visual()
        };

        Ok(PresentSupport::Xcb {
          connection: connection_t,
          visual_id,
        })
      },
      #[cfg(all(
        feature = "common-present-support-xlib",
        not(target_os = "windows")
      ))]
      (
        Platform::Xlib,
        RawDisplayHandle::Xlib(XlibDisplayHandle {
          display, screen, ..
        }),
      ) => {
        let display = display.expect("display was null");

        let visual_id = unsafe {
          let display_raw: *mut ::x11::xlib::Display = display.cast().as_ptr();
          let default_visual =
            ::x11::xlib::XDefaultVisual(display_raw, screen as _);

          ::x11::xlib::XVisualIDFromVisual(default_visual)
        };

        Ok(PresentSupport::Xlib { display, visual_id })
      },
      _ => Err(
        format!("unsupported platform: {platform:?} {display_handle:?}")
          .into(),
      ),
    }
  }
}
impl PresentSupportFromRawWindowHandle for PresentSupport {}
