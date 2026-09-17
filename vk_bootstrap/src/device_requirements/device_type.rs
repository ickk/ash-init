use ::ash::vk::PhysicalDeviceType;

#[derive(Copy, Clone, Debug)]
pub enum DeviceType {
  Other,
  Integrated,
  Discrete,
  Virtual,
  Cpu,
}

impl From<DeviceType> for PhysicalDeviceType {
  #[inline]
  fn from(device_type: DeviceType) -> Self {
    match device_type {
      DeviceType::Other => PhysicalDeviceType::OTHER,
      DeviceType::Integrated => PhysicalDeviceType::INTEGRATED_GPU,
      DeviceType::Discrete => PhysicalDeviceType::DISCRETE_GPU,
      DeviceType::Virtual => PhysicalDeviceType::VIRTUAL_GPU,
      DeviceType::Cpu => PhysicalDeviceType::CPU,
    }
  }
}
