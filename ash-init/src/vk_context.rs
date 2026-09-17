// © ickk 2023-2026, All Rights Reserved.

use {
  crate::{
    debug_callback::{debug_callback_ffi, DebugCallback},
    macros::delegate,
    DeviceRequirements, Features, InstanceRequirements, PresentSupport, Queue,
    QueueRequirements, Version,
  },
  ::ash::{ext, khr, prelude::VkResult, vk, Device, Entry, Instance},
  ::std::{
    collections::HashSet,
    ffi::{c_void, CStr, CString},
  },
};

pub struct VkContext {
  entry: Entry,
  instance: Instance,
  ext_debug_utils_fns: Option<ext::debug_utils::Instance>,
  debug_messengers: Vec<(vk::DebugUtilsMessengerEXT, DebugCallbackPointer)>,
}

// This is required to share VkContext between threads, because of the
// `*mut Box<dyn DebugCallback>`. Generally these pointers need not really be
// used except by vulkan itself anyway.
#[derive(Clone, Debug)]
pub struct DebugCallbackPointer(pub *mut Box<dyn DebugCallback>);
// Safety: `DebugCallback: Send + Sync`
unsafe impl Send for DebugCallbackPointer {}
unsafe impl Sync for DebugCallbackPointer {}

impl VkContext {
  pub fn entry(&self) -> &Entry {
    &self.entry
  }
  pub fn instance(&self) -> &Instance {
    &self.instance
  }
  /// function pointers for the EXT_debug_utils extensions.
  ///
  /// Only present when the extension is enabled.
  pub fn debug_utils(&self) -> Option<&ext::debug_utils::Instance> {
    self.ext_debug_utils_fns.as_ref()
  }
  pub fn debug_messengers(
    &self,
  ) -> &[(vk::DebugUtilsMessengerEXT, DebugCallbackPointer)] {
    &self.debug_messengers
  }
}

impl VkContext {
  /// Create a new `VkContext`
  ///
  /// Only the first `DebugCallback` will receive Loader messages at Instance
  /// creation time.
  ///
  /// `DebugCallback`s are only used when the EXT_debug_utils extension is
  /// enabled.
  pub fn new(
    app: (&str, impl Into<Version>),
    engine: (&str, impl Into<Version>),
    requirements: InstanceRequirements,
    debug_callbacks: Vec<Box<dyn DebugCallback>>,
  ) -> Result<Self, Box<str>> {
    let (app_name, app_version) = app;
    let (engine_name, engine_version) = engine;
    let InstanceRequirements {
      version: requested_version,
      extensions: requested_extensions,
    } = requirements;

    let entry = unsafe { Entry::load().map_err(|_| "Failed to load Entry")? };

    let api_version = {
      let instance_version = unsafe { entry.try_enumerate_instance_version() }
        .unwrap()
        .unwrap_or(vk::make_api_version(0, 1, 0, 0));
      let requested_version = requested_version.to_vk_api_version();
      if requested_version > instance_version {
        return Err(
          "Requested API version is greater than Instance Version".into(),
        );
      }
      requested_version
    };

    let app_name =
      CString::new(app_name).map_err(|_| "app_name contains Null")?;
    let engine_name =
      CString::new(engine_name).map_err(|_| "engine_name contains Null")?;

    let app_info = vk::ApplicationInfo::default()
      .application_name(app_name.as_c_str())
      .application_version(app_version.into().to_vk_api_version())
      .engine_name(engine_name.as_c_str())
      .engine_version(engine_version.into().to_vk_api_version())
      .api_version(api_version);

    // TODO: check the enumerated extension props first
    let extensions = requested_extensions.to_vec_cstr_ptr();

    let mut instance_create_info = vk::InstanceCreateInfo::default()
      .application_info(&app_info)
      .enabled_extension_names(&extensions);

    let instance;

    let (ext_debug_utils_fns, debug_messengers);
    if requested_extensions.ext_debug_utils {
      let debug_messenger_create_infos: Vec<(
        vk::DebugUtilsMessengerCreateInfoEXT,
        *mut Box<dyn DebugCallback>,
      )> = debug_callbacks
        .into_iter()
        .map(|messenger| {
          let severities = messenger.severities();
          let types = messenger.types();
          let p_user_data = Box::into_raw(Box::new(messenger));
          let messenger_create_info =
            vk::DebugUtilsMessengerCreateInfoEXT::default()
              .message_severity(severities)
              .message_type(types)
              .pfn_user_callback(Some(debug_callback_ffi))
              .user_data(p_user_data.cast::<c_void>());

          (messenger_create_info, p_user_data)
        })
        .collect();

      // We allow the first debug messenger to receive Loader messages at
      // Instance creation time <https://docs.vulkan.org/spec/latest/chapters/initialization.html#VkInstanceCreateInfo>.
      let first_debug_messenger_create_info =
        &mut debug_messenger_create_infos
          .first()
          .map(|(messenger, _)| messenger)
          .cloned();
      if let Some(messenger_create_info) = first_debug_messenger_create_info {
        instance_create_info =
          instance_create_info.push_next(messenger_create_info);
      }

      instance = unsafe { entry.create_instance(&instance_create_info, None) }
        .map_err(|_| "Failed to create Instance")?;

      ext_debug_utils_fns =
        Some(ext::debug_utils::Instance::new(&entry, &instance));

      debug_messengers =
        ext_debug_utils_fns.as_ref().map(|ext_debug_utils_fns| {
          debug_messenger_create_infos
            .into_iter()
            .map(|(messenger_create_info, p_user_data)| {
              let messenger = unsafe {
                ext_debug_utils_fns
                  .create_debug_utils_messenger(&messenger_create_info, None)
              }
              .unwrap();

              (messenger, DebugCallbackPointer(p_user_data))
            })
            .collect()
        });
    } else {
      ext_debug_utils_fns = None;
      debug_messengers = None;
      instance = unsafe { entry.create_instance(&instance_create_info, None) }
        .map_err(|_| "Failed to create Instance")?;
    }

    Ok(VkContext {
      entry,
      instance,
      ext_debug_utils_fns,
      debug_messengers: debug_messengers.unwrap_or(Vec::new()),
    })
  }

