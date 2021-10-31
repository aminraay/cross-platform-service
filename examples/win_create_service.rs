use std::env::current_exe;

use cross_platform_service::windows::win_service_manager::{
    WindowsServiceInfo,
    install_windows_service,
};
use cross_platform_service::windows::win32_api_services::{
    ServiceType,
    ServiceStartType,
    ServiceErrorControl,
    SERVICE_ALL_ACCESS,
};

const SERVICE_NAME: &str = "MyWinSVC";
const SERVICE_DISPLAY_NAME: &str = "My Windows Service Sample";
const SERVICE_DESCRIPTION: &str = "My Windows Service Sample Description";

fn main() {
    let mut current_file = current_exe().unwrap();
    current_file.pop();
    current_file.push("win_sample_service.exe");

    let service_exe_path = current_file.to_str().unwrap();

    println!("Try to install service with binary path of '{}'", service_exe_path);

    let info = WindowsServiceInfo {
        name: SERVICE_NAME.into(),
        display_name: SERVICE_DISPLAY_NAME.into(),
        description: Some(SERVICE_DESCRIPTION.into()),
        desired_access: SERVICE_ALL_ACCESS,
        service_type: ServiceType::Win32OwnProcess,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        binary_path: service_exe_path.into(),
        load_order_group: None,
        tag_id: None,
        dependencies: None,
        service_start_name: None,
        password: None,
    };

    match install_windows_service(info) {
        Ok(_) => {
            println!("Service created successfully");
        }
        Err(err) => {
            println!("Could not create service. {}", err.to_string());
        }
    }
}
