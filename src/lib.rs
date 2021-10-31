#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod unix;

pub mod service;
pub mod service_manager;

#[cfg(windows)]
mod windows_api_bindings {
    windows::include_bindings!();
}
