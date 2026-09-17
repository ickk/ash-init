use crate::macros::make_extensions;

make_extensions! {
  pub InstanceExtensions {
    khr_android_surface: b"VK_KHR_android_surface\0",
    khr_display: b"VK_KHR_display\0",
    khr_get_display_properties2: b"VK_KHR_get_display_properties2\0",
    khr_get_surface_capabilities2: b"VK_KHR_get_surface_capabilities2\0",
    khr_portability_enumeration: b"VK_KHR_portability_enumeration\0",
    khr_surface: b"VK_KHR_surface\0",
    khr_surface_protected_capabilities: b"VK_KHR_surface_protected_capabilities\0",
    khr_wayland_surface: b"VK_KHR_wayland_surface\0",
    khr_win32_surface: b"VK_KHR_win32_surface\0",
    khr_xcb_surface: b"VK_KHR_xcb_surface\0",
    khr_xlib_surface: b"VK_KHR_xlib_surface\0",
    ext_acquire_drm_display: b"VK_EXT_acquire_drm_display\0",
    ext_acquire_xlib_display: b"VK_EXT_acquire_xlib_display\0",
    ext_debug_utils: b"VK_EXT_debug_utils\0",
    ext_direct_mode_display: b"VK_EXT_direct_mode_display\0",
    ext_directfb_surface: b"VK_EXT_directfb_surface\0",
    ext_display_surface_counter: b"VK_EXT_display_surface_counter\0",
    ext_headless_surface: b"VK_EXT_headless_surface\0",
    ext_layer_settings: b"VK_EXT_layer_settings\0",
    ext_metal_surface: b"VK_EXT_metal_surface\0",
    ext_surface_maintenance1: b"VK_EXT_surface_maintenance1\0",
    ext_swapchain_colorspace: b"VK_EXT_swapchain_colorspace\0",
    fuchsia_imagepipe_surface: b"VK_FUCHSIA_imagepipe_surface\0",
    ggp_stream_descriptor_surface: b"VK_GGP_stream_descriptor_surface\0",
    google_surfaceless_query: b"VK_GOOGLE_surfaceless_query\0",
    lunarg_direct_driver_loading: b"VK_LUNARG_direct_driver_loading\0",
    nn_vi_surface: b"VK_NN_vi_surface\0",
    qnx_screen_surface: b"VK_QNX_screen_surface\0",
  }
}
