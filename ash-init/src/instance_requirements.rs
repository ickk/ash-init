use crate::{macros::make_extensions, Version};

pub struct InstanceRequirements {
  pub version: Version,
  pub extensions: InstanceExtensions,
}

impl Default for InstanceRequirements {
  fn default() -> Self {
    InstanceRequirements {
      version: Version::new(1, 0, 0),
      extensions: InstanceExtensions::default(),
    }
  }
}

make_extensions! {
  pub InstanceExtensions {
    khr_android_surface: cr"VK_KHR_android_surface",
    khr_display: cr"VK_KHR_display",
    khr_get_display_properties2: cr"VK_KHR_get_display_properties2",
    khr_get_surface_capabilities2: cr"VK_KHR_get_surface_capabilities2",
    khr_portability_enumeration: cr"VK_KHR_portability_enumeration",
    khr_surface: cr"VK_KHR_surface",
    khr_surface_protected_capabilities: cr"VK_KHR_surface_protected_capabilities",
    khr_wayland_surface: cr"VK_KHR_wayland_surface",
    khr_win32_surface: cr"VK_KHR_win32_surface",
    khr_xcb_surface: cr"VK_KHR_xcb_surface",
    khr_xlib_surface: cr"VK_KHR_xlib_surface",
    ext_acquire_drm_display: cr"VK_EXT_acquire_drm_display",
    ext_acquire_xlib_display: cr"VK_EXT_acquire_xlib_display",
    ext_debug_utils: cr"VK_EXT_debug_utils",
    ext_direct_mode_display: cr"VK_EXT_direct_mode_display",
    ext_directfb_surface: cr"VK_EXT_directfb_surface",
    ext_display_surface_counter: cr"VK_EXT_display_surface_counter",
    ext_headless_surface: cr"VK_EXT_headless_surface",
    ext_layer_settings: cr"VK_EXT_layer_settings",
    ext_metal_surface: cr"VK_EXT_metal_surface",
    ext_surface_maintenance1: cr"VK_EXT_surface_maintenance1",
    ext_swapchain_colorspace: cr"VK_EXT_swapchain_colorspace",
    fuchsia_imagepipe_surface: cr"VK_FUCHSIA_imagepipe_surface",
    ggp_stream_descriptor_surface: cr"VK_GGP_stream_descriptor_surface",
    google_surfaceless_query: cr"VK_GOOGLE_surfaceless_query",
    lunarg_direct_driver_loading: cr"VK_LUNARG_direct_driver_loading",
    nn_vi_surface: cr"VK_NN_vi_surface",
    qnx_screen_surface: cr"VK_QNX_screen_surface",
  }
}
