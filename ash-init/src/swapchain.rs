// © ickk 2023-2026, All Rights Reserved.

use {
  crate::{
    DeviceContext, DeviceContextRef, Queue, Result, Surface, SurfaceRef,
    SwapchainFrame,
  },
  ::ash::{khr, vk, Device},
};

const SURFACE_FORMAT: vk::SurfaceFormatKHR = vk::SurfaceFormatKHR {
  format: vk::Format::B8G8R8A8_UNORM,
  color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
};

pub struct Swapchain<S: SurfaceRef> {
  // pub(crate) device_context: DC,
  pub(crate) khr_swapchain_fns: khr::swapchain::Device,
  pub(crate) queue: Queue,
  pub(crate) surface: S,

  pub(crate) handle: vk::SwapchainKHR,
  pub(crate) format: vk::Format,
  pub(crate) color_space: vk::ColorSpaceKHR,
  pub(crate) extent: vk::Extent2D,
  pub(crate) images: Vec<vk::Image>,
  pub(crate) image_views: Vec<vk::ImageView>,
  // fresh semaphores and fences
  pub(crate) semaphores: Vec<vk::Semaphore>,
  pub(crate) semaphore_count: usize, // number of semaphores spawned
  pub(crate) fences: Vec<vk::Fence>,
  pub(crate) fence_count: usize, // number of semaphores spawned
  // workaround for lack of `VK_EXT_swapchain_maintenance1`
  // <https://docs.vulkan.org/samples/latest/samples/api/swapchain_recreation/README.html>
  pub(crate) present_history: Vec<SwapchainFrame>,
  pub(crate) in_flight: Vec<SwapchainFrame>,
}

impl<S: SurfaceRef> Swapchain<S> {
  #[inline]
  pub fn count(&self) -> usize {
    self.images.len()
  }
  #[inline]
  pub fn queue(&self) -> &Queue {
    &self.queue
  }
  #[inline]
  pub fn handle(&self) -> vk::SwapchainKHR {
    self.handle
  }
  #[inline]
  pub fn format(&self) -> vk::Format {
    self.format
  }
  #[inline]
  pub fn color_space(&self) -> vk::ColorSpaceKHR {
    self.color_space
  }
  #[inline]
  pub fn extent(&self) -> vk::Extent2D {
    self.extent
  }
  #[inline]
  pub fn surface(&self) -> &Surface<'_, <S as SurfaceRef>::D> {
    self.surface.as_ref()
  }
  #[inline]
  pub fn device_context(
    &self,
  ) -> &DeviceContext<<<S as SurfaceRef>::D as DeviceContextRef>::VkC> {
    self.surface.device_context()
  }
  #[inline]
  pub fn device(&self) -> &Device {
    self.device_context().device()
  }
}

