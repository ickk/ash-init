use {
  crate::{DeviceContext, Result, VkContext},
  ::ash::vk,
};

#[derive(Clone, Debug)]
pub struct Image {
  pub(crate) handle: vk::Image,
  pub(crate) memory: vk::DeviceMemory,
  pub(crate) extent2d: vk::Extent2D,
  pub(crate) aspect: vk::ImageAspectFlags,
  pub(crate) image_views: Vec<(vk::Format, vk::ImageView)>,
}

impl Image {
  #[inline]
  pub fn memory(&self) -> vk::DeviceMemory {
    self.memory
  }
  #[inline]
  pub fn handle(&self) -> vk::Image {
    self.handle
  }
  #[inline]
  pub fn extent3d(&self) -> vk::Extent3D {
    vk::Extent3D {
      width: self.extent2d.width,
      height: self.extent2d.height,
      depth: 1,
    }
  }
  #[inline]
  pub fn extent2d(&self) -> vk::Extent2D {
    self.extent2d
  }
  #[inline]
  pub fn aspect(&self) -> vk::ImageAspectFlags {
    self.aspect
  }
  #[inline]
  pub fn image_view(&self, index: usize) -> vk::ImageView {
    self.image_views[index].1
  }
  #[inline]
  pub fn format(&self, index: usize) -> vk::Format {
    self.image_views[index].0
  }
}

impl<VkC> DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  pub fn create_image(
    &self,
    extent2d: vk::Extent2D,
    format: vk::Format,
    tiling: vk::ImageTiling,
    usage: vk::ImageUsageFlags,
    requested_memory_properties: vk::MemoryPropertyFlags,
    aspect: vk::ImageAspectFlags,
    render_target: bool,
  ) -> Result<Image> {
    let mut queue_family_indices = self
      .queues()
      .iter()
      .map(|q| q.family_index())
      .collect::<Vec<_>>();
    let mut sharing_mode = if queue_family_indices.len() > 1 {
      vk::SharingMode::CONCURRENT
    } else {
      vk::SharingMode::EXCLUSIVE
    };
    if render_target {
      queue_family_indices = self
        .queues
        .first()
        .into_iter()
        .map(|q| q.family_index())
        .collect();
      sharing_mode = vk::SharingMode::EXCLUSIVE;
    }
    let image_create_info = vk::ImageCreateInfo::default()
      .flags(vk::ImageCreateFlags::empty())
      .image_type(vk::ImageType::TYPE_2D)
      .format(format)
      .extent(vk::Extent3D {
        width: extent2d.width,
        height: extent2d.height,
        depth: 1,
      })
      .mip_levels(1)
      .array_layers(1)
      .samples(vk::SampleCountFlags::TYPE_1)
      .tiling(tiling)
      .usage(usage)
      .queue_family_indices(&queue_family_indices)
      .sharing_mode(sharing_mode);
    let handle =
      unsafe { self.device.create_image(&image_create_info, None) }?;
    let memory_requirements =
      unsafe { self.device.get_image_memory_requirements(handle) };

    let memory_type_index = 'mem_type_index: {
      for (i, memory_type) in
        self.memory_properties.memory_types.iter().enumerate()
      {
        if (memory_requirements.memory_type_bits & (1 << i)) != 0
          && (memory_type.property_flags & requested_memory_properties)
            == requested_memory_properties
        {
          break 'mem_type_index i as u32;
        }
      }
      return Err("failed to find suitable memory type".into());
    };

    let memory_allocate_info = vk::MemoryAllocateInfo::default()
      .allocation_size(memory_requirements.size)
      .memory_type_index(memory_type_index);
    let memory =
      unsafe { self.device.allocate_memory(&memory_allocate_info, None) }?;
    unsafe { self.device.bind_image_memory(handle, memory, 0) }?;

    let image_view_create_info = vk::ImageViewCreateInfo::default()
      .view_type(vk::ImageViewType::TYPE_2D)
      .image(handle)
      .format(format)
      .subresource_range(vk::ImageSubresourceRange {
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
        aspect_mask: aspect,
      });

    let mut image_views = Vec::new();
    image_views.push((format, unsafe {
      self.device.create_image_view(&image_view_create_info, None)
    }?));

    Ok(Image {
      memory,
      handle,
      extent2d,
      aspect,
      image_views,
    })
  }

  pub fn add_image_view(
    &self,
    image: &mut Image,
    format: vk::Format,
  ) -> Result<usize> {
    if let Some(existing_image_view_index) = image
      .image_views
      .iter()
      .enumerate()
      .find(|(_, &(f, _))| f == format)
      .map(|(index, _)| index)
    {
      Ok(existing_image_view_index)
    } else {
      let new_index = image.image_views.len();

      let image_view_create_info = vk::ImageViewCreateInfo::default()
        .view_type(vk::ImageViewType::TYPE_2D)
        .image(image.handle)
        .format(format)
        .subresource_range(vk::ImageSubresourceRange {
          base_mip_level: 0,
          level_count: 1,
          base_array_layer: 0,
          layer_count: 1,
          aspect_mask: image.aspect,
        });

      image.image_views.push((format, unsafe {
        self.device.create_image_view(&image_view_create_info, None)
      }?));

      Ok(new_index)
    }
  }

  #[allow(clippy::missing_safety_doc)]
  pub unsafe fn destroy_image(&self, image: &Image) {
    unsafe {
      for &(_, image_view) in &image.image_views {
        self.device.destroy_image_view(image_view, None);
      }
      self.device.destroy_image(image.handle(), None);
      self.device.free_memory(image.memory(), None);
    }
  }
}
