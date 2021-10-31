// This Windows service executable example. This example calls most Windows APIs functions directly.
// This is right example if:
//      - Need to enable/disable service actions (Start/Stop/Continue/Pause)
//      - Need to notify progress of pending controls to operating system
//      - Service needs to support pause/continue on windows
//      - Need to define windows service type anything except than Win32OwnProcess

// For more technical information about Microsoft Windows services visit:
// https://docs.microsoft.com/en-us/windows/win32/services/the-complete-service-sample



use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use cross_platform_service::windows::win32_api_services::{
    register_service_ctrl_handler,
    ServiceControl,
    ServiceControlsAccepted,
    ServiceState,
    ServiceStatus,
    ServiceType,
    set_service_status,
    start_service_ctrl_dispatcher,
};

pub const SERVICE_NAME: &str = "MyWinSVC";
static mut RUNNING: Option<Arc<AtomicBool>> = None;

fn main() {
    // Define some sort of mechanism to notify main service function to stop
    unsafe {
        RUNNING = Some(Arc::new(AtomicBool::new(true)));
    }

    // By calling start_service_ctrl_dispatcher method current thread will be put in sleeping
    // by Operating system (Windows) and 'svc_main' will be called as service main function
    match start_service_ctrl_dispatcher(SERVICE_NAME, Some(svc_main)) {
        Ok(_) => {
            // Service run successfully
        }
        Err(err) => {
            println!("Could not start service. {}", err.to_string())
        }
    };
}

fn svc_main(_: Vec<String>) {
       // Windows service must register control function at the beginning
    let handle = register_service_ctrl_handler(
        SERVICE_NAME, Some(service_ctrl)).unwrap();

    // Optional Step: Set service status to StartPending
    // If initializing service would take long time you may set StartPending multiple times each
    // time with check_point increased by one.
    // wait_time is approximate time in seconds service requires to finish StartPending status
    set_service_status(&handle, ServiceStatus {
        service_type: ServiceType::Win32OwnProcess,
        service_state: ServiceState::StartPending,
        controls_accepted: vec![],
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }).expect("Cannot set service status");

    // Write service initializing code here

    // Set the service status to running.
    // Indicate service will accept Stop control from now on
    set_service_status(&handle, ServiceStatus {
        service_type: ServiceType::Win32OwnProcess,
        service_state: ServiceState::Running,
        controls_accepted: vec![ServiceControlsAccepted::Stop],
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }).expect("Cannot set service status");

    let running = unsafe {
        RUNNING.as_ref().unwrap().clone()
    };

    // Write service operation code here
    // set_service_status may be used to change service status during operation and
    // received controls in svc_ctrl function
    for _ in 0..600 {
        if !running.load(Ordering::Relaxed) {
            // Running value will be set to false when SIGTERM signal received on Linux or stop
            // control sent by Windows service manager

            break;
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    // Optional step: Set service status to StopPending
    // If stopping service would take long time you may set StopPending multiple times each
    // time with check_point increased by one.
    set_service_status(&handle, ServiceStatus {
        service_type: ServiceType::Win32OwnProcess,
        service_state: ServiceState::StopPending,
        controls_accepted: vec![],
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }).expect("Cannot set service status");

    // Write closing resources code here

    // Notify operating system the service is stopped
    set_service_status(&handle, ServiceStatus {
        service_type: ServiceType::Win32OwnProcess,
        service_state: ServiceState::Stopped,
        controls_accepted: vec![],
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }).expect("Cannot set service status");
}

fn service_ctrl(control: ServiceControl) {
    // Handle service controls sent by operating system
    // Note that the list of accepted controls must be set by set_service_status in svc_main
    // function
    match control {
        ServiceControl::Continue => {}
        ServiceControl::Interrogate => {}
        ServiceControl::NetBindAdd => {}
        ServiceControl::NetBindDisable => {}
        ServiceControl::NetBindEnable => {}
        ServiceControl::NetBindRemove => {}
        ServiceControl::ParamChange => {}
        ServiceControl::Pause => {}
        ServiceControl::Stop => {
            // Set running value to false to notify main function to stop
            unsafe {
                RUNNING.as_ref().unwrap().store(false, Ordering::Relaxed);
            }
        }
    }
}
