// © ickk 2023-2026, All Rights Reserved.

use {
  crate::{
    debug::Object, DeviceRequirements, ErrorList, Queue, Result, VkContext,
  },
  ::ash::{ext, vk, Device, Instance},
  ::core::fmt,
};

pub struct DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  pub(crate) vk_context: VkC,
  pub(crate) physical_device: vk::PhysicalDevice,
  pub(crate) device: Device,
  pub(crate) queues: Vec<Queue>,
  pub(crate) memory_properties: vk::PhysicalDeviceMemoryProperties,
  pub(crate) ext_debug_utils_fns: Option<ext::debug_utils::Device>,
}

impl<VkC> fmt::Debug for DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("DeviceContext")
      .field("physical_device", &self.physical_device)
      .field("queues", &self.queues)
      .finish()
  }
}

impl<VkC> DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  #[inline]
  pub fn vk_context(&self) -> &VkContext {
    self.vk_context.as_ref()
  }
  #[inline]
  pub fn instance(&self) -> &Instance {
    self.vk_context().instance()
  }
  #[inline]
  pub fn physical_device(&self) -> vk::PhysicalDevice {
    self.physical_device
  }
  #[inline]
  pub fn device(&self) -> &Device {
    &self.device
  }
  #[inline]
  pub fn queues(&self) -> &[Queue] {
    &self.queues
  }
  #[inline]
  pub fn get_queue(&self, name: &str) -> Option<&Queue> {
    self.queues.iter().find(|queue| queue.name() == name)
  }
  #[inline]
  pub fn queue(&self, name: &str) -> &Queue {
    self
      .get_queue(name)
      .unwrap_or_else(|| panic!("failed to find queue with name: {name}"))
  }
  #[inline]
  pub fn memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
    &self.memory_properties
  }
  /// function pointers for the EXT_debug_utils extensions.
  ///
  /// Only present when the extension is enabled.
  pub fn debug_utils(&self) -> Option<&ext::debug_utils::Device> {
    self.ext_debug_utils_fns.as_ref()
  }
}

