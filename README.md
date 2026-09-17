`ash-init`
==========

An opinionated package to help with Vulkan Device initialisation.

- `VkContext` bundles up `ash::Entry` & `ash::Instance`, as well as
  `EXT_debug_utils` function pointers and callbacks if enabled. The constructor
  expects `InstanceRequirements`, and after it has been created you may pass
  `DeviceRequirements` to created a logical device.

- `DeviceContext` is a lot more opinionated, it also deals with types for
  `Surface`, `Swapchain`, `Image`, &c.

Look at the [device_creation](./ash-init/examples/device_creation.rs) example
to see how device initialisation is transformed into a much simpler declarative
API.

-------------------------------------------------------------------------------
<footer><small>© ickk 2023-2026, All Rights Reserved.</small></footer>
