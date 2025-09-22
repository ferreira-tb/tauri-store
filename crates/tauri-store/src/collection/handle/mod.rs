#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod desktop;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod mobile;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub use desktop::Handle;

#[cfg(any(target_os = "android", target_os = "ios"))]
pub use mobile::Handle;
