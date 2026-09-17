//! Sample [`DebugCallback`][crate::debug::DebugCallback] implementations

pub(crate) mod blackhole_debug_callback;
pub(crate) mod print_debug_callback;

pub use self::{
  blackhole_debug_callback::BlackholeDebugCallback,
  print_debug_callback::PrintDebugCallback,
};

#[cfg(feature = "tracing")]
pub(crate) mod tracing_debug_callback;
#[cfg(feature = "tracing")]
pub use self::tracing_debug_callback::TracingDebugCallback;

#[cfg(feature = "log")]
pub(crate) mod log_debug_callback;
#[cfg(feature = "log")]
pub use self::log_debug_callback::LogDebugCallback;
