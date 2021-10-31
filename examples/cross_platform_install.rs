use std::env::current_exe;

use cross_platform_service::service_manager::{install, ServiceInfo};

const SERVICE_NAME: &str = "MyCrPlSVC";

// CAUTION: This example uses "cross_platform_service" example as the executable path.
// Make sure to compile it before running with this example.

fn main() {
    // Get service executable path
    let path = get_exec_path();

    let service_info = ServiceInfo {
        name: SERVICE_NAME.into(),
        description: "My cross platform service".into(),
        exec_path: path,
        auto_start: false,
    };

    match install(service_info) {
        Ok(_) => {
            println!("Service '{}' installed successfully", SERVICE_NAME);
        }
        Err(err) => {
            println!("Could not install service '{}'. {}", SERVICE_NAME, err.to_string());
        }
    }
}

fn get_exec_path() -> String {
    // Use cross_platform_service example as service executable path

    let mut path = current_exe()
        .unwrap();
    path.pop();
    if std::env::consts::FAMILY.eq("windows") {
        path.push("cross_platform_service.exe");
    } else {
        path.push("cross_platform_service");
    }

    path.to_str().unwrap().into()
}