// © ickk 2023-2026, All Rights Reserved.

use {crate::queue_requirements::QueueCapabilities, ::ash::vk};

#[derive(Clone, Debug)]
pub struct Queue {
  pub(crate) name: Box<str>,
  pub(crate) index: u32,
  pub(crate) family_index: u32,
  pub(crate) priority: f32,
  pub(crate) handle: vk::Queue,
  pub(crate) capabilities: QueueCapabilities,
}

impl Queue {
  #[inline]
  pub fn name(&self) -> &str {
    &self.name
  }

  #[inline]
  pub fn index(&self) -> u32 {
    self.index
  }

  #[inline]
  pub fn family_index(&self) -> u32 {
    self.family_index
  }

  #[inline]
  pub fn priority(&self) -> f32 {
    self.priority
  }

  #[inline]
  pub fn handle(&self) -> vk::Queue {
    self.handle
  }

  #[inline]
  pub fn capabilities(&self) -> &QueueCapabilities {
    &self.capabilities
  }
}
