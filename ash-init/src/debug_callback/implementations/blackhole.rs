// © ickk 2023-2026, All Rights Reserved.

use {
  crate::debug_callback::{CallbackData, DebugCallback},
  ::ash::vk,
};

/// A [`DebugCallback`] impl that listens for all message types & severities
/// but does nothing with the messages received.
pub struct BlackholeDebugCallback;

impl BlackholeDebugCallback {
  #[expect(clippy::new_ret_no_self)]
  pub fn new() -> Box<dyn DebugCallback> {
    Box::new(BlackholeDebugCallback {})
  }
}

impl DebugCallback for BlackholeDebugCallback {
  fn severities(&self) -> vk::DebugUtilsMessageSeverityFlagsEXT {
    use vk::DebugUtilsMessageSeverityFlagsEXT as F;
    F::ERROR | F::INFO | F::VERBOSE | F::WARNING
  }

  fn types(&self) -> vk::DebugUtilsMessageTypeFlagsEXT {
    use vk::DebugUtilsMessageTypeFlagsEXT as F;
    F::DEVICE_ADDRESS_BINDING | F::GENERAL | F::PERFORMANCE | F::VALIDATION
  }

  fn callback(
    &self,
    _: vk::DebugUtilsMessageSeverityFlagsEXT,
    _: vk::DebugUtilsMessageTypeFlagsEXT,
    _: CallbackData,
  ) {
  }
}