  /// get a physical device fulfilling requirements
  pub fn get_physical_device(
    &self,
    requirements: &DeviceRequirements,
  ) -> Result<vk::PhysicalDevice, Box<str>> {
    let physical_devices = unsafe { self.enumerate_physical_devices() }
      .map_err(|_| "Failed to enumerate physical devices")?;

    'devices: for device in physical_devices {
      let properties = unsafe { self.get_physical_device_properties(device) };
      {
        // device type
        if let Some(requested_device_type) = requirements.device_type {
          if properties.device_type != requested_device_type.into() {
            continue 'devices;
          }
        }

        // name
        let name_substr = requirements.device_name_substr;
        let device_name =
          unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_str()
            .unwrap();
        if !device_name
          .to_lowercase()
          .contains(&name_substr.to_lowercase())
        {
          continue 'devices;
        }
      }

      {
        // features
        let available_features = {
          let mut features11 = vk::PhysicalDeviceVulkan11Features::default();
          let mut features12 = vk::PhysicalDeviceVulkan12Features::default();
          let mut features13 = vk::PhysicalDeviceVulkan13Features::default();
          let mut khr_fragment_shader_barycentric =
            vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR::default();
          let mut features2 = vk::PhysicalDeviceFeatures2::default()
            .features(vk::PhysicalDeviceFeatures::default())
            .push_next(&mut khr_fragment_shader_barycentric)
            .push_next(&mut features13)
            .push_next(&mut features12)
            .push_next(&mut features11);
          unsafe {
            self.get_physical_device_features2(device, &mut features2)
          };

          Features::from_vk_physical_device_features2(
            features2.features,
            features11,
            features12,
            features13,
            khr_fragment_shader_barycentric,
          )
        };
        if !requirements.features.is_subset(&available_features) {
          continue 'devices;
        }
      }

      {
        // device extensions
        let extensions =
          unsafe { self.enumerate_device_extension_properties(device) }
            .expect("Failed to enumerate device extensions")
            .iter()
            .map(|extension| unsafe {
              CStr::from_ptr(extension.extension_name.as_ptr())
            })
            .collect::<HashSet<_>>();
        let required_extensions = requirements
          .extensions
          .to_vec_cstr()
          .into_iter()
          .collect::<HashSet<_>>();
        if !required_extensions.is_subset(&extensions) {
          continue 'devices;
        }
      }

      // queue requirements
      if self
        .get_queue_create_infos(device, &requirements.queues)
        .is_err()
      {
        continue 'devices;
      }

      return Ok(device);
    }

