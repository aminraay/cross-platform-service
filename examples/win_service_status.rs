use cross_platform_service::windows::win_service_manager::get_service_status;

const SERVICE_NAME: &str = "MyWinSVC";

fn main() {
    let service_status = get_service_status(SERVICE_NAME.into()).unwrap();

    println!("Service Name: {}", SERVICE_NAME);
    println!("Current State: {:?}", service_status.service_state);
    if service_status.controls_accepted.len() > 0 {
        println!("Controls Accepted: ");
        for it in service_status.controls_accepted {
            println!("\t{:?}", it);
        }
    } else {
        println!("Controls Accepted: [None]");
    }
}