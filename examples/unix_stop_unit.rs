use std::time::Duration;

use cross_platform_service::unix::services::{Mode, stop_unit};

const SERVICE_NAME: &str = "MyUnixSVC.service";

fn main() {
    match stop_unit(SERVICE_NAME,
                     Mode::Replace,
                     Duration::from_secs(2)) {
        Ok(_) => {
            println!("Service '{}' stopped successfully", SERVICE_NAME);
        }
        Err(err) => {
            println!("Could not stop service '{}', {}", SERVICE_NAME, err.to_string());
        }
    }
}