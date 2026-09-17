use {
  crate::debug::*,
  ::ash::vk,
  ::core::{cell::RefCell, fmt::Write as _},
  ::itertools::Itertools,
};

/// A [`DebugCallback`] impl that logs messages it receives using [`log`].
pub struct LogDebugCallback {
  message_severities: vk::DebugUtilsMessageSeverityFlagsEXT,
  message_types: vk::DebugUtilsMessageTypeFlagsEXT,
}

impl LogDebugCallback {
  /// Create a new `LogDebugCallback` that listens for messages matching the
  /// specified severities and types.
  ///
  /// You can use [`MessageSeverityFlags`][crate::debug::MessageSeverityFlags]
  /// and [`MessageTypeFlags`][crate::debug::MessageTypeFlags] to populate
  /// these arguments instead of `vk::DebugUtilsMessage*FlagsEXT`.
  pub fn new(
    message_severities: impl Into<vk::DebugUtilsMessageSeverityFlagsEXT>,
    message_types: impl Into<vk::DebugUtilsMessageTypeFlagsEXT>,
  ) -> Box<dyn DebugCallback> {
    Box::new(LogDebugCallback {
      message_severities: message_severities.into(),
      message_types: message_types.into(),
    })
  }
}

impl DebugCallback for LogDebugCallback {
  fn severities(&self) -> vk::DebugUtilsMessageSeverityFlagsEXT {
    self.message_severities
  }

  fn types(&self) -> vk::DebugUtilsMessageTypeFlagsEXT {
    self.message_types
  }

  #[allow(unused_must_use)]
  fn callback(
    &self,
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: CallbackData,
  ) {
    // use a threadlocal String as a buffer. This avoids allocating a new
    // string for every message.
    thread_local! {
      static OUTPUT_BUFFER: RefCell<String> = const {
        RefCell::new(String::new())
      }
    }

    OUTPUT_BUFFER.with_borrow_mut(|output| {
      output.clear();

      write!(
        output,
        "\
        Severity: {message_severity:?}, Type: {message_types:?}\n\
        message: {message_name} (0x{message_number:x})\n  {message}\n\
      ",
        message_name = callback_data.message_id_name(),
        message_number = callback_data.message_id_number(),
        message = callback_data.message(),
      );
      if !callback_data.queue_labels().is_empty() {
        writeln!(
          output,
          "queue_labels: {queue_labels}",
          queue_labels = callback_data.queue_labels().iter().format(", ")
        );
      }
      if !callback_data.command_buffer_labels().is_empty() {
        writeln!(
          output,
          "command_buffer_labels: {command_buffer_labels}",
          command_buffer_labels =
            callback_data.command_buffer_labels().iter().format(", ")
        );
      }
      if !callback_data.objects().is_empty() {
        writeln!(
          output,
          "objects: {objects}",
          objects = callback_data.objects().iter().format(", ")
        );
      }

      use vk::DebugUtilsMessageSeverityFlagsEXT as SeverityFlags;
      if message_severity.contains(SeverityFlags::ERROR) {
        log::error!("{output}");
      } else if message_severity.contains(SeverityFlags::WARNING) {
        log::warn!("{output}");
      } else if message_severity.contains(SeverityFlags::INFO) {
        log::info!("{output}");
      } else {
        log::debug!("{output}");
      }
    });
  }
}
