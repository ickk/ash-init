// © ickk 2023-2026, All Rights Reserved.

use {crate::PresentSupport, ::ash::vk};

#[derive(Clone, Debug)]
pub struct QueueRequirements<'name> {
  pub names: Vec<&'name str>,
  pub priorities: Vec<f32>,
  pub capabilities: QueueCapabilities,
  pub present_support: Option<PresentSupport>,
}

impl<'name> QueueRequirements<'name> {
  pub const NONE: Self = Self {
    names: Vec::new(),
    priorities: Vec::new(),
    capabilities: QueueCapabilities::NONE,
    present_support: None,
  };

  #[inline]
  pub fn new() -> Self {
    Self::NONE
  }

  #[inline]
  pub fn names(mut self, names: Vec<&'name str>) -> Self {
    self.names = names;
    self
  }

  #[inline]
  pub fn priorities(mut self, priorities: &[f32]) -> Self {
    self.priorities.clear();
    self.priorities.extend_from_slice(priorities);
    self
  }

  #[inline]
  pub fn capabilities(mut self, capabilities: QueueCapabilities) -> Self {
    self.capabilities = capabilities;
    self
  }

  #[inline]
  pub fn present_support(
    mut self,
    presentation_support: Option<PresentSupport>,
  ) -> Self {
    self.present_support = presentation_support;
    self
  }
}

impl Default for QueueRequirements<'_> {
  #[inline]
  fn default() -> Self {
    Self::NONE
  }
}

#[derive(Clone, Debug, Copy)]
pub struct QueueCapabilities {
  pub graphics: bool,
  pub compute: bool,
  pub transfer: bool,
  pub sparse_binding: bool,
  pub protected: bool,
  pub video_decode_khr: bool,
  pub video_encode_khr: bool,
  pub optical_flow_nv: bool,
}

impl From<QueueCapabilities> for vk::QueueFlags {
  fn from(capabilities: QueueCapabilities) -> Self {
    let mut flags = vk::QueueFlags::default();
    if capabilities.graphics {
      flags |= vk::QueueFlags::GRAPHICS;
    }
    if capabilities.compute {
      flags |= vk::QueueFlags::COMPUTE;
    }
    if capabilities.transfer {
      flags |= vk::QueueFlags::TRANSFER;
    }
    if capabilities.sparse_binding {
      flags |= vk::QueueFlags::SPARSE_BINDING;
    }
    if capabilities.protected {
      flags |= vk::QueueFlags::PROTECTED;
    }
    if capabilities.video_decode_khr {
      flags |= vk::QueueFlags::VIDEO_DECODE_KHR;
    }
    if capabilities.video_encode_khr {
      flags |= vk::QueueFlags::VIDEO_ENCODE_KHR;
    }
    if capabilities.optical_flow_nv {
      flags |= vk::QueueFlags::OPTICAL_FLOW_NV;
    }
    flags
  }
}

impl From<vk::QueueFlags> for QueueCapabilities {
  fn from(flags: vk::QueueFlags) -> Self {
    QueueCapabilities {
      graphics: flags.contains(vk::QueueFlags::GRAPHICS),
      compute: flags.contains(vk::QueueFlags::COMPUTE),
      transfer: flags.contains(vk::QueueFlags::TRANSFER),
      sparse_binding: flags.contains(vk::QueueFlags::SPARSE_BINDING),
      protected: flags.contains(vk::QueueFlags::PROTECTED),
      video_decode_khr: flags.contains(vk::QueueFlags::VIDEO_DECODE_KHR),
      video_encode_khr: flags.contains(vk::QueueFlags::VIDEO_ENCODE_KHR),
      optical_flow_nv: flags.contains(vk::QueueFlags::OPTICAL_FLOW_NV),
    }
  }
}

use crate::macros::delegate_builder_setters;

impl QueueCapabilities {
  pub const NONE: Self = Self {
    graphics: false,
    compute: false,
    transfer: false,
    sparse_binding: false,
    protected: false,
    video_decode_khr: false,
    video_encode_khr: false,
    optical_flow_nv: false,
  };

  #[inline]
  pub fn new() -> Self {
    Self::NONE
  }

  delegate_builder_setters! {
    to self {
      pub fn graphics(bool);
      pub fn compute(bool);
      pub fn transfer(bool);
      pub fn sparse_binding(bool);
      pub fn protected(bool);
      pub fn video_decode_khr(bool);
      pub fn video_encode_khr(bool);
      pub fn optical_flow_nv(bool);
    }
  }
}

impl Default for QueueCapabilities {
  #[inline]
  fn default() -> Self {
    Self::NONE
  }
}
