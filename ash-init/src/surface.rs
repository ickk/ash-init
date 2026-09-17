// © ickk 2023-2026, All Rights Reserved.

use {
  crate::{
    present_support::{HasDisplayHandleExt, HasWindowHandleExt},
    DeviceContext, DeviceContextRef, Platform, Result, VkContext,
  },
  ::ash::{khr, vk, Device},
  ::raw_window_handle::{RawWindowHandle, WindowHandle},
};

pub struct Surface<'w, D: DeviceContextRef> {
  pub(crate) device_context: D,
  pub(crate) khr_surface_fns: khr::surface::Instance,
  pub(crate) window_handle: WindowHandle<'w>,
  pub(crate) handle: vk::SurfaceKHR,
}

impl<'w, D: DeviceContextRef> Surface<'w, D> {
  #[inline]
  pub fn vk_context(&self) -> &VkContext {
    self.device_context.as_ref().vk_context()
  }
  #[inline]
  pub fn device_context(
    &self,
  ) -> &DeviceContext<<D as DeviceContextRef>::VkC> {
    self.device_context.as_ref()
  }
  #[inline]
  pub fn device(&self) -> &Device {
    self.device_context().device()
  }
  #[inline]
  pub fn window_handle(&self) -> WindowHandle<'_> {
    self.window_handle
  }
  #[inline]
  pub fn handle(&self) -> vk::SurfaceKHR {
    self.handle
  }
}

impl<'w, D: DeviceContextRef> Drop for Surface<'w, D> {
  fn drop(&mut self) {
    unsafe { self.khr_surface_fns.destroy_surface(self.handle, None) };
  }
}

impl<'w, D: DeviceContextRef> Surface<'w, D> {
  // TODO add cfg for cargo-features enabling platforms
  pub fn new(
    device_context: D,
    window: &'w impl HasWindowHandleExt,
  ) -> Result<Surface<'w, D>> {
    let window_handle = window
      .window_handle()
      .map_err(|_| "window handle not available or unsupported")?;

    let display_handle = window
      .display_handle()
      .map_err(|_| "display handle not available or unsupported")?;

    let platform = display_handle.platform()?;

    let khr_surface_fns = khr::surface::Instance::new(
      device_context.vk_context().entry(),
      device_context.vk_context().instance(),
    );

    let surface_handle;

    match platform {
      // requires KHR_win32_surface extension to be enabled
      #[cfg(target_os = "windows")]
      Platform::Win32 => {
        {
          let create_info =
            match (display_handle.as_raw(), window_handle.as_raw()) {
              (_, RawWindowHandle::Win32(window_handle)) => {
                vk::Win32SurfaceCreateInfoKHR::default()
                  .hwnd(window_handle.hwnd.get() as vk::HWND)
              },
              _ => {
                return Err(
                  "invalid window type {window_handle:?} \
                  for platform: {platform:?}"
                    .into(),
                )
              },
            };
          let khr_win32_surface_fns = khr::win32_surface::Instance::new(
            device_context.vk_context().entry(),
            device_context.vk_context().instance(),
          );

          // TODO: sanity check queue can render to this surface?
          surface_handle = unsafe {
            khr_win32_surface_fns.create_win32_surface(&create_info, None)
          }
          .map_err(|_| "failed to create surface")?;
        }
      },
      // requires KHR_xcb_surface extension to be enabled
      #[cfg(target_os = "linux")]
      Platform::Xcb => {
        use ::raw_window_handle::RawDisplayHandle;
        let create_info = match (
          display_handle.as_raw(),
          window_handle.as_raw(),
        ) {
          (
            RawDisplayHandle::Xcb(display_handle),
            RawWindowHandle::Xcb(window_handle),
          ) => vk::XcbSurfaceCreateInfoKHR::default()
            .connection(display_handle.connection.unwrap().as_ptr())
            .window(window_handle.window.get()),
          _ => return Err(
            "invalid window type {window_handle:?} for platform: {platform:?}"
              .into(),
          ),
        };

        let khr_xcb_surface_fns = khr::xcb_surface::Instance::new(
          device_context.vk_context().entry(),
          device_context.vk_context().instance(),
        );

        surface_handle = unsafe {
          khr_xcb_surface_fns.create_xcb_surface(&create_info, None)
        }
        .map_err(|_| "failed to create surface")?;
      },
      // requires KHR_xlib_surface extension to be enabled
      #[cfg(target_os = "linux")]
      Platform::Xlib => {
        use ::raw_window_handle::RawDisplayHandle;
        let create_info = match (
          display_handle.as_raw(),
          window_handle.as_raw(),
        ) {
          (
            RawDisplayHandle::Xlib(display_handle),
            RawWindowHandle::Xlib(window_handle),
          ) => vk::XlibSurfaceCreateInfoKHR::default()
            .dpy(display_handle.display.unwrap().as_ptr())
            .window(window_handle.window),
          _ => return Err(
            "invalid window type {window_handle:?} for platform: {platform:?}"
              .into(),
          ),
        };

        let khr_xlib_surface_fns = khr::xlib_surface::Instance::new(
          device_context.vk_context().entry(),
          device_context.vk_context().instance(),
        );

        surface_handle = unsafe {
          khr_xlib_surface_fns.create_xlib_surface(&create_info, None)
        }
        .map_err(|_| "failed to create surface")?;
      },
      _ => {
        return Err(
          format!(
            "Platform::{platform:?} not supported or required build feature \
            not enabled"
          )
          .into(),
        )
      },
    }

    // TODO bug(#4) setting debug_object_name with Xlib surface causes a
    // segfault??????
    // device_context.set_debug_object_name(surface_handle, "surface_handle")?;

    Ok(Surface {
      device_context,
      khr_surface_fns,
      window_handle,
      handle: surface_handle,
    })
  }

  #[inline]
  pub fn present_modes(&self) -> Result<Vec<vk::PresentModeKHR>> {
    unsafe {
      self
        .khr_surface_fns
        .get_physical_device_surface_present_modes(
          self.device_context().physical_device,
          self.handle,
        )
        .map_err(|_| "failed to query surface present modes".into())
    }
  }

  #[inline]
  pub fn capabilities(&self) -> Result<vk::SurfaceCapabilitiesKHR> {
    unsafe {
      self
        .khr_surface_fns
        .get_physical_device_surface_capabilities(
          self.device_context().physical_device,
          self.handle,
        )
    }
    .map_err(|_| "failed to query surface capabilities".into())
  }

  #[inline]
  pub fn formats(&self) -> Result<Vec<vk::SurfaceFormatKHR>> {
    unsafe {
      self.khr_surface_fns.get_physical_device_surface_formats(
        self.device_context().physical_device,
        self.handle,
      )
    }
    .map_err(|_| "failed to query surface formats".into())
  }
}
