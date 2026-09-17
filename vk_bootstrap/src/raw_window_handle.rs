use ::raw_window_handle::{
  HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
};

#[rustfmt::skip]
pub trait HasDisplayAndWindowHandle: HasDisplayHandle + HasWindowHandle {}
impl<W: HasDisplayHandle + HasWindowHandle> HasDisplayAndWindowHandle for W {}

pub trait WindowHandlePlatform: HasWindowHandle {
  fn platform(&self) -> crate::Result<crate::Platform> {
    use crate::Platform as P;
    use RawWindowHandle as R;

    let window_handle = self
      .window_handle()
      .map_err(|err| err.to_string())?
      .as_raw();

    Ok(match window_handle {
      R::Win32(_) => P::Win32,
      R::Wayland(_) => P::Wayland,
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
impl<W> WindowHandlePlatform for W where W: HasWindowHandle {}

pub trait DisplayHandlePlatform: HasDisplayHandle {
  fn platform(&self) -> crate::Result<crate::Platform> {
    use crate::Platform as P;
    use RawDisplayHandle as R;

    let display_handle = self.display_handle()?.as_raw();

    Ok(match display_handle {
      R::Windows(_) => P::Win32,
      R::Wayland(_) => P::Wayland,
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
impl<D> DisplayHandlePlatform for D where D: HasDisplayHandle {}
