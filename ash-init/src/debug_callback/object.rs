// © ickk 2023-2026, All Rights Reserved.

use ::ash::vk::{self, Handle as _, ObjectType};

/// Enumeration of Vulkan Object handles
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Object {
  Unknown(u64) = 0,
  Instance(vk::Instance) = 1,
  PhysicalDevice(vk::PhysicalDevice) = 2,
  Device(vk::Device) = 3,
  Queue(vk::Queue) = 4,
  Semaphore(vk::Semaphore) = 5,
  CommandBuffer(vk::CommandBuffer) = 6,
  Fence(vk::Fence) = 7,
  DeviceMemory(vk::DeviceMemory) = 8,
  Buffer(vk::Buffer) = 9,
  Image(vk::Image) = 10,
  Event(vk::Event) = 11,
  QueryPool(vk::QueryPool) = 12,
  BufferView(vk::BufferView) = 13,
  ImageView(vk::ImageView) = 14,
  ShaderModule(vk::ShaderModule) = 15,
  PipelineCache(vk::PipelineCache) = 16,
  PipelineLayout(vk::PipelineLayout) = 17,
  RenderPass(vk::RenderPass) = 18,
  Pipeline(vk::Pipeline) = 19,
  DescriptorSetLayout(vk::DescriptorSetLayout) = 20,
  Sampler(vk::Sampler) = 21,
  DescriptorPool(vk::DescriptorPool) = 22,
  DescriptorSet(vk::DescriptorSet) = 23,
  Framebuffer(vk::Framebuffer) = 24,
  CommandPool(vk::CommandPool) = 25,
  SamplerYcbcrConversion(vk::SamplerYcbcrConversion) = 1000156000,
  DescriptorUpdateTemplate(vk::DescriptorUpdateTemplate) = 1000085000,
  PrivateDataSlot(vk::PrivateDataSlot) = 1000295000,
  SurfaceKhr(vk::SurfaceKHR) = 1000000000,
  SwapchainKhr(vk::SwapchainKHR) = 1000001000,
  DisplayKhr(vk::DisplayKHR) = 1000002000,
  DisplayModeKhr(vk::DisplayModeKHR) = 1000002001,
  DebugReportCallbackExt(vk::DebugReportCallbackEXT) = 1000011000,
  VideoSessionKhr(vk::VideoSessionKHR) = 1000023000,
  VideoSessionParametersKhr(vk::VideoSessionParametersKHR) = 1000023001,
  CuModuleNvx(vk::CuModuleNVX) = 1000029000,
  CuFunctionNvx(vk::CuFunctionNVX) = 1000029001,
  DebugUtilsMessengerExt(vk::DebugUtilsMessengerEXT) = 1000128000,
  AccelerationStructureKhr(vk::AccelerationStructureKHR) = 1000150000,
  ValidationCacheExt(vk::ValidationCacheEXT) = 1000160000,
  AccelerationStructureNv(vk::AccelerationStructureNV) = 1000165000,
  PerformanceConfigurationIntel(vk::PerformanceConfigurationINTEL) =
    1000210000,
  DeferredOperationKhr(vk::DeferredOperationKHR) = 1000268000,
  IndirectCommandsLayoutNv(vk::IndirectCommandsLayoutNV) = 1000277000,
  BufferCollectionFuchsia(vk::BufferCollectionFUCHSIA) = 1000366000,
  MicromapExt(vk::MicromapEXT) = 1000396000,
  OpticalFlowSessionNv(vk::OpticalFlowSessionNV) = 1000464000,
  ShaderExt(vk::ShaderEXT) = 1000482000,
  Other {
    object_type: vk::ObjectType,
    handle: u64,
  },
}

