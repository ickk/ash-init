use {
  super::Object,
  ::ash::vk,
  ::core::{
    ffi::{c_void, CStr},
    fmt,
    ptr::NonNull,
    slice,
  },
};

/// Implementors of [`DebugCallback`] can be registered as callbacks using
/// VK_EXT_debug_utils.
///
/// This trait abstracts the ffi and raw pointer stuff that is required when
/// dealing with the callback directly.
///
/// May be called from multiple threads simultaneously[1]
///
/// [1]: https://docs.vulkan.org/spec/latest/chapters/debugging.html#debugging-debug-messengers
pub trait DebugCallback: Send + Sync {
  /// This value is used once when the messenger is created to set which
  /// messages it will receive from Vulkan.
  fn severities(&self) -> vk::DebugUtilsMessageSeverityFlagsEXT;
  /// This value is used once when the messenger is created to set which
  /// messages it will receive from Vulkan.
  fn types(&self) -> vk::DebugUtilsMessageTypeFlagsEXT;
  /// This callback will be called for each message matching the `severities` &
  /// `types` flags as above.
  fn callback(
    &self,
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: CallbackData,
  );
}

/// Wrapper for [`vk::DebugUtilsMessengerCallbackDataEXT`] that provides
/// methods for accessing the fields with natural rust types (instead of ffi
/// types).
///
/// It also captures the lifetime of the callback scope (which is a requirement
/// of the vulkan spec for `VK_EXT_debug_utils`) using a reference.
#[repr(transparent)]
pub struct CallbackData<'cb>(&'cb vk::DebugUtilsMessengerCallbackDataEXT<'cb>);
/// Wrapper for [`vk::DebugUtilsLabelEXT`] that provides methods for accessing
/// the fields with native rust types (instead of ffi types).
#[repr(transparent)]
pub struct DebugLabel<'cb>(vk::DebugUtilsLabelEXT<'cb>);
/// Wrapper for [`vk::DebugUtilsObjectNameInfoEXT`] that provide methods for
/// accessing the fields with native rust types (instead of ffi types).
#[repr(transparent)]
pub struct DebugObjectNameInfo<'cb>(vk::DebugUtilsObjectNameInfoEXT<'cb>);

impl<'cb> CallbackData<'cb> {
  #[inline]
  pub fn message_id_name(&self) -> &str {
    unsafe { CStr::from_ptr(self.0.p_message_id_name) }
      .to_str()
      .unwrap()
  }
  #[inline]
  pub fn message_id_number(&self) -> u32 {
    self.0.message_id_number as u32
  }
  #[inline]
  pub fn message(&self) -> &str {
    unsafe { CStr::from_ptr(self.0.p_message) }
      .to_str()
      .unwrap()
  }
  #[inline]
  pub fn queue_labels(&self) -> &[DebugLabel<'_>] {
    if !self.0.p_queue_labels.is_null() {
      // safety:
      // - `DebugLabel` is a transparent wrapper of `vk::DebugUtilsLabelExt`
      // - `p_queue_labels` is non-null
      unsafe {
        ::core::mem::transmute::<&[vk::DebugUtilsLabelEXT], &[DebugLabel]>(
          slice::from_raw_parts(
            self.0.p_queue_labels,
            self.0.queue_label_count as usize,
          ),
        )
      }
    } else {
      &[]
    }
  }
  #[inline]
  pub fn command_buffer_labels(&self) -> &[DebugLabel<'_>] {
    if !self.0.p_cmd_buf_labels.is_null() {
      // safety:
      // - `DebugLabel` is a transparent wrapper of `vk::DebugUtilsLabelExt`
      // - `p_cmd_buf_labels` is non-null
      unsafe {
        ::core::mem::transmute::<&[vk::DebugUtilsLabelEXT], &[DebugLabel]>(
          slice::from_raw_parts(
            self.0.p_cmd_buf_labels,
            self.0.cmd_buf_label_count as usize,
          ),
        )
      }
    } else {
      &[]
    }
  }
  #[inline]
  pub fn objects(&self) -> &[DebugObjectNameInfo<'_>] {
    if !self.0.p_objects.is_null() {
      // safety:
      // - `DebugObjectNameInfo` is a transparent wrapper of
      //   `vk::DebugUtilsObjectNameInfoEXT`
      // - `p_objects`` is non-null
      unsafe {
        ::core::mem::transmute::<
          &[vk::DebugUtilsObjectNameInfoEXT],
          &[DebugObjectNameInfo],
        >(slice::from_raw_parts(
          self.0.p_objects,
          self.0.object_count as usize,
        ))
      }
    } else {
      &[]
    }
  }
  #[inline]
  pub fn as_raw(&self) -> &vk::DebugUtilsMessengerCallbackDataEXT<'_> {
    self.0
  }
}