    Err("Failed to find physical device matching requirements".into())
  }

  /// # Safety
  ///
  /// The caller of this function is responsible for later calling
  /// [`Device::destroy_device`] on the device.
  pub unsafe fn get_logical_device_and_queues(
    &self,
    physical_device: vk::PhysicalDevice,
    requirements: &DeviceRequirements,
  ) -> Result<(ash::Device, Vec<Queue>), Box<str>> {
    let features = requirements.features.features();
    let mut features11 = requirements.features.features11();
    let mut features12 = requirements.features.features12();
    let mut features13 = requirements.features.features13();
    let mut khr_fragment_shader_barycentric =
      requirements.features.khr_fragment_shader_barycentric();
    let mut features2 = vk::PhysicalDeviceFeatures2::default()
      .features(features)
      .push_next(&mut features11)
      .push_next(&mut features12)
      .push_next(&mut features13)
      .push_next(&mut khr_fragment_shader_barycentric);

    let extensions = requirements.extensions.to_vec_cstr_ptr();

    let queue_create_infos =
      self.get_queue_create_infos(physical_device, &requirements.queues)?;

    // create the device
    let device_create_info = vk::DeviceCreateInfo::default()
      .push_next(&mut features2)
      .enabled_extension_names(&extensions)
      .queue_create_infos(&queue_create_infos);

    let device = unsafe {
      self.create_device(physical_device, &device_create_info, None)
    }
    .map_err(|_| "Failed to create device fulfilling requirements.")?;

    // build friendly queue object
    let mut queues = Vec::new();
    for (queue_create_info, queue_requirements) in
      ::core::iter::zip(queue_create_infos, &requirements.queues)
    {
      for index in 0..queue_create_info.queue_count {
        let handle = unsafe {
          device.get_device_queue(queue_create_info.queue_family_index, index)
        };

        queues.push(Queue {
          name: queue_requirements.names[index as usize].into(),
          index,
          family_index: queue_create_info.queue_family_index,
          priority: queue_requirements.priorities[index as usize],
          handle,
          capabilities: queue_requirements.capabilities,
        })
      }
    }

    Ok((device, queues))
  }

  // find each of the queues in queue_requirements and generate the respective
  // `vk::DeviceQueueCreateInfo`s
  fn get_queue_create_infos<'r>(
    &self,
    physical_device: vk::PhysicalDevice,
    queue_requirements: &'r [QueueRequirements],
  ) -> Result<Vec<vk::DeviceQueueCreateInfo<'r>>, Box<str>> {
    for requirements in queue_requirements {
      if requirements.names.len() != requirements.priorities.len() {
        return Err(
          "number of priorities must be equal to the number of names".into(),
        );
      }
    }

    let queue_family_properties = unsafe {
      self.get_physical_device_queue_family_properties(physical_device)
    };
    let mut queue_family_properties =
      queue_family_properties.iter().map(Some).collect::<Vec<_>>();
    let mut queue_create_infos = Vec::new();
    for queue_requirements in queue_requirements.iter() {
      let required_capabilities =
        vk::QueueFlags::from(queue_requirements.capabilities);
      let mut index = None;

      for (i, queue_family) in queue_family_properties.iter().enumerate() {
        if let Some(queue_family) = queue_family {
          // check for queue_count and flags
          if queue_family.queue_count
            >= queue_requirements.priorities.len() as u32
            && queue_family.queue_flags.contains(required_capabilities)
          {
            // check for presentation support
            match queue_requirements.present_support {
              None => {
                queue_create_infos.push(
                  vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(i as u32)
                    .queue_priorities(&queue_requirements.priorities),
                );
                index = Some(i);
                break;
              },
              Some(PresentSupport::Win32) => {
                let khr_win32_surface_fns = khr::win32_surface::Instance::new(
                  &self.entry,
                  &self.instance,
                );
                if unsafe {
                  khr_win32_surface_fns
                    .get_physical_device_win32_presentation_support(
                      physical_device,
                      i as u32,
                    )
                } {
                  queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::default()
                      .queue_family_index(i as u32)
                      .queue_priorities(&queue_requirements.priorities),
                  );
                  index = Some(i);
                  break;
                }
              },
              Some(PresentSupport::Xcb {
                mut connection,
                visual_id,
              }) => {
                let khr_xcb_surface_fns =
                  khr::xcb_surface::Instance::new(&self.entry, &self.instance);
                if unsafe {
                  khr_xcb_surface_fns
                    .get_physical_device_xcb_presentation_support(
                      physical_device,
                      i as u32,
                      connection.as_mut(),
                      visual_id,
                    )
                } {
                  queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::default()
                      .queue_family_index(i as u32)
                      .queue_priorities(&queue_requirements.priorities),
                  );
                  index = Some(i);
                  break;
                }
              },
              Some(PresentSupport::Xlib {
                mut display,
                visual_id,
              }) => {
                let khr_xlib_surface_fns = khr::xlib_surface::Instance::new(
                  &self.entry,
                  &self.instance,
                );
                if unsafe {
                  khr_xlib_surface_fns
                    .get_physical_device_xlib_presentation_support(
                      physical_device,
                      i as u32,
                      display.as_mut(),
                      #[allow(clippy::useless_conversion)]
                      visual_id.try_into().expect("visual_id didn't fit"),
                    )
                } {
                  queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::default()
                      .queue_family_index(i as u32)
                      .queue_priorities(&queue_requirements.priorities),
                  );
                  index = Some(i);
                  break;
                }
              },
              // _ => unimplemented!(),
            }
          }
        }
      }

      if let Some(index) = index {
        queue_family_properties[index] = None;
      }
    }
    if queue_create_infos.len() == queue_requirements.len() {
      Ok(queue_create_infos)
    } else {
      Err("failed to find all required queues".into())
    }
  }
}

