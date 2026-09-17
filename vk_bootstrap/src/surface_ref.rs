use {
  crate::{DeviceContext, DeviceContextRef, Surface, VkContext},
  ::ash::{vk, Device},
  ::raw_window_handle::WindowHandle,
};

pub trait SurfaceRef {
  type D: DeviceContextRef;
  fn as_ref(&self) -> &Surface<'_, Self::D>;

  fn vk_context(&self) -> &VkContext {
    self.as_ref().vk_context()
  }
  fn device_context(
    &self,
  ) -> &DeviceContext<<Self::D as DeviceContextRef>::VkC> {
    self.as_ref().device_context()
  }
  fn device(&self) -> &Device {
    self.as_ref().device()
  }
  fn window_handle(&self) -> WindowHandle<'_> {
    self.as_ref().window_handle()
  }
  fn handle(&self) -> vk::SurfaceKHR {
    self.as_ref().handle()
  }
}

impl<D> SurfaceRef for Surface<'_, D>
where
  D: DeviceContextRef,
{
  type D = D;
  fn as_ref(&self) -> &Surface<'_, D> {
    self
  }
}
impl<D> SurfaceRef for Box<Surface<'_, D>>
where
  D: DeviceContextRef,
{
  type D = D;
  fn as_ref(&self) -> &Surface<'_, D> {
    self
  }
}
impl<D> SurfaceRef for ::std::rc::Rc<Surface<'_, D>>
where
  D: DeviceContextRef,
{
  type D = D;
  fn as_ref(&self) -> &Surface<'_, D> {
    self
  }
}
impl<D> SurfaceRef for ::std::sync::Arc<Surface<'_, D>>
where
  D: DeviceContextRef,
{
  type D = D;
  fn as_ref(&self) -> &Surface<'_, D> {
    self
  }
}
impl<D> SurfaceRef for &Surface<'_, D>
where
  D: DeviceContextRef,
{
  type D = D;
  fn as_ref(&self) -> &Surface<'_,D> {
    self
  }
}
impl<D> SurfaceRef for &mut Surface<'_, D>
where
  D: DeviceContextRef,
{
  type D = D;
  fn as_ref(&self) -> &Surface<'_, D> {
    self
  }
}
