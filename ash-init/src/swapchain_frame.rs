use ::ash::vk;

#[derive(PartialEq, Clone, Debug)]
pub struct SwapchainFrame {
  pub(crate) image_index: u32,
  pub(crate) swapchain_handle: vk::SwapchainKHR,
  pub(crate) image_acquired_fence: vk::Fence,
  // to be populated with the image_acquired_fence of the following frame
  pub(crate) next_acquire_fence: Option<vk::Fence>,

  pub(crate) extent: vk::Extent2D,
  pub(crate) image: vk::Image,
  pub(crate) image_view: vk::ImageView,
  // used to block drawing until the swapchain signals the image is available
  pub(crate) image_acquired: vk::Semaphore,
  // used by the application to signal when the image is read to present
  pub(crate) image_ready_to_present: vk::Semaphore,
}

impl SwapchainFrame {
  #[inline]
  pub fn index(&self) -> u32 {
    self.image_index
  }
  #[inline]
  pub fn extent(&self) -> vk::Extent2D {
    self.extent
  }
  #[inline]
  pub fn image_handle(&self) -> vk::Image {
    self.image
  }
  #[inline]
  pub fn image_view(&self) -> vk::ImageView {
    self.image_view
  }
  #[inline]
  pub fn image_acquired(&self) -> vk::Semaphore {
    self.image_acquired
  }
  #[inline]
  pub fn image_ready_to_present(&self) -> vk::Semaphore {
    self.image_ready_to_present
  }
  #[inline]
  pub fn next_acquire_fence(&self) -> Option<vk::Fence> {
    self.next_acquire_fence
  }
}
