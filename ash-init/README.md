`ash-init`
==========

Vulkan Instance & Device bootstrapping

This crate consists of flexible but opinionated types for setting up a vulkan
renderer.

- `VkContext` bundles up `ash::Entry` & `ash::Instance`, as well as
  `EXT_debug_utils` function pointers and callbacks if enabled. The constructor
  expects `InstanceRequirements`, and after it has been created you may pass
  `DeviceRequirements` to created a logical device.

- `DeviceContext` is a lot more opinionated, it also deals with types for
  `Surface`, `Swapchain`, `Image`, &c.