impl<VkC> DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  pub fn new(
    vk_context: VkC,
    requirements: DeviceRequirements,
  ) -> Result<Self> {
    let vkc = vk_context.as_ref();

    let physical_device = vkc
      .get_physical_device(&requirements)
      .map_err(|_| "failed to get physical device")?;

    let (device, queues) = unsafe {
      vkc.get_logical_device_and_queues(physical_device, &requirements)
    }
    .map_err(|_| "failed to get logical device and queues")?;

    let memory_properties =
      unsafe { vkc.get_physical_device_memory_properties(physical_device) };

    let ext_debug_utils_fns = if vkc.debug_utils().is_some() {
      Some(ext::debug_utils::Device::new(vkc.instance(), &device))
    } else {
      None
    };

    if let Some(debug_utils) = &ext_debug_utils_fns {
      for queue in &queues {
        let name = ::std::ffi::CString::new(queue.name()).unwrap();
        let name_info = vk::DebugUtilsObjectNameInfoEXT::default()
          .object_handle(queue.handle)
          .object_name(&name);
        let _ = unsafe { debug_utils.set_debug_utils_object_name(&name_info) };
      }
    }

    Ok(DeviceContext {
      vk_context,
      physical_device,
      device,
      queues,
      memory_properties,
      ext_debug_utils_fns,
    })
  }

  #[inline]
  pub fn create_binary_semaphore(&self) -> Result<vk::Semaphore> {
    let create_info = vk::SemaphoreCreateInfo::default();
    unsafe { self.device.create_semaphore(&create_info, None) }
      .map_err(|e| e.into())
  }

  #[inline]
  pub fn create_timeline_semaphore(&self) -> Result<vk::Semaphore> {
    let mut type_info = vk::SemaphoreTypeCreateInfo::default()
      .semaphore_type(vk::SemaphoreType::TIMELINE)
      .initial_value(0);
    let create_info =
      vk::SemaphoreCreateInfo::default().push_next(&mut type_info);
    unsafe {
      self
        .device
        .create_semaphore(&create_info, None)
        .map_err(|e| e.into())
    }
  }

  #[inline]
  pub fn create_fence(&self, signaled: bool) -> Result<vk::Fence> {
    let mut create_info = vk::FenceCreateInfo::default();
    if signaled {
      create_info = create_info.flags(vk::FenceCreateFlags::SIGNALED);
    }
    unsafe { self.device.create_fence(&create_info, None) }
      .map_err(|e| e.into())
  }

  pub fn get_fence_status(&self, fence: vk::Fence) -> Result<bool> {
    unsafe { self.device.get_fence_status(fence).map_err(|e| e.into()) }
  }

  /// Set the EXT_debug_utils object name
  ///
  /// This is a no-op when the EXT_debug_utils extension is not enabled.
  #[inline]
  pub fn set_debug_object_name(
    &self,
    object_handle: impl vk::Handle,
    name: impl ::core::fmt::Display,
  ) -> Result<()> {
    if let Some(debug_utils) = self.debug_utils() {
      let name: &str = &name.to_string();
      let name = ::std::ffi::CString::new(name)?;

      let debug_info = vk::DebugUtilsObjectNameInfoEXT::default()
        .object_handle(object_handle)
        .object_name(&name);

      return unsafe { debug_utils.set_debug_utils_object_name(&debug_info) }
        .map_err(|e| e.into());
    }

    Result::Ok(())
  }

  /// Set the EXT_debug_utils object name for several objects
  ///
  /// This is a no-op when the EXT_debug_utils extension is not enabled.
  #[inline]
  pub fn set_debug_object_name_many(
    &self,
    object_handles: &[Object],
    name: impl ::core::fmt::Display,
  ) -> Result<()> {
    if let Some(debug_utils) = self.debug_utils() {
      let name: &str = &name.to_string();
      let name = ::std::ffi::CString::new(name)?;

      let mut errors = Vec::new();

      for &object in object_handles {
        let mut debug_info =
          vk::DebugUtilsObjectNameInfoEXT::default().object_name(&name);
        debug_info.object_type = object.object_type();
        debug_info.object_handle = object.raw_object_handle();

        if let Err(e) =
          unsafe { debug_utils.set_debug_utils_object_name(&debug_info) }
        {
          errors.push(e)
        }
      }

      if !errors.is_empty() {
        return Result::Err(ErrorList::from(errors).into());
      }
    }

    Result::Ok(())
  }

  /// create a buffer with vk::SharingMode::EXCLUSIVE
  ///
  /// This means the buffer will only be accessed from a single queue-family
  #[allow(clippy::missing_safety_doc)]
  pub unsafe fn create_buffer_exclusive(
    &self,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
  ) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    let ref device = self.device;

    let queue_family_indices = [];
    let buffer_create_info = vk::BufferCreateInfo::default()
      .flags(vk::BufferCreateFlags::empty())
      .size(size)
      .usage(usage)
      .sharing_mode(vk::SharingMode::EXCLUSIVE)
      .queue_family_indices(&queue_family_indices);

    let buffer = unsafe { device.create_buffer(&buffer_create_info, None)? };

    let memory_requirements =
      unsafe { device.get_buffer_memory_requirements(buffer) };

    let memory_type_index = 'block: {
      let memory_properties = unsafe {
        self
          .vk_context
          .as_ref()
          .instance()
          .get_physical_device_memory_properties(self.physical_device)
      };

      // spec says `mem_requirements.memory_type_bits & (1 << i)` is set for
      // supported memory types only when `memory_properties.memory_types[i]` is
      // supported for the resource.
      for (i, memory_type) in memory_properties.memory_types.iter().enumerate()
      {
        if (memory_requirements.memory_type_bits & (1 << i)) != 0
          && (memory_type.property_flags & properties) == properties
        {
          break 'block i as u32;
        }
      }

      return Err(
        format!("failed to find suitable memory type: {properties:?}").into(),
      );
    };

    let mut memory_allocate_info = vk::MemoryAllocateInfo::default()
      .allocation_size(memory_requirements.size)
      .memory_type_index(memory_type_index);
    let mut memory_allocate_flags_info =
      vk::MemoryAllocateFlagsInfo::default();
    if usage.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS) {
      memory_allocate_flags_info.flags |=
        vk::MemoryAllocateFlags::DEVICE_ADDRESS;
      memory_allocate_info =
        memory_allocate_info.push_next(&mut memory_allocate_flags_info);
    }

    let buffer_memory =
      unsafe { device.allocate_memory(&memory_allocate_info, None)? };

    unsafe {
      device.bind_buffer_memory(buffer, buffer_memory, 0)?;
    }

    Ok((buffer, buffer_memory))
  }
}

impl<VkC> Drop for DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  fn drop(&mut self) {
    unsafe { self.device.destroy_device(None) };
  }
}