impl Drop for VkContext {
  fn drop(&mut self) {
    // kill the messengers
    if let Some(debug_utils) = self.debug_utils() {
      for &(messenger, _) in self.debug_messengers().iter() {
        unsafe {
          debug_utils.destroy_debug_utils_messenger(messenger, None);
        }
      }
    }

    unsafe {
      self.instance.destroy_instance(None);
    }

    // Free the DebugCallbacks.
    // This happens last because there may be some callbacks still listening as
    // late as the call to `destroy_instance`.
    for (_, debug_callback_pointer) in self.debug_messengers().iter() {
      let _ = unsafe { Box::from_raw(debug_callback_pointer.0) };
    }
  }
}

/// Delegated to [`ash::Entry`]
impl VkContext {
  delegate! {
    to self.entry {
      pub unsafe fn try_enumerate_instance_version(
        &self,
      ) -> VkResult<Option<u32>>;

      pub unsafe fn enumerate_instance_layer_properties(
        &self,
      ) -> VkResult<Vec<vk::LayerProperties>>;

      pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&CStr>,
      ) -> VkResult<Vec<vk::ExtensionProperties>>;
    }
  }
}

/// Delegated to [`ash::Instance`]
#[allow(clippy::missing_safety_doc)]
impl VkContext {
  delegate! {
    to self.instance {
      pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
      ) -> VkResult<Device>;

      pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
      ) -> vk::FormatProperties;

      pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        typ: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
      ) -> VkResult<vk::ImageFormatProperties>;

      pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: vk::PhysicalDevice,
      ) -> vk::PhysicalDeviceMemoryProperties;

      pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: vk::PhysicalDevice,
      ) -> vk::PhysicalDeviceProperties;

      pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: vk::PhysicalDevice,
      ) -> Vec<vk::QueueFamilyProperties>;

      pub unsafe fn get_physical_device_features(
        &self,
        physical_device: vk::PhysicalDevice,
      ) -> vk::PhysicalDeviceFeatures;

      pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
        features2: &mut vk::PhysicalDeviceFeatures2,
      );

      pub unsafe fn enumerate_physical_devices(
        &self,
      ) -> VkResult<Vec<vk::PhysicalDevice>>;

      pub unsafe fn enumerate_device_extension_properties(
        &self,
        device: vk::PhysicalDevice,
      ) -> VkResult<Vec<vk::ExtensionProperties>>;

      pub unsafe fn enumerate_device_layer_properties(
        &self,
        device: vk::PhysicalDevice,
      ) -> VkResult<Vec<vk::LayerProperties>>;

      pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        typ: vk::ImageType,
        samples: vk::SampleCountFlags,
        usage: vk::ImageUsageFlags,
        tiling: vk::ImageTiling,
      ) -> Vec<vk::SparseImageFormatProperties>;
    }
  }
}
