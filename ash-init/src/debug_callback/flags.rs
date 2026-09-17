// © ickk 2023-2026, All Rights Reserved.

use {
  ::ash::vk,
  ::core::ops::{BitAnd, BitOr, BitXor},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MessageSeverityFlags {
  pub verbose: bool,
  pub info: bool,
  pub warning: bool,
  pub error: bool,
}

impl MessageSeverityFlags {
  pub const VERBOSE: Self = MessageSeverityFlags {
    verbose: true,
    ..Self::NONE
  };
  pub const INFO: Self = MessageSeverityFlags {
    info: true,
    ..Self::NONE
  };
  pub const WARNING: Self = MessageSeverityFlags {
    warning: true,
    ..Self::NONE
  };
  pub const ERROR: Self = MessageSeverityFlags {
    error: true,
    ..Self::NONE
  };
  pub const ALL: Self = MessageSeverityFlags {
    verbose: true,
    info: true,
    warning: true,
    error: true,
  };
  pub const NONE: Self = MessageSeverityFlags {
    verbose: false,
    info: false,
    warning: false,
    error: false,
  };
  pub const DEFAULT: Self = MessageSeverityFlags::NONE;
}

impl Default for MessageSeverityFlags {
  fn default() -> Self {
    MessageSeverityFlags::DEFAULT
  }
}

impl From<MessageSeverityFlags> for vk::DebugUtilsMessageSeverityFlagsEXT {
  #[inline]
  fn from(f: MessageSeverityFlags) -> Self {
    let verbose = f.verbose as u32;
    let info = (f.info as u32) << 4;
    let warning = (f.warning as u32) << 8;
    let error = (f.error as u32) << 12;

    vk::DebugUtilsMessageSeverityFlagsEXT::from_raw(
      error | warning | info | verbose,
    )
  }
}

impl BitOr for MessageSeverityFlags {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self {
    MessageSeverityFlags {
      verbose: self.verbose | rhs.verbose,
      info: self.info | rhs.info,
      warning: self.warning | rhs.warning,
      error: self.error | rhs.error,
    }
  }
}

impl BitAnd for MessageSeverityFlags {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self {
    MessageSeverityFlags {
      verbose: self.verbose & rhs.verbose,
      info: self.info & rhs.info,
      warning: self.warning & rhs.warning,
      error: self.error & rhs.error,
    }
  }
}

impl BitXor for MessageSeverityFlags {
  type Output = Self;

  fn bitxor(self, rhs: Self) -> Self {
    MessageSeverityFlags {
      verbose: self.verbose ^ rhs.verbose,
      info: self.info ^ rhs.info,
      warning: self.warning ^ rhs.warning,
      error: self.error ^ rhs.error,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MessageTypeFlags {
  pub general: bool,
  pub validation: bool,
  pub performance: bool,
  pub device_address_binding: bool,
}

impl MessageTypeFlags {
  pub const GENERAL: Self = MessageTypeFlags {
    general: true,
    ..Self::NONE
  };
  pub const VALIDATION: Self = MessageTypeFlags {
    validation: true,
    ..Self::NONE
  };
  pub const PERFORMANCE: Self = MessageTypeFlags {
    performance: true,
    ..Self::NONE
  };
  pub const DEVICE_ADDRESS_BINDING: Self = MessageTypeFlags {
    device_address_binding: true,
    ..Self::NONE
  };
  pub const ALL: Self = MessageTypeFlags {
    general: true,
    validation: true,
    performance: true,
    device_address_binding: true,
  };
  pub const NONE: Self = MessageTypeFlags {
    general: false,
    validation: false,
    performance: false,
    device_address_binding: false,
  };
  pub const DEFAULT: Self = MessageTypeFlags::NONE;
}

impl Default for MessageTypeFlags {
  fn default() -> Self {
    MessageTypeFlags::DEFAULT
  }
}

impl From<MessageTypeFlags> for vk::DebugUtilsMessageTypeFlagsEXT {
  #[inline]
  fn from(f: MessageTypeFlags) -> Self {
    let general = f.general as u32;
    let validation = (f.validation as u32) << 1;
    let performance = (f.performance as u32) << 2;
    let device_address_binding = (f.device_address_binding as u32) << 3;

    vk::DebugUtilsMessageTypeFlagsEXT::from_raw(
      device_address_binding | performance | validation | general,
    )
  }
}

impl BitOr for MessageTypeFlags {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self {
    MessageTypeFlags {
      general: self.general | rhs.general,
      validation: self.validation | rhs.validation,
      performance: self.performance | rhs.performance,
      device_address_binding: self.device_address_binding
        | rhs.device_address_binding,
    }
  }
}

impl BitAnd for MessageTypeFlags {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self {
    MessageTypeFlags {
      general: self.general & rhs.general,
      validation: self.validation & rhs.validation,
      performance: self.performance & rhs.performance,
      device_address_binding: self.device_address_binding
        & rhs.device_address_binding,
    }
  }
}

impl BitXor for MessageTypeFlags {
  type Output = Self;

  fn bitxor(self, rhs: Self) -> Self {
    MessageTypeFlags {
      general: self.general ^ rhs.general,
      validation: self.validation ^ rhs.validation,
      performance: self.performance ^ rhs.performance,
      device_address_binding: self.device_address_binding
        ^ rhs.device_address_binding,
    }
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use {super::*, ash::vk};

  #[test]
  fn message_severity_compliance() {
    assert_eq!(
      vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
      vk::DebugUtilsMessageSeverityFlagsEXT::from(MessageSeverityFlags {
        verbose: true,
        ..Default::default()
      })
    );

    assert_eq!(
      vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
      vk::DebugUtilsMessageSeverityFlagsEXT::from(MessageSeverityFlags {
        info: true,
        ..Default::default()
      })
    );

    assert_eq!(
      vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
      vk::DebugUtilsMessageSeverityFlagsEXT::from(MessageSeverityFlags {
        warning: true,
        ..Default::default()
      })
    );

    assert_eq!(
      vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
      vk::DebugUtilsMessageSeverityFlagsEXT::from(MessageSeverityFlags {
        error: true,
        ..Default::default()
      })
    );

    assert_eq!(
      vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
        | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
      vk::DebugUtilsMessageSeverityFlagsEXT::from(MessageSeverityFlags::ALL)
    );
  }

  #[test]
  fn message_type_compliance() {
    assert_eq!(
      vk::DebugUtilsMessageTypeFlagsEXT::GENERAL,
      vk::DebugUtilsMessageTypeFlagsEXT::from(MessageTypeFlags {
        general: true,
        ..Default::default()
      })
    );

    assert_eq!(
      vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
      vk::DebugUtilsMessageTypeFlagsEXT::from(MessageTypeFlags {
        validation: true,
        ..Default::default()
      })
    );

    assert_eq!(
      vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
      vk::DebugUtilsMessageTypeFlagsEXT::from(MessageTypeFlags {
        performance: true,
        ..Default::default()
      })
    );

    assert_eq!(
      vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
      vk::DebugUtilsMessageTypeFlagsEXT::from(MessageTypeFlags {
        device_address_binding: true,
        ..Default::default()
      })
    );

    assert_eq!(
      vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
        | vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
      vk::DebugUtilsMessageTypeFlagsEXT::from(MessageTypeFlags::ALL)
    );
  }
}
