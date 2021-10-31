use cross_platform_service::windows::win32_api_services::ServiceState;
use cross_platform_service::windows::win_service_manager::{get_service_status, start_windows_service};

const SERVICE_NAME: &str = "MyWinSVC";

fn main() {
    let status = get_service_status(SERVICE_NAME)
        .unwrap();

    if status.service_state != ServiceState::Stopped {
        println!("Service '{}' is not stopped state", SERVICE_NAME);
    } else {
        match start_windows_service(SERVICE_NAME) {
            Ok(_) => {
                println!("Service '{}' was successfully started", SERVICE_NAME);
            }
            Err(err) => {
                println!("Could not start service '{}', {}", SERVICE_NAME, err.to_string());
            }
        }
    }
}