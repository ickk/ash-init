mod error;
mod platform;
mod present_support;
mod raw_window_handle;
mod version;

// VkContext stuff
pub mod debug;
mod device_requirements;
mod instance_requirements;
mod macros;
mod queue;
mod vk_context;

// DeviceContext stuff
mod device_context;
mod device_context_ref;
mod image;
mod shader_module;
mod surface;
mod surface_ref;
mod swapchain;
mod swapchain_frame;

pub use {
  crate::{
    device_context::DeviceContext,
    device_context_ref::DeviceContextRef,
    device_requirements::{
      DeviceExtensions, DeviceRequirements, Features, QueueCapabilities,
      QueueRequirements,
    },
    error::{Error, ErrorList, Result},
    image::Image,
    instance_requirements::{InstanceExtensions, InstanceRequirements},
    platform::Platform,
    present_support::PresentSupport,
    queue::Queue,
    shader_module::ShaderModule,
    surface::Surface,
    surface_ref::SurfaceRef,
    swapchain::Swapchain,
    swapchain_frame::SwapchainFrame,
    version::{StrVersion, Version, VersionToVk},
    vk_context::VkContext,
  },
  ::ash::{self, vk, Device},
};
