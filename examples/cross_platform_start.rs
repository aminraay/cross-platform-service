use cross_platform_service::service_manager::start;

const SERVICE_NAME: &str = "MyCrPlSVC";

fn main() {
    match start(SERVICE_NAME) {
        Ok(_) => {
            println!("Service '{}' successfully started", SERVICE_NAME)
        }
        Err(err) => {
            println!("Could not start service '{}', {}", SERVICE_NAME, err.to_string())
        }
    }
}