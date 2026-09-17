// © ickk 2023-2026, All Rights Reserved.

//! Sample [`DebugCallback`][crate::debug::DebugCallback] implementations

pub(crate) mod blackhole_debug_callback;

pub use self::blackhole_debug_callback::BlackholeDebugCallback;

#[cfg(feature = "tracing")]
pub(crate) mod tracing_debug_callback;
#[cfg(feature = "tracing")]
pub use self::tracing_debug_callback::TracingDebugCallback;
