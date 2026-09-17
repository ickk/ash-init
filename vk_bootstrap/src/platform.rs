use crate::{Error, PresentSupport, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
  Win32,
  Xcb,
  Xlib,
  Wayland,
}

impl TryFrom<PresentSupport> for Platform {
  type Error = Error;

  fn try_from(present_support: PresentSupport) -> Result<Self> {
    match present_support {
      PresentSupport::None => Err("No present support".into()),
      PresentSupport::Win32 => Ok(Platform::Win32),
      PresentSupport::Xcb { .. } => Ok(Platform::Xcb),
      PresentSupport::Xlib { .. } => Ok(Platform::Xlib),
      PresentSupport::Wayland { .. } => Ok(Platform::Wayland),
    }
  }
}