impl<'cb> DebugLabel<'cb> {
  #[inline]
  pub fn label_name(&self) -> &str {
    unsafe { CStr::from_ptr(self.0.p_label_name) }
      .to_str()
      .unwrap()
  }
  #[inline]
  pub fn color(&self) -> &[f32; 4] {
    &self.0.color
  }
  #[inline]
  pub fn as_raw(&self) -> &vk::DebugUtilsLabelEXT<'_> {
    &self.0
  }
}

impl fmt::Display for DebugLabel<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if f.alternate() {
      let [r, g, b, _a] = self.color().map(|f| (f * 255.0) as u8);
      f.write_fmt(format_args!(
        "\x1b[38;2;{r};{g};{b}m{label}\x1b[0m",
        label = self.label_name()
      ))
    } else {
      f.write_str(self.label_name())
    }
  }
}

impl<'cb> DebugObjectNameInfo<'cb> {
  #[inline]
  pub fn object_name(&self) -> Option<&str> {
    let p = NonNull::new(self.0.p_object_name as *mut i8);
    p.and_then(|p| unsafe { CStr::from_ptr(p.as_ptr()) }.to_str().ok())
  }
  #[inline]
  pub fn object(&self) -> Object {
    Object::from_object_type_and_handle(
      self.0.object_type,
      self.0.object_handle,
    )
  }
  #[inline]
  pub fn as_raw(&self) -> &vk::DebugUtilsObjectNameInfoEXT<'_> {
    &self.0
  }
}

impl fmt::Display for DebugObjectNameInfo<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(name) = self.object_name() {
      f.write_fmt(format_args!("{:?}: {name}", self.object()))
    } else {
      f.write_fmt(format_args!("{:?}", self.object()))
    }
  }
}

/// This callback matches the signature required by
/// `vk::DebugUtilsMessengerCreateInfoExt.pfn_user_callback`.
///
/// The corresponding `p_user_data` field should be a
/// `Box<Box<dyn DebugCallback>` (the second Box allows the wide pointer of
/// the `Box<dyn T>` to fit into a usize as required by the signature).
// TODO: use [`::std::boxed::ThinBox`] when stabilised.
///
/// # Safety
///
/// This callback MUST only be supplied to
/// [`vk::DebugUtilsMessengerCreateInfoExt`]'s `pfn_user_callback` field when
/// the corresponding `p_user_data` field is a raw pointer pointing to a
/// `Box<dyn DebugCallback>`.
///
/// Be sure to note the double indirection. i.e.
/// `p_user_data` should be a `(*mut Box<dyn DebugCallback>).cast::<c_void>()`
pub(crate) unsafe extern "system" fn debug_callback_ffi(
  message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  message_types: vk::DebugUtilsMessageTypeFlagsEXT,
  callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  user_data: *mut c_void,
) -> vk::Bool32 {
  let callback_data = unsafe { &*callback_data };
  let user_data = user_data.cast::<Box<dyn DebugCallback>>();
  let user_data = unsafe { Box::from_raw(user_data) };

  // data with pointers captures a lifetime scoped to this callback through
  // CallbackData<'cb>
  user_data.callback(
    message_severity,
    message_types,
    CallbackData(callback_data),
  );

  _ = Box::into_raw(user_data);

  vk::FALSE
}
