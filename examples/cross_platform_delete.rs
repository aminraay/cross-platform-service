use cross_platform_service::service_manager::delete;

const SERVICE_NAME: &str = "MyCrPlSVC";

fn main() {
    // Delete service [SERVICE_NAME] in Windows and "[SERVICE_NAME].service" systemd unit file
    // in Linux

    match delete(SERVICE_NAME) {
        Ok(_) => {
            println!("Service '{}' successfully deleted", SERVICE_NAME)
        }
        Err(err) => {
            println!("Could not delete service '{}', {}", SERVICE_NAME, err.to_string())
        }
    }

}