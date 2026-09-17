// © ickk 2023-2026, All Rights Reserved.

use {
  crate::debug::*,
  ::ash::vk,
  ::core::{cell::RefCell, fmt::Write as _},
};

/// A [`DebugCallback`] impl that creates [`tracing`] events for messages it
/// receives.
pub struct TracingDebugCallback<const ENABLE_COLOR: bool = false> {
  message_severities: vk::DebugUtilsMessageSeverityFlagsEXT,
  message_types: vk::DebugUtilsMessageTypeFlagsEXT,
}

impl TracingDebugCallback {
  /// Create a new `TracingDebugCallback` that listens for messages matching
  /// the specified severities and types.
  ///
  /// You can use [`MessageSeverityFlags`][crate::debug::MessageSeverityFlags]
  /// and [`MessageTypeFlags`][crate::debug::MessageTypeFlags] to populate
  /// these arguments instead of `vk::DebugUtilsMessage*FlagsEXT`.
  #[expect(clippy::new_ret_no_self)]
  pub fn new(
    message_severities: impl Into<vk::DebugUtilsMessageSeverityFlagsEXT>,
    message_types: impl Into<vk::DebugUtilsMessageTypeFlagsEXT>,
  ) -> Box<dyn DebugCallback> {
    Box::new(TracingDebugCallback::<false> {
      message_severities: message_severities.into(),
      message_types: message_types.into(),
    })
  }

  /// Create a new `TracingDebugCallback` that listens for messages matching
  /// the specified severities and types.
  ///
  /// You can use [`MessageSeverityFlags`][crate::debug::MessageSeverityFlags]
  /// and [`MessageTypeFlags`][crate::debug::MessageTypeFlags] to populate
  /// these arguments instead of `vk::DebugUtilsMessage*FlagsEXT`.
  pub fn new_color(
    message_severities: impl Into<vk::DebugUtilsMessageSeverityFlagsEXT>,
    message_types: impl Into<vk::DebugUtilsMessageTypeFlagsEXT>,
  ) -> Box<dyn DebugCallback> {
    Box::new(TracingDebugCallback::<true> {
      message_severities: message_severities.into(),
      message_types: message_types.into(),
    })
  }
}

impl<const ENABLE_COLOR: bool> DebugCallback
  for TracingDebugCallback<ENABLE_COLOR>
{
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

      // meta data
      if ENABLE_COLOR {
        use vk::DebugUtilsMessageSeverityFlagsEXT as S;
        let colour = if message_severity.contains(S::ERROR) {
          "\x1b[1;31m"
        } else if message_severity.contains(S::WARNING) {
          "\x1b[33m"
        } else {
          "\x1b[0m"
        };
        writeln!(
          output,
          "{colour}{message_types:?} {message_severity:?}\x1b[0m"
        );
      } else {
        writeln!(output, "{message_types:?} {message_severity:?}");
      }

      // message title
      if ENABLE_COLOR {
        write!(output, "\x1b[4m{}\x1b[0m", callback_data.message_id_name());
      } else {
        write!(output, "{}", callback_data.message_id_name());
      }
      if callback_data.message_id_number() != 0 {
        write!(output, " (0x{:x?})", callback_data.message_id_number());
      }
      writeln!(output);

      // special case validation errors to remove redundant info
      let mut message = callback_data.message().trim_start();
      'validation_trim: {
        if message_types
          .contains(vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
        {
          let header = [
            "Validation Error: [ ",
            callback_data.message_id_name(),
            " ]",
          ];
          for s in header {
            if message.starts_with(s) {
              message = &message[s.len()..];
            } else {
              message = callback_data.message().trim_start();
              break 'validation_trim;
            }
          }
          for _ in 0..2 {
            if let Some(index) = message.find("| ") {
              message = &message[index + 2..];
            } else {
              message = callback_data.message().trim_start();
              break 'validation_trim;
            }
          }
          if let Some((problem, explanation)) =
            message.split_once("The Vulkan spec states: ")
          {
            if ENABLE_COLOR {
              let mut problem_msg = problem;
              write!(output, " ");
              while let Some((before, rest)) = problem_msg.split_once('[') {
                if let Some((name, after)) = rest.split_once(']') {
                  write!(output, "{before}[");
                  write!(output, "\x1b[34m{name}\x1b[0m");
                  write!(output, "]");
                  problem_msg = after;
                }
              }

              writeln!(
                output,
                "{problem_msg}\n\x1b[90mThe Vulkan spec states:\x1b[0m"
              );
            } else {
              writeln!(output, " {problem}\nThe Vulkan spec states:");
            }
            message = explanation;
          }
        }
      }

      writeln!(output, " {message}");

      use vk::DebugUtilsMessageSeverityFlagsEXT as SeverityFlags;
      if message_severity.contains(SeverityFlags::ERROR) {
        tracing::error!(target: "Vulkan", "{output}");
      } else if message_severity.contains(SeverityFlags::WARNING) {
        tracing::warn!(target: "Vulkan", "{output}");
      } else if message_severity.contains(SeverityFlags::INFO) {
        tracing::info!(target: "Vulkan", "{output}");
      } else {
        tracing::debug!(target: "Vulkan", "{output}");
      }
    });
  }
}
