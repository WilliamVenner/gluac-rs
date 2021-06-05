#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("Unsupported operating system");

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unsupported operating system pointer width");

#[cfg(feature = "parking_lot")]
pub(crate) type Mutex<T> = parking_lot::Mutex<T>;
#[cfg(not(feature = "parking_lot"))]
pub(crate) type Mutex<T> = std::sync::Mutex<T>;

#[cfg(feature = "parking_lot")]
pub(crate) type MutexGuard<'a, T> = parking_lot::MutexGuard<'a, T>;
#[cfg(not(feature = "parking_lot"))]
pub(crate) type MutexGuard<'a, T> = std::sync::MutexGuard<'a, T>;

pub mod lua;

#[macro_use]
mod api;
pub use api::*;

mod compiler;

#[cfg(test)]
mod tests;
