use cross_platform_service::unix::services::{get_unit_status};
use std::time::Duration;

const SERVICE_NAME: &str = "MyUnixSVC.service";

fn main() {
    let status = get_unit_status(SERVICE_NAME,
                                 Duration::from_secs(10))
        .unwrap();

    match status {
        None => {
            println!("Unit '{}' was not found", SERVICE_NAME);
        }
        Some(value) => {
            println!("Unit Name: {}", value.name);
            println!("Description: {}", value.description);
            println!("Is Active: {}", value.is_active);
            // To get list of available sub states use the following command:
            // systemctl --state=help
            println!("State: {}", value.state);
            println!("Exec Path: {}", value.exec_path);
        }
    }
}