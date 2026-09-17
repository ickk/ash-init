pub mod debug;
mod device_requirements;
mod error;
mod instance_requirements;
mod macros;
mod present_support;
mod queue;
mod queue_requirements;
mod version;
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
    device_requirements::{DeviceExtensions, DeviceRequirements, Features},
    error::{Error, ErrorList, Result},
    image::Image,
    instance_requirements::{InstanceExtensions, InstanceRequirements},
    present_support::{
      HasDisplayHandleExt, HasWindowHandleExt, Platform, PresentSupport,
    },
    queue::Queue,
    queue_requirements::{QueueCapabilities, QueueRequirements},
    shader_module::ShaderModule,
    surface::Surface,
    surface_ref::SurfaceRef,
    swapchain::Swapchain,
    swapchain_frame::SwapchainFrame,
    version::Version,
    vk_context::VkContext,
  },
  ::ash::{self, vk, Device},
};
