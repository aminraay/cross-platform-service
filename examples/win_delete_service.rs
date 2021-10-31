use cross_platform_service::service_manager::delete;


const SERVICE_NAME: &str = "MyWinSVC";

fn main() {
    match delete(SERVICE_NAME) {
        Ok(_) => {
            println!("Service '{}' was successfully deleted", SERVICE_NAME);
        }
        Err(err) => {
            println!("Could not delete service '{}'. {}", SERVICE_NAME, err.to_string());
        }
    }
}