// © ickk 2023-2026, All Rights Reserved.

//! Sample [`DebugCallback`][crate::debug::DebugCallback] implementations

pub(crate) mod blackhole;
pub use self::blackhole::BlackholeDebugCallback;

#[cfg(feature = "tracing")]
pub(crate) mod tracing;
#[cfg(feature = "tracing")]
pub use self::tracing::TracingDebugCallback;
