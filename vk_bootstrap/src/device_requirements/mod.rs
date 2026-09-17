mod device_type;
mod extensions;
mod features;
mod queue_requirements;

pub use self::{
  device_type::DeviceType,
  extensions::DeviceExtensions,
  features::Features,
  queue_requirements::{QueueCapabilities, QueueRequirements},
};

#[derive(Default)]
pub struct DeviceRequirements<'device_name, 'queue_names> {
  pub device_name_substr: &'device_name str,
  pub device_type: Option<DeviceType>,
  pub features: Features,
  pub extensions: DeviceExtensions,
  pub queues: Vec<QueueRequirements<'queue_names>>,
}