impl<S: SurfaceRef> Swapchain<S> {
  /// `count` specifies the number of images in the swapchain
  pub fn new(
    surface: S,
    queue: Queue,
    count: u32,
    present_mode: vk::PresentModeKHR,
  ) -> Result<Self> {
    let s = surface.as_ref();

    let vk::SurfaceFormatKHR {
      format,
      color_space,
    } = if s.formats()?.contains(&SURFACE_FORMAT) {
      SURFACE_FORMAT
    } else {
      return Err("Surface doesn't support desired format".into()); // TODO
    };
    let present_mode = if s.present_modes()?.contains(&present_mode) {
      present_mode
    } else {
      return Err("surface doesn't support desired present mode".into()); // TODO
    };
    let capabilities = s.capabilities()?;
    let extent = match capabilities.current_extent {
      vk::Extent2D {
        width: 0xffff_ffff,
        height: 0xffff_ffff,
      } => return Err("unspecified surface extent".into()), // TODO
      extent => extent,
    };
    use vk::ImageUsageFlags as F;
    let usage = F::COLOR_ATTACHMENT | F::TRANSFER_DST;
    let sharing_mode = vk::SharingMode::EXCLUSIVE;
    let queue_families = [queue.family_index()];
    let pre_transform = capabilities.current_transform;
    let composite = vk::CompositeAlphaFlagsKHR::OPAQUE;
    let clip = false;

    let create_info = vk::SwapchainCreateInfoKHR::default()
      .surface(s.handle())
      .min_image_count(count)
      .image_format(format)
      .image_color_space(color_space)
      .image_extent(extent)
      .image_array_layers(1)
      .image_usage(usage)
      .image_sharing_mode(sharing_mode)
      .queue_family_indices(&queue_families)
      .pre_transform(pre_transform)
      .composite_alpha(composite)
      .present_mode(present_mode)
      .clipped(clip);

    let khr_swapchain_fns =
      khr::swapchain::Device::new(s.vk_context().instance(), s.device());

    let swapchain_handle =
      unsafe { khr_swapchain_fns.create_swapchain(&create_info, None) }
        .map_err(|_| "failed to create swapchain")?;
    s.device_context()
      .set_debug_object_name(swapchain_handle, "Swapchain")?;

    let images =
      unsafe { khr_swapchain_fns.get_swapchain_images(swapchain_handle) }
        .map_err(|_| "failed to get swapchain images")?;

    let mut image_views = Vec::with_capacity(images.len());
    for (i, &image) in images.iter().enumerate() {
      let create_info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .subresource_range(vk::ImageSubresourceRange {
          level_count: 1,
          base_mip_level: 0,
          layer_count: 1,
          base_array_layer: 0,
          aspect_mask: vk::ImageAspectFlags::COLOR,
        });
      let image_view =
        unsafe { s.device().create_image_view(&create_info, None) }
          .map_err(|_| "failed to create swapchain image view")?;
      image_views.push(image_view);

      s.device_context()
        .set_debug_object_name(image, format!("Image(Swapchain: {i})"))?;
      s.device_context().set_debug_object_name(
        image_view,
        format!("ImageView(Swapchain: {i})"),
      )?;
    }

    let semaphores = Vec::with_capacity(images.len());
    let fences = Vec::with_capacity(images.len());
    let present_history = Vec::with_capacity(images.len());
    let in_flight = Vec::with_capacity(images.len());

    Ok(Swapchain {
      khr_swapchain_fns,
      queue,
      surface,
      handle: swapchain_handle,
      format,
      color_space,
      extent,
      images,
      image_views,
      semaphore_count: semaphores.len(),
      semaphores,
      fence_count: fences.len(),
      fences,
      present_history,
      in_flight,
    })
  }

  pub fn next_frame(&mut self) -> Result<SwapchainFrame> {
    let image_acquired_fence = if let Some(fence) = self.fences.pop() {
      fence
    } else {
      let fence = self.device_context().create_fence(false)?;
      ::tracing::warn!("creating fence");
      self.device_context().set_debug_object_name(
        fence,
        format!("Fence(Swapchain: {})", self.fence_count),
      )?;
      self.fence_count += 1;
      fence
    };
    let image_acquired = if let Some(semaphore) = self.semaphores.pop() {
      semaphore
    } else {
      let semaphore = self.device_context().create_binary_semaphore()?;
      ::tracing::warn!("creating binary_semaphore");
      self.device_context().set_debug_object_name(
        semaphore,
        format!("Semaphore(Swapchain: {})", self.semaphore_count),
      )?;
      self.semaphore_count += 1;
      semaphore
    };
    let image_ready_to_present = if let Some(semaphore) = self.semaphores.pop()
    {
      semaphore
    } else {
      let semaphore = self.device_context().create_binary_semaphore()?;
      ::tracing::warn!("creating binary_semaphore");
      self.device_context().set_debug_object_name(
        semaphore,
        format!("Semaphore(Swapchain: {})", self.semaphore_count),
      )?;
      self.semaphore_count += 1;
      semaphore
    };

    let (image_index, _suboptimal) = unsafe {
      self.khr_swapchain_fns.acquire_next_image(
        self.handle,
        u64::MAX, // intel & amd gpus seem to fail shorter timeouts
        image_acquired,
        image_acquired_fence,
      )
    }
    .map_err(|_| "failed to acquire next image")?;

    // give previous frame of same index new image_available_fence, this will
    // signal when it's safe to clean up the previous frame
    if let Some(f) = self
      .present_history
      .iter_mut()
      .find(|f| f.image_index == image_index)
    {
      f.next_acquire_fence = Some(image_acquired_fence);
      // .map(|f| f.next_acquire_fence = Some(image_acquired_fence));
    }

    // if suboptimal {
    //   todo!("recreate swapchain")
    // }

    self.in_flight.push(SwapchainFrame {
      swapchain_handle: self.handle,
      image_index,
      image_acquired,
      image_acquired_fence,
      image_ready_to_present,
      extent: self.extent,
      image: self.images[image_index as usize],
      image_view: self.image_views[image_index as usize],
      next_acquire_fence: None,
    });

    // return `SwapchainFrame` rather than `&SwapchainFrame` because we want `present` to take
    // ownership of the object so that the caller can't present the same frame
    // twice.
    Ok(SwapchainFrame {
      ..self.in_flight[self.in_flight.len() - 1] // cheeky clone
    })
  }

