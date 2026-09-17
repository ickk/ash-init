mod extensions;

pub use self::extensions::InstanceExtensions;
use crate::Version;

pub struct InstanceRequirements {
  pub version: Version,
  pub extensions: InstanceExtensions,
}

impl Default for InstanceRequirements {
  fn default() -> Self {
    InstanceRequirements {
      version: Version::new(1, 0, 0),
      extensions: InstanceExtensions::default(),
    }
  }
}
