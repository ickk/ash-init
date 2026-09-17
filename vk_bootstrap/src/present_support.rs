use ::core::{
  ffi::{c_ulong, c_void},
  ptr::NonNull,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PresentSupport {
  #[default]
  None,
  Win32,
  Xcb {
    connection: NonNull<c_void>,
    visual_id: u32,
  },
  Xlib {
    display: NonNull<c_void>,
    visual_id: c_ulong,
  },
  Wayland,
}

impl From<Option<PresentSupport>> for PresentSupport {
  fn from(option: Option<PresentSupport>) -> PresentSupport {
    if let Some(present_support) = option {
      present_support
    } else {
      PresentSupport::None
    }
  }
}

impl PresentSupport {
  pub fn into_option(
    present_support: PresentSupport,
  ) -> Option<PresentSupport> {
    if let PresentSupport::None = present_support {
      None
    } else {
      Some(present_support)
    }
  }
}
