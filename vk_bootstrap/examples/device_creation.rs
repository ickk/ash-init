use ::vk_bootstrap::{
  debug::{
    callbacks::TracingDebugCallback, MessageSeverityFlags, MessageTypeFlags,
  },
  DeviceExtensions, DeviceRequirements, Features, InstanceExtensions,
  InstanceRequirements, PresentSupport, QueueCapabilities, QueueRequirements,
  Version, VkContext,
};

const APP_NAME: &str = file!();
const APP_VERSION: Version = Version::new(0, 0, 0);
const RENDERER_NAME: &str = "toil/engine/vk_context";
const RENDERER_VERSION: Version = Version::new(0, 0, 0);

fn main() {
  let vk_context = VkContext::new(
    (APP_NAME, APP_VERSION),
    (RENDERER_NAME, RENDERER_VERSION),
    InstanceRequirements {
      version: Version::new(1, 3, 0),
      extensions: InstanceExtensions {
        ext_debug_utils: cfg!(debug_assertions),
        khr_surface: true,
        khr_win32_surface: cfg!(target_os = "windows"),
        khr_xcb_surface: cfg!(target_os = "linux"),
        ..InstanceExtensions::default()
      },
    },
    vec![TracingDebugCallback::new(
      MessageSeverityFlags {
        verbose: false,
        info: true,
        warning: true,
        error: true,
      },
      MessageTypeFlags::ALL,
    )],
  )
  .unwrap();

  let (device, queues) = {
    let requirements = DeviceRequirements {
      device_name_substr: "nvidia",
      features: Features {
        buffer_device_address: true,
        descriptor_indexing: true,
        ..Features::default()
      },
      extensions: DeviceExtensions {
        ext_pageable_device_local_memory: true,
        ext_memory_priority: true,
        ..DeviceExtensions::default()
      },
      queues: vec![
        QueueRequirements {
          names: vec!["graphics"],
          priorities: vec![1.0],
          capabilities: QueueCapabilities {
            compute: true,
            graphics: true,
            ..QueueCapabilities::default()
          },
          #[cfg(target_os = "windows")]
          present_support: PresentSupport::Win32,
          #[cfg(target_os = "linux")]
          present_support: PresentSupport::None,
          ..QueueRequirements::default()
        },
        QueueRequirements {
          names: vec!["transfer"],
          priorities: vec![0.5],
          capabilities: QueueCapabilities {
            transfer: true,
            ..QueueCapabilities::default()
          },
          ..QueueRequirements::default()
        },
      ],
      ..DeviceRequirements::default()
    };

    let physical_device =
      vk_context.get_physical_device(&requirements).unwrap();

    unsafe {
      vk_context.get_logical_device_and_queues(physical_device, &requirements)
    }
    .unwrap()
  };

  println!("queues: {queues:#?}");

  unsafe { device.destroy_device(None) };
}
