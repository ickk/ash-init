use ::ash::vk;

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

  pub const fn to_vk_api_version(self) -> u32 {
    vk::make_api_version(0, self.major, self.minor, self.patch)
  }
}