impl Object {
  #[inline]
  pub(crate) fn from_object_type_and_handle(
    object_type: vk::ObjectType,
    handle: u64,
  ) -> Self {
    use vk::ObjectType as O;
    use Object as S;
    match object_type {
      O::UNKNOWN => S::Unknown(handle),
      O::INSTANCE => S::Instance(vk::Instance::from_raw(handle)),
      O::PHYSICAL_DEVICE => {
        S::PhysicalDevice(vk::PhysicalDevice::from_raw(handle))
      },
      O::DEVICE => S::Device(vk::Device::from_raw(handle)),
      O::QUEUE => S::Queue(vk::Queue::from_raw(handle)),
      O::SEMAPHORE => S::Semaphore(vk::Semaphore::from_raw(handle)),
      O::COMMAND_BUFFER => {
        S::CommandBuffer(vk::CommandBuffer::from_raw(handle))
      },
      O::FENCE => S::Fence(vk::Fence::from_raw(handle)),
      O::DEVICE_MEMORY => S::DeviceMemory(vk::DeviceMemory::from_raw(handle)),
      O::BUFFER => S::Buffer(vk::Buffer::from_raw(handle)),
      O::IMAGE => S::Image(vk::Image::from_raw(handle)),
      O::EVENT => S::Event(vk::Event::from_raw(handle)),
      O::QUERY_POOL => S::QueryPool(vk::QueryPool::from_raw(handle)),
      O::BUFFER_VIEW => S::BufferView(vk::BufferView::from_raw(handle)),
      O::IMAGE_VIEW => S::ImageView(vk::ImageView::from_raw(handle)),
      O::SHADER_MODULE => S::ShaderModule(vk::ShaderModule::from_raw(handle)),
      O::PIPELINE_CACHE => {
        S::PipelineCache(vk::PipelineCache::from_raw(handle))
      },
      O::PIPELINE_LAYOUT => {
        S::PipelineLayout(vk::PipelineLayout::from_raw(handle))
      },
      O::RENDER_PASS => S::RenderPass(vk::RenderPass::from_raw(handle)),
      O::PIPELINE => S::Pipeline(vk::Pipeline::from_raw(handle)),
      O::DESCRIPTOR_SET_LAYOUT => {
        S::DescriptorSetLayout(vk::DescriptorSetLayout::from_raw(handle))
      },
      O::SAMPLER => S::Sampler(vk::Sampler::from_raw(handle)),
      O::DESCRIPTOR_POOL => {
        S::DescriptorPool(vk::DescriptorPool::from_raw(handle))
      },
      O::DESCRIPTOR_SET => {
        S::DescriptorSet(vk::DescriptorSet::from_raw(handle))
      },
      O::FRAMEBUFFER => S::Framebuffer(vk::Framebuffer::from_raw(handle)),
      O::COMMAND_POOL => S::CommandPool(vk::CommandPool::from_raw(handle)),
      O::SAMPLER_YCBCR_CONVERSION => {
        S::SamplerYcbcrConversion(vk::SamplerYcbcrConversion::from_raw(handle))
      },
      O::DESCRIPTOR_UPDATE_TEMPLATE => S::DescriptorUpdateTemplate(
        vk::DescriptorUpdateTemplate::from_raw(handle),
      ),
      O::PRIVATE_DATA_SLOT => {
        S::PrivateDataSlot(vk::PrivateDataSlot::from_raw(handle))
      },
      O::SURFACE_KHR => S::SurfaceKhr(vk::SurfaceKHR::from_raw(handle)),
      O::SWAPCHAIN_KHR => S::SwapchainKhr(vk::SwapchainKHR::from_raw(handle)),
      O::DISPLAY_KHR => S::DisplayKhr(vk::DisplayKHR::from_raw(handle)),
      O::DISPLAY_MODE_KHR => {
        S::DisplayModeKhr(vk::DisplayModeKHR::from_raw(handle))
      },
      O::DEBUG_REPORT_CALLBACK_EXT => {
        S::DebugReportCallbackExt(vk::DebugReportCallbackEXT::from_raw(handle))
      },
      O::VIDEO_SESSION_KHR => {
        S::VideoSessionKhr(vk::VideoSessionKHR::from_raw(handle))
      },
      O::VIDEO_SESSION_PARAMETERS_KHR => S::VideoSessionParametersKhr(
        vk::VideoSessionParametersKHR::from_raw(handle),
      ),
      O::CU_MODULE_NVX => S::CuModuleNvx(vk::CuModuleNVX::from_raw(handle)),
      O::CU_FUNCTION_NVX => {
        S::CuFunctionNvx(vk::CuFunctionNVX::from_raw(handle))
      },
      O::DEBUG_UTILS_MESSENGER_EXT => {
        S::DebugUtilsMessengerExt(vk::DebugUtilsMessengerEXT::from_raw(handle))
      },
      O::ACCELERATION_STRUCTURE_KHR => S::AccelerationStructureKhr(
        vk::AccelerationStructureKHR::from_raw(handle),
      ),
      O::VALIDATION_CACHE_EXT => {
        S::ValidationCacheExt(vk::ValidationCacheEXT::from_raw(handle))
      },
      O::ACCELERATION_STRUCTURE_NV => S::AccelerationStructureNv(
        vk::AccelerationStructureNV::from_raw(handle),
      ),
      O::PERFORMANCE_CONFIGURATION_INTEL => S::PerformanceConfigurationIntel(
        vk::PerformanceConfigurationINTEL::from_raw(handle),
      ),
      O::DEFERRED_OPERATION_KHR => {
        S::DeferredOperationKhr(vk::DeferredOperationKHR::from_raw(handle))
      },
      O::INDIRECT_COMMANDS_LAYOUT_NV => S::IndirectCommandsLayoutNv(
        vk::IndirectCommandsLayoutNV::from_raw(handle),
      ),
      O::BUFFER_COLLECTION_FUCHSIA => S::BufferCollectionFuchsia(
        vk::BufferCollectionFUCHSIA::from_raw(handle),
      ),
      O::MICROMAP_EXT => S::MicromapExt(vk::MicromapEXT::from_raw(handle)),
      O::OPTICAL_FLOW_SESSION_NV => {
        S::OpticalFlowSessionNv(vk::OpticalFlowSessionNV::from_raw(handle))
      },
      O::SHADER_EXT => S::ShaderExt(vk::ShaderEXT::from_raw(handle)),
      _ => S::Other {
        object_type,
        handle,
      },
    }
  }

