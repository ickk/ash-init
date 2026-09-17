use ::std::num::ParseIntError;

#[derive(Copy, Clone, Debug)]
pub struct Version {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
}

impl From<(u32, u32, u32)> for Version {
  fn from((major, minor, patch): (u32, u32, u32)) -> Self {
    Version {
      major,
      minor,
      patch,
    }
  }
}

impl Version {
  pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
    Version {
      major,
      minor,
      patch,
    }
  }
}

impl TryFrom<(&str, &str, &str)> for Version {
  type Error = ParseIntError;

  fn try_from(
    (major, minor, patch): (&str, &str, &str),
  ) -> ::core::result::Result<Self, ParseIntError> {
    Ok(Version::new(major.parse()?, minor.parse()?, patch.parse()?))
  }
}

pub struct StrVersion<'s>(pub &'s str, pub &'s str, pub &'s str);

impl From<StrVersion<'_>> for Version {
  fn from(StrVersion(major, minor, patch): StrVersion) -> Self {
    Version::try_from((major, minor, patch)).unwrap()
  }
}

pub trait VersionToVk {
  fn to_vk_api_version(self) -> u32;
}

impl VersionToVk for Version {
  fn to_vk_api_version(self) -> u32 {
    ::ash::vk::make_api_version(0, self.major, self.minor, self.patch)
  }
}
