use {
  crate::{DeviceContext, Image, Queue, Result, ShaderModule, VkContext},
  ::ash::{ext, vk, Device},
};

pub trait DeviceContextRef {
  type VkC: AsRef<VkContext>;
  fn as_ref(&self) -> &DeviceContext<Self::VkC>;

  // delegate getters
  fn vk_context(&self) -> &VkContext {
    self.as_ref().vk_context()
  }
  fn physical_device(&self) -> vk::PhysicalDevice {
    self.as_ref().physical_device()
  }
  fn device(&self) -> &Device {
    self.as_ref().device()
  }
  fn queues(&self) -> &[Queue] {
    self.as_ref().queues()
  }
  fn get_queue(&self, name: &str) -> Option<&Queue> {
    self.as_ref().get_queue(name)
  }
  fn queue(&self, name: &str) -> &Queue {
    self.as_ref().queue(name)
  }
  fn memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
    self.as_ref().memory_properties()
  }
  fn debug_utils(&self) -> Option<&ext::debug_utils::Device> {
    self.as_ref().debug_utils()
  }

  // methods
  fn create_binary_semaphore(&self) -> Result<vk::Semaphore> {
    self.as_ref().create_binary_semaphore()
  }
  fn create_timeline_semaphore(&self) -> Result<vk::Semaphore> {
    self.as_ref().create_timeline_semaphore()
  }
  fn create_fence(&self, signaled: bool) -> Result<vk::Fence> {
    self.as_ref().create_fence(signaled)
  }
  fn get_fence_status(&self, fence: vk::Fence) -> Result<bool> {
    self.as_ref().get_fence_status(fence)
  }
  fn set_debug_object_name(
    &self,
    object_handle: impl vk::Handle,
    name: impl ::core::fmt::Display,
  ) -> Result<()> {
    self.as_ref().set_debug_object_name(object_handle, name)
  }
  #[allow(clippy::missing_safety_doc)]
  unsafe fn create_buffer_exclusive(
    &self,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
  ) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    unsafe {
      self
        .as_ref()
        .create_buffer_exclusive(size, usage, properties)
    }
  }
  // Image methods
  fn create_image(
    &self,
    extent2d: vk::Extent2D,
    format: vk::Format,
    tiling: vk::ImageTiling,
    usage: vk::ImageUsageFlags,
    requested_memory_properties: vk::MemoryPropertyFlags,
    aspect: vk::ImageAspectFlags,
    render_target: bool,
  ) -> Result<Image> {
    self.as_ref().create_image(
      extent2d,
      format,
      tiling,
      usage,
      requested_memory_properties,
      aspect,
      render_target,
    )
  }
  fn add_image_view(
    &self,
    image: &mut Image,
    format: vk::Format,
  ) -> Result<usize> {
    self.as_ref().add_image_view(image, format)
  }
  #[allow(clippy::missing_safety_doc)]
  unsafe fn destroy_image(&self, image: &Image) {
    unsafe { self.as_ref().destroy_image(image) }
  }
  // ShaderModule methods
  fn create_shader_module(&self, filename: &str) -> Result<ShaderModule> {
    self.as_ref().create_shader_module(filename)
  }
  #[allow(clippy::missing_safety_doc)]
  unsafe fn destroy_shader_module(&self, shader_module: ShaderModule) {
    unsafe { self.as_ref().destroy_shader_module(shader_module) }
  }
}

impl<VkC> DeviceContextRef for DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  type VkC = VkC;
  fn as_ref(&self) -> &DeviceContext<VkC> {
    self
  }
}

impl<VkC> DeviceContextRef for Box<DeviceContext<VkC>>
where
  VkC: AsRef<VkContext>,
{
  type VkC = VkC;
  fn as_ref(&self) -> &DeviceContext<VkC> {
    self
  }
}

impl<VkC> DeviceContextRef for ::std::rc::Rc<DeviceContext<VkC>>
where
  VkC: AsRef<VkContext>,
{
  type VkC = VkC;
  fn as_ref(&self) -> &DeviceContext<VkC> {
    self
  }
}

impl<VkC> DeviceContextRef for ::std::sync::Arc<DeviceContext<VkC>>
where
  VkC: AsRef<VkContext>,
{
  type VkC = VkC;
  fn as_ref(&self) -> &DeviceContext<VkC> {
    self
  }
}

impl<VkC> DeviceContextRef for &mut DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  type VkC = VkC;
  fn as_ref(&self) -> &DeviceContext<VkC> {
    self
  }
}

// also implement for references of the above types
impl<T> DeviceContextRef for &T
where
  T: DeviceContextRef,
{
  type VkC = T::VkC;
  fn as_ref(&self) -> &DeviceContext<Self::VkC> {
    T::as_ref(self)
  }
}