  pub fn object_type(&self) -> vk::ObjectType {
    match self {
      Object::Unknown(_) => ObjectType::UNKNOWN,
      Object::Instance(_) => ObjectType::INSTANCE,
      Object::PhysicalDevice(_) => ObjectType::PHYSICAL_DEVICE,
      Object::Device(_) => ObjectType::DEVICE,
      Object::Queue(_) => ObjectType::QUEUE,
      Object::Semaphore(_) => ObjectType::SEMAPHORE,
      Object::CommandBuffer(_) => ObjectType::COMMAND_BUFFER,
      Object::Fence(_) => ObjectType::FENCE,
      Object::DeviceMemory(_) => ObjectType::DEVICE_MEMORY,
      Object::Buffer(_) => ObjectType::BUFFER,
      Object::Image(_) => ObjectType::IMAGE,
      Object::Event(_) => ObjectType::EVENT,
      Object::QueryPool(_) => ObjectType::QUERY_POOL,
      Object::BufferView(_) => ObjectType::BUFFER_VIEW,
      Object::ImageView(_) => ObjectType::IMAGE_VIEW,
      Object::ShaderModule(_) => ObjectType::SHADER_MODULE,
      Object::PipelineCache(_) => ObjectType::PIPELINE_CACHE,
      Object::PipelineLayout(_) => ObjectType::PIPELINE_LAYOUT,
      Object::RenderPass(_) => ObjectType::RENDER_PASS,
      Object::Pipeline(_) => ObjectType::PIPELINE,
      Object::DescriptorSetLayout(_) => ObjectType::DESCRIPTOR_SET_LAYOUT,
      Object::Sampler(_) => ObjectType::SAMPLER,
      Object::DescriptorPool(_) => ObjectType::DESCRIPTOR_POOL,
      Object::DescriptorSet(_) => ObjectType::DESCRIPTOR_SET,
      Object::Framebuffer(_) => ObjectType::FRAMEBUFFER,
      Object::CommandPool(_) => ObjectType::COMMAND_POOL,
      Object::SamplerYcbcrConversion(_) => {
        ObjectType::SAMPLER_YCBCR_CONVERSION
      },
      Object::DescriptorUpdateTemplate(_) => {
        ObjectType::DESCRIPTOR_UPDATE_TEMPLATE
      },
      Object::PrivateDataSlot(_) => ObjectType::PRIVATE_DATA_SLOT,
      Object::SurfaceKhr(_) => ObjectType::SURFACE_KHR,
      Object::SwapchainKhr(_) => ObjectType::SWAPCHAIN_KHR,
      Object::DisplayKhr(_) => ObjectType::DISPLAY_KHR,
      Object::DisplayModeKhr(_) => ObjectType::DISPLAY_MODE_KHR,
      Object::DebugReportCallbackExt(_) => {
        ObjectType::DEBUG_REPORT_CALLBACK_EXT
      },
      Object::VideoSessionKhr(_) => ObjectType::VIDEO_SESSION_KHR,
      Object::VideoSessionParametersKhr(_) => {
        ObjectType::VIDEO_SESSION_PARAMETERS_KHR
      },
      Object::CuModuleNvx(_) => ObjectType::CU_MODULE_NVX,
      Object::CuFunctionNvx(_) => ObjectType::CU_FUNCTION_NVX,
      Object::DebugUtilsMessengerExt(_) => {
        ObjectType::DEBUG_UTILS_MESSENGER_EXT
      },
      Object::AccelerationStructureKhr(_) => {
        ObjectType::ACCELERATION_STRUCTURE_KHR
      },
      Object::ValidationCacheExt(_) => ObjectType::VALIDATION_CACHE_EXT,
      Object::AccelerationStructureNv(_) => {
        ObjectType::ACCELERATION_STRUCTURE_NV
      },
      Object::PerformanceConfigurationIntel(_) => {
        ObjectType::PERFORMANCE_CONFIGURATION_INTEL
      },
      Object::DeferredOperationKhr(_) => ObjectType::DEFERRED_OPERATION_KHR,
      Object::IndirectCommandsLayoutNv(_) => {
        ObjectType::INDIRECT_COMMANDS_LAYOUT_NV
      },
      Object::BufferCollectionFuchsia(_) => {
        ObjectType::BUFFER_COLLECTION_FUCHSIA
      },
      Object::MicromapExt(_) => ObjectType::MICROMAP_EXT,
      Object::OpticalFlowSessionNv(_) => ObjectType::OPTICAL_FLOW_SESSION_NV,
      Object::ShaderExt(_) => ObjectType::SHADER_EXT,
      Object::Other { object_type, .. } => *object_type,
    }
  }

  pub fn raw_object_handle(&self) -> u64 {
    match self {
      Object::Unknown(o) => *o,
      Object::Instance(o) => o.as_raw(),
      Object::PhysicalDevice(o) => o.as_raw(),
      Object::Device(o) => o.as_raw(),
      Object::Queue(o) => o.as_raw(),
      Object::Semaphore(o) => o.as_raw(),
      Object::CommandBuffer(o) => o.as_raw(),
      Object::Fence(o) => o.as_raw(),
      Object::DeviceMemory(o) => o.as_raw(),
      Object::Buffer(o) => o.as_raw(),
      Object::Image(o) => o.as_raw(),
      Object::Event(o) => o.as_raw(),
      Object::QueryPool(o) => o.as_raw(),
      Object::BufferView(o) => o.as_raw(),
      Object::ImageView(o) => o.as_raw(),
      Object::ShaderModule(o) => o.as_raw(),
      Object::PipelineCache(o) => o.as_raw(),
      Object::PipelineLayout(o) => o.as_raw(),
      Object::RenderPass(o) => o.as_raw(),
      Object::Pipeline(o) => o.as_raw(),
      Object::DescriptorSetLayout(o) => o.as_raw(),
      Object::Sampler(o) => o.as_raw(),
      Object::DescriptorPool(o) => o.as_raw(),
      Object::DescriptorSet(o) => o.as_raw(),
      Object::Framebuffer(o) => o.as_raw(),
      Object::CommandPool(o) => o.as_raw(),
      Object::SamplerYcbcrConversion(o) => o.as_raw(),
      Object::DescriptorUpdateTemplate(o) => o.as_raw(),
      Object::PrivateDataSlot(o) => o.as_raw(),
      Object::SurfaceKhr(o) => o.as_raw(),
      Object::SwapchainKhr(o) => o.as_raw(),
      Object::DisplayKhr(o) => o.as_raw(),
      Object::DisplayModeKhr(o) => o.as_raw(),
      Object::DebugReportCallbackExt(o) => o.as_raw(),
      Object::VideoSessionKhr(o) => o.as_raw(),
      Object::VideoSessionParametersKhr(o) => o.as_raw(),
      Object::CuModuleNvx(o) => o.as_raw(),
      Object::CuFunctionNvx(o) => o.as_raw(),
      Object::DebugUtilsMessengerExt(o) => o.as_raw(),
      Object::AccelerationStructureKhr(o) => o.as_raw(),
      Object::ValidationCacheExt(o) => o.as_raw(),
      Object::AccelerationStructureNv(o) => o.as_raw(),
      Object::PerformanceConfigurationIntel(o) => o.as_raw(),
      Object::DeferredOperationKhr(o) => o.as_raw(),
      Object::IndirectCommandsLayoutNv(o) => o.as_raw(),
      Object::BufferCollectionFuchsia(o) => o.as_raw(),
      Object::MicromapExt(o) => o.as_raw(),
      Object::OpticalFlowSessionNv(o) => o.as_raw(),
      Object::ShaderExt(o) => o.as_raw(),
      Object::Other { handle, .. } => *handle,
    }
  }
}

macro_rules! impl_from_vk_object {
    (
      $(
        $vk_object:ty => $object_variant:expr
      ),*$(,)?
    ) => {
      $(
        impl From<$vk_object> for Object {
          fn from(object: $vk_object) -> Object {
            $object_variant(object)
          }
        }
      )*
    };
}

impl_from_vk_object! {
  vk::Instance => Object::Instance,
  vk::PhysicalDevice => Object::PhysicalDevice,
  vk::Device => Object::Device,
  vk::Queue => Object::Queue,
  vk::Semaphore => Object::Semaphore,
  vk::CommandBuffer => Object::CommandBuffer,
  vk::Fence => Object::Fence,
  vk::DeviceMemory => Object::DeviceMemory,
  vk::Buffer => Object::Buffer,
  vk::Image => Object::Image,
  vk::Event => Object::Event,
  vk::QueryPool => Object::QueryPool,
  vk::BufferView => Object::BufferView,
  vk::ImageView => Object::ImageView,
  vk::ShaderModule => Object::ShaderModule,
  vk::PipelineCache => Object::PipelineCache,
  vk::PipelineLayout => Object::PipelineLayout,
  vk::RenderPass => Object::RenderPass,
  vk::Pipeline => Object::Pipeline,
  vk::DescriptorSetLayout => Object::DescriptorSetLayout,
  vk::Sampler => Object::Sampler,
  vk::DescriptorPool => Object::DescriptorPool,
  vk::DescriptorSet => Object::DescriptorSet,
  vk::Framebuffer => Object::Framebuffer,
  vk::CommandPool => Object::CommandPool,
  vk::SamplerYcbcrConversion => Object::SamplerYcbcrConversion,
  vk::DescriptorUpdateTemplate => Object::DescriptorUpdateTemplate,
  vk::PrivateDataSlot => Object::PrivateDataSlot,
  vk::SurfaceKHR => Object::SurfaceKhr,
  vk::SwapchainKHR => Object::SwapchainKhr,
  vk::DisplayKHR => Object::DisplayKhr,
  vk::DisplayModeKHR => Object::DisplayModeKhr,
  vk::DebugReportCallbackEXT => Object::DebugReportCallbackExt,
  vk::VideoSessionKHR => Object::VideoSessionKhr,
  vk::VideoSessionParametersKHR => Object::VideoSessionParametersKhr,
  vk::CuModuleNVX => Object::CuModuleNvx,
  vk::CuFunctionNVX => Object::CuFunctionNvx,
  vk::DebugUtilsMessengerEXT => Object::DebugUtilsMessengerExt,
  vk::AccelerationStructureKHR => Object::AccelerationStructureKhr,
  vk::ValidationCacheEXT => Object::ValidationCacheExt,
  vk::AccelerationStructureNV => Object::AccelerationStructureNv,
  vk::PerformanceConfigurationINTEL => Object::PerformanceConfigurationIntel,
  vk::DeferredOperationKHR => Object::DeferredOperationKhr,
  vk::IndirectCommandsLayoutNV => Object::IndirectCommandsLayoutNv,
  vk::BufferCollectionFUCHSIA => Object::BufferCollectionFuchsia,
  vk::MicromapEXT => Object::MicromapExt,
  vk::OpticalFlowSessionNV => Object::OpticalFlowSessionNv,
  vk::ShaderEXT => Object::ShaderExt,
}
