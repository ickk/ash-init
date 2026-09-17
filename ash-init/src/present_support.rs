use {
  crate::error::Result,
  ::core::{
    ffi::{c_ulong, c_void},
    ptr::NonNull,
  },
  ::raw_window_handle::{
    HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    XcbDisplayHandle, XcbWindowHandle, XlibDisplayHandle, XlibWindowHandle,
  },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
  Win32,
  Xcb,
  Xlib,
  // Wayland,
}

impl From<PresentSupport> for Platform {
  fn from(present_support: PresentSupport) -> Self {
    match present_support {
      PresentSupport::Win32 => Platform::Win32,
      PresentSupport::Xcb { .. } => Platform::Xcb,
      PresentSupport::Xlib { .. } => Platform::Xlib,
      // PresentSupport::Wayland { .. } => Platform::Wayland,
    }
  }
}

pub trait HasWindowHandleExt: HasWindowHandle + HasDisplayHandle {
  fn platform(&self) -> crate::Result<crate::Platform> {
    use crate::Platform as P;
    use RawWindowHandle as R;

    let window_handle = self
      .window_handle()
      .map_err(|err| err.to_string())?
      .as_raw();

    Ok(match window_handle {
      R::Win32(_) => P::Win32,
      // R::Wayland(_) => P::Wayland,
      R::Xcb(_) => P::Xcb,
      R::Xlib(_) => P::Xlib,
      _ => {
        return Err(
          format!("Platform: {window_handle:?} not supported").into(),
        )
      },
    })
  }
}
impl<W> HasWindowHandleExt for W where W: HasWindowHandle + HasDisplayHandle {}

pub trait HasDisplayHandleExt: HasDisplayHandle {
  fn platform(&self) -> crate::Result<crate::Platform> {
    use crate::Platform as P;
    use RawDisplayHandle as R;

    let display_handle = self.display_handle()?.as_raw();

    Ok(match display_handle {
      R::Windows(_) => P::Win32,
      // R::Wayland(_) => P::Wayland,
      R::Xcb(_) => P::Xcb,
      R::Xlib(_) => P::Xlib,
      _ => {
        return Err(
          format!("Platform: {display_handle:?} not supported").into(),
        )
      },
    })
  }
}
impl<D> HasDisplayHandleExt for D where D: HasDisplayHandle {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PresentSupport {
  Win32,
  Xcb {
    connection: NonNull<c_void>,
    visual_id: u32,
  },
  Xlib {
    display: NonNull<c_void>,
    visual_id: c_ulong,
  },
  // Wayland,
}

impl PresentSupport {
  pub fn from_window(
    window: &impl HasWindowHandleExt,
  ) -> Result<PresentSupport> {
    let platform = HasWindowHandleExt::platform(window)?;
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
      // _ => return Err(format!("unsupported platform: {platform:?}").into()),
    })
  }

  pub fn from_display(
    display: &impl HasDisplayHandle,
  ) -> Result<PresentSupport> {
    let platform = display.platform()?;
    let display_handle = display.display_handle()?.as_raw();

    match (platform, display_handle) {
      (Platform::Win32, RawDisplayHandle::Windows(_)) => {
        Ok(PresentSupport::Win32)
      },
      #[cfg(all(feature = "xcb", not(target_os = "windows")))]
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
      #[cfg(all(feature = "xlib", not(target_os = "windows")))]
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
