use cross_platform_service::windows::win32_api_services::{ServiceControl, ServiceControlsAccepted, ServiceState};
use cross_platform_service::windows::win_service_manager::{get_service_status, control_windows_service};

const SERVICE_NAME: &str = "MyWinSVC";

fn main() {
    let status = get_service_status(SERVICE_NAME)
        .unwrap();

    // Check if service already stopped
    if status.service_state == ServiceState::Stopped {
        println!("Service '{}' is already stopped", SERVICE_NAME);
    } else if !status.controls_accepted.contains(&ServiceControlsAccepted::Stop) {
        // If service was NOT capable of stopping
        println!("Service '{}' does not accept stop at the moment.", SERVICE_NAME)
    } else {
        // Send Stop to service
        match control_windows_service(SERVICE_NAME, ServiceControl::Stop) {
            Ok(_) => {
                println!("Service '{}' successfully stopped", SERVICE_NAME);
            }
            Err(err) => {
                println!("Could not stop service. {}", err.to_string());
            }
        };
    }
}