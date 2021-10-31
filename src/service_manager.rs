use std::io::Error;

#[cfg(unix)]
use std::time::Duration;

#[cfg(unix)]
use crate::unix::services::{
    delete_unit,
    disable_unit_files,
    enable_unit_files,
    install_string,
    Mode,
    start_unit,
    stop_unit,
};

#[cfg(windows)]
use crate::windows::{
    win_service_manager::{
        control_windows_service,
        delete_windows_service,
        install_windows_service,
        start_windows_service,
        WindowsServiceInfo,
    },
    win32_api_services::{
        SERVICE_ALL_ACCESS,
        ServiceControl,
        ServiceErrorControl,
        ServiceStartType,
        ServiceType},
};

#[cfg(unix)]
fn get_unit_name(service_name: &str) -> String {
    format!("{}.service", service_name)
}

/// Starts specified service
///
/// Uses D-Bus StartUnit function from path '/org/freedesktop/systemd1' to start the specified service.
///
/// Appends ".service" at the end of service name.
#[cfg(unix)]
pub fn start(service_name: &str) -> Result<(), Error> {
    start_unit(get_unit_name(service_name).as_str(),
               Mode::Replace,
               Duration::from_secs(30))
}

/// Stops service in operating system
///
/// Uses D-Bus StopUnit function from path '/org/freedesktop/systemd1' to stop specified service.
///
/// Appends ".service" at the end of service name.
#[cfg(unix)]
pub fn stop(service_name: &str) -> Result<(), Error> {
    stop_unit(get_unit_name(service_name).as_str(),
              Mode::Replace,
              Duration::from_secs(30))
}

/// Deletes specified service
///
/// Deletes service unit file from path "/etc/systemd/system".
///
/// Appends ".service" at the end of service name.
#[cfg(unix)]
pub fn delete(service_name: &str) -> Result<(), Error> {
    delete_unit(get_unit_name(service_name).as_str())
}

/// Contains required information for service installation
pub struct ServiceInfo {
    /// Service name. It's better to choose a service name without *space* character.
    pub name: String,
    /// Service description
    pub description: String,
    /// Executable file path of the service
    pub exec_path: String,
    /// Indicate if service should start at booting time
    pub auto_start: bool,
}

/// Installs service by specified service information
///
/// Adds new service file to path "/etc/systemd/system".
///
/// Appends ".service" at the end of service name.
#[cfg(unix)]
pub fn install(service_info: ServiceInfo) -> Result<(), Error> {
    let service_file = format!("[Unit]
Description={}

[Service]
Type=simple
ExecStart={}", service_info.description, service_info.exec_path);

    let name = format!("{}.service", service_info.name.as_str());
    install_string(name.as_str(), service_file.as_str())?;

    if service_info.auto_start {
        enable_unit_files(vec![name.as_str()], Duration::from_secs(30))
    } else {
        disable_unit_files(vec![name.as_str()], Duration::from_secs(30))
    }
}

/// Starts service in operating system by specified name
///
/// Uses [StartServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicew)
/// Windows API function to start service
///
#[cfg(windows)]
pub fn start(service_name: &str) -> Result<(), Error> {
    start_windows_service(service_name)
}

/// Stops service in operating system
///
/// Uses [ControlService](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-controlservice)
/// Windows API to send *SERVICE_CONTROL_STOP* to specified service.
///
#[cfg(windows)]
pub fn stop(service_name: &str) -> Result<(), Error> {
    control_windows_service(service_name, ServiceControl::Stop)
}

/// Deletes specified service
///
/// Uses [DeleteService](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-deleteservice)
/// Windows API to delete specified service name.
///
#[cfg(windows)]
pub fn delete(service_name: &str) -> Result<(), Error> {
    delete_windows_service(service_name)
}

/// Installs service by specified service information
///
/// Uses [CreateServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
/// Windows API to create service.
///
#[cfg(windows)]
pub fn install(service_info: ServiceInfo) -> Result<(), Error> {
    let start = if service_info.auto_start {
        ServiceStartType::AutoStart
    } else { ServiceStartType::DemandStart };

    let w_service_info = WindowsServiceInfo {
        name: service_info.name.clone(),
        display_name: service_info.name,
        description: Some(service_info.description),
        desired_access: SERVICE_ALL_ACCESS,
        service_type: ServiceType::Win32OwnProcess,
        start_type: start,
        error_control: ServiceErrorControl::Normal,
        binary_path: service_info.exec_path,
        load_order_group: None,
        tag_id: None,
        dependencies: None,
        service_start_name: None,
        password: None,
    };

    install_windows_service(w_service_info)
}