  // pub fn current_frame(&self) -> Result<SwapchainFrame> {
  //   let most_recent_frame =
  //     self.in_flight.get(self.in_flight.len() - 1).unwrap();
  //   Ok(SwapchainFrame {
  //     ..*most_recent_frame
  //   })
  // }

  pub fn present(
    &mut self,
    frame: SwapchainFrame,
    wait_semaphores: &[vk::Semaphore],
  ) -> Result<()> {
    self.in_flight.retain(|f| *f != frame);

    let swapchains = [self.handle];
    let image_indices = [frame.image_index];
    let present_info = vk::PresentInfoKHR::default()
      .wait_semaphores(wait_semaphores)
      .swapchains(&swapchains)
      .image_indices(&image_indices);

    unsafe {
      self
        .khr_swapchain_fns
        .queue_present(self.queue.handle(), &present_info)
    }
    .map_err(|_| "failed to present queue")?;

    reset_old_frame_data(self)?;
    self.present_history.push(frame);

    Ok(())
  }
}

/// remove frames whose `next_acquire_fence` has signaled
fn reset_old_frame_data<S: SurfaceRef>(
  swapchain: &mut Swapchain<S>,
) -> Result<()> {
  for i in (0..swapchain.present_history.len()).rev() {
    let frame = &swapchain.present_history[i];

    if let Some(fence) = frame.next_acquire_fence {
      let fence_status = unsafe {
        swapchain.device_context().device().get_fence_status(fence)
      }?;
      if fence_status {
        // not sure why but it's necessary to check this fence before reset
        let _image_acquired_fence_status = unsafe {
          swapchain
            .device_context()
            .device()
            .get_fence_status(frame.image_acquired_fence)
        }?;
        let fences = [frame.image_acquired_fence];
        unsafe {
          swapchain
            .device()
            .reset_fences(&fences)
            .map_err(|_| "failed to reset fence")?
        };
        swapchain.fences.push(frame.image_acquired_fence);
        swapchain.semaphores.push(frame.image_acquired);
        swapchain.semaphores.push(frame.image_ready_to_present);

        swapchain.present_history.remove(i);
      }
    }
  }

  Ok(())
}

impl<S: SurfaceRef> Drop for Swapchain<S> {
  fn drop(&mut self) {
    reset_old_frame_data(self).unwrap();

    unsafe { self.device().queue_wait_idle(self.queue().handle()) }.unwrap();

    for SwapchainFrame {
      image_acquired,
      image_acquired_fence,
      image_ready_to_present,
      ..
    } in self.present_history.iter().cloned()
    {
      unsafe {
        self.device().destroy_fence(image_acquired_fence, None);
        self.device().destroy_semaphore(image_acquired, None);
        self
          .device()
          .destroy_semaphore(image_ready_to_present, None);
      }
    }
    for &semaphore in &self.semaphores {
      unsafe { self.device().destroy_semaphore(semaphore, None) };
    }
    for &fence in &self.fences {
      unsafe { self.device().destroy_fence(fence, None) };
    }
    for &image_view in &self.image_views {
      unsafe { self.device().destroy_image_view(image_view, None) }
    }
    unsafe { self.khr_swapchain_fns.destroy_swapchain(self.handle, None) };
  }
}
