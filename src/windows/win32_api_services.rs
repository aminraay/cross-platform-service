use core::ffi::c_void;
use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

use crate::windows_api_bindings::{
    Windows::Win32::Foundation::PWSTR,
    Windows::Win32::Security::SC_HANDLE,
    Windows::Win32::System::Services::*,
};

/// Provides value of [SC_MANAGER_ALL_ACCESS](https://docs.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights)
pub const SC_MANAGER_ALL_ACCESS: u32 = 0xF003F;

/// Provides value of [SERVICE_ALL_ACCESS](https://docs.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights)
pub const SERVICE_ALL_ACCESS: u32 = 0xF01FF;

/// Contains Windows API service handle value
pub struct ServiceHandle(pub SC_HANDLE);

/// Contains Windows API service status handle value
pub struct ServiceStatusHandle(pub SERVICE_STATUS_HANDLE);

//region --- ServiceType ---

/// Provides dwServiceType values
///
/// For more information visit [CreateServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
#[derive(Debug, PartialEq)]
pub enum ServiceType {
    /// Represents SERVICE_FILE_SYSTEM_DRIVER value
    FileSystemDriver = SERVICE_FILE_SYSTEM_DRIVER.0 as isize,
    /// Represents SERVICE_KERNEL_DRIVER value
    KernelDriver = SERVICE_KERNEL_DRIVER.0 as isize,
    /// Represents SERVICE_WIN32_OWN_PROCESS value
    Win32OwnProcess = SERVICE_WIN32_OWN_PROCESS.0 as isize,
    /// Represents SERVICE_WIN32_SHARE_PROCESS value
    Win32ShareProcess = SERVICE_WIN32_SHARE_PROCESS.0 as isize,
    /// Represents SERVICE_USER_OWN_PROCESS value
    UserOwnProcess = SERVICE_USER_OWN_PROCESS.0 as isize,
    /// Represents SERVICE_USER_SHARE_PROCESS value
    UserShareProcess = SERVICE_USER_SHARE_PROCESS.0 as isize,
}

impl Into<ENUM_SERVICE_TYPE> for ServiceType {
    fn into(self) -> ENUM_SERVICE_TYPE {
        ENUM_SERVICE_TYPE(self as u32)
    }
}

impl TryFrom<u32> for ServiceType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if ServiceType::FileSystemDriver as u32 == value {
            Ok(ServiceType::FileSystemDriver)
        } else if ServiceType::KernelDriver as u32 == value {
            Ok(ServiceType::KernelDriver)
        } else if ServiceType::Win32OwnProcess as u32 == value {
            Ok(ServiceType::Win32OwnProcess)
        } else if ServiceType::Win32ShareProcess as u32 == value {
            Ok(ServiceType::Win32ShareProcess)
        } else if ServiceType::UserOwnProcess as u32 == value {
            Ok(ServiceType::UserOwnProcess)
        } else if ServiceType::UserShareProcess as u32 == value {
            Ok(ServiceType::UserShareProcess)
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid service type value"))
        }
    }
}

//endregion

//region --- ServiceStartType ---

/// Provides dwStartType values
///
/// For more information visit [CreateServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
#[derive(Debug, PartialEq)]
pub enum ServiceStartType {
    /// Represents SERVICE_AUTO_START value
    AutoStart = SERVICE_AUTO_START.0 as isize,
    /// Represents SERVICE_BOOT_START value
    BootStart = SERVICE_BOOT_START.0 as isize,
    /// Represents SERVICE_DEMAND_START value
    DemandStart = SERVICE_DEMAND_START.0 as isize,
    /// Represents SERVICE_DISABLED value
    Disabled = SERVICE_DISABLED.0 as isize,
    /// Represents SERVICE_SYSTEM_START value
    SystemStart = SERVICE_SYSTEM_START.0 as isize,
}

impl Into<SERVICE_START_TYPE> for ServiceStartType {
    fn into(self) -> SERVICE_START_TYPE {
        SERVICE_START_TYPE(self as u32)
    }
}

//endregion

//region --- ServiceErrorControl ---

/// Provides dwErrorControl values
///
/// For more information visit [CreateServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
#[derive(Debug, PartialEq)]
pub enum ServiceErrorControl {
    /// Represents SERVICE_ERROR_CRITICAL value
    Critical = SERVICE_ERROR_CRITICAL.0 as isize,
    /// Represents SERVICE_ERROR_IGNORE value
    Ignore = SERVICE_ERROR_IGNORE.0 as isize,
    /// Represents SERVICE_ERROR_NORMAL value
    Normal = SERVICE_ERROR_NORMAL.0 as isize,
    /// Represents SERVICE_ERROR_SEVERE value
    Severe = SERVICE_ERROR_SEVERE.0 as isize,
}

impl Into<SERVICE_ERROR> for ServiceErrorControl {
    fn into(self) -> SERVICE_ERROR {
        SERVICE_ERROR(self as u32)
    }
}

//endregion

//region --- ServiceControlAccepted ---

/// [SERVICE_STATUS](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status)
#[derive(Debug, PartialEq)]
pub enum ServiceControlsAccepted {
    /// Provides SERVICE_ACCEPT_NETBINDCHANGE value
    NetBindChange = SERVICE_ACCEPT_NETBINDCHANGE as isize,
    /// Provides SERVICE_ACCEPT_PARAMCHANGE value
    ParamChange = SERVICE_ACCEPT_PARAMCHANGE as isize,
    /// Provides SERVICE_ACCEPT_PAUSE_CONTINUE value
    PauseContinue = SERVICE_ACCEPT_PAUSE_CONTINUE as isize,
    /// Provides SERVICE_ACCEPT_PRESHUTDOWN value
    PreShutdown = SERVICE_ACCEPT_PRESHUTDOWN as isize,
    /// Provides SERVICE_ACCEPT_SHUTDOWN value
    Shutdown = SERVICE_ACCEPT_SHUTDOWN as isize,
    /// Provides SERVICE_ACCEPT_STOP value
    Stop = SERVICE_ACCEPT_STOP as isize,
    /// Provides SERVICE_ACCEPT_HARDWAREPROFILECHANGE value
    HardwareProfileChange = SERVICE_ACCEPT_HARDWAREPROFILECHANGE as isize,
    /// Provides SERVICE_ACCEPT_POWEREVENT value
    PowerEvent = SERVICE_ACCEPT_POWEREVENT as isize,
    /// Provides SERVICE_ACCEPT_SESSIONCHANGE value
    SessionChange = SERVICE_ACCEPT_SESSIONCHANGE as isize,
    /// Provides SERVICE_ACCEPT_TIMECHANGE value
    TimeChange = SERVICE_ACCEPT_TIMECHANGE as isize,
    /// Provides SERVICE_ACCEPT_TRIGGEREVENT value
    TriggerEvent = SERVICE_ACCEPT_TRIGGEREVENT as isize,
    /// Provides SERVICE_ACCEPT_USERMODEREBOOT value
    UserModeReboot = 0x00000800,
}


fn u32_to_controls_accepted(value: u32) -> Vec<ServiceControlsAccepted> {
    let mut result = vec![];
    if (ServiceControlsAccepted::NetBindChange as u32 & value) == ServiceControlsAccepted::NetBindChange as u32 { result.push(ServiceControlsAccepted::NetBindChange); }
    if (ServiceControlsAccepted::ParamChange as u32 & value) == ServiceControlsAccepted::ParamChange as u32 { result.push(ServiceControlsAccepted::ParamChange); }
    if (ServiceControlsAccepted::PauseContinue as u32 & value) == ServiceControlsAccepted::PauseContinue as u32 { result.push(ServiceControlsAccepted::PauseContinue); }
    if (ServiceControlsAccepted::PreShutdown as u32 & value) == ServiceControlsAccepted::PreShutdown as u32 { result.push(ServiceControlsAccepted::PreShutdown); }
    if (ServiceControlsAccepted::Shutdown as u32 & value) == ServiceControlsAccepted::Shutdown as u32 { result.push(ServiceControlsAccepted::Shutdown); }
    if (ServiceControlsAccepted::Stop as u32 & value) == ServiceControlsAccepted::Stop as u32 { result.push(ServiceControlsAccepted::Stop); }
    if (ServiceControlsAccepted::HardwareProfileChange as u32 & value) == ServiceControlsAccepted::HardwareProfileChange as u32 { result.push(ServiceControlsAccepted::HardwareProfileChange); }
    if (ServiceControlsAccepted::PowerEvent as u32 & value) == ServiceControlsAccepted::PowerEvent as u32 { result.push(ServiceControlsAccepted::PowerEvent); }
    if (ServiceControlsAccepted::SessionChange as u32 & value) == ServiceControlsAccepted::SessionChange as u32 { result.push(ServiceControlsAccepted::SessionChange); }
    if (ServiceControlsAccepted::TimeChange as u32 & value) == ServiceControlsAccepted::TimeChange as u32 { result.push(ServiceControlsAccepted::TimeChange); }
    if (ServiceControlsAccepted::TriggerEvent as u32 & value) == ServiceControlsAccepted::TriggerEvent as u32 { result.push(ServiceControlsAccepted::TriggerEvent); }
    if (ServiceControlsAccepted::UserModeReboot as u32 & value) == ServiceControlsAccepted::UserModeReboot as u32 { result.push(ServiceControlsAccepted::UserModeReboot); }
    result
}

//endregion

//region --- ServiceControl ---

/// Provides dwControl values when calling Windows API [ControlService](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-controlservice)
/// or receiving dwControl param via [LPHANDLER_FUNCTION](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nc-winsvc-lphandler_function)
pub enum ServiceControl {
    /// Presents SERVICE_CONTROL_CONTINUE value
    Continue = SERVICE_CONTROL_CONTINUE as isize,
    /// Presents SERVICE_CONTROL_INTERROGATE value
    Interrogate = SERVICE_CONTROL_INTERROGATE as isize,
    /// Presents SERVICE_CONTROL_NETBINDADD value
    NetBindAdd = SERVICE_CONTROL_NETBINDADD as isize,
    /// Presents SERVICE_CONTROL_NETBINDDISABLE value
    NetBindDisable = SERVICE_CONTROL_NETBINDDISABLE as isize,
    /// Presents SERVICE_CONTROL_NETBINDENABLE value
    NetBindEnable = SERVICE_CONTROL_NETBINDENABLE as isize,
    /// Presents SERVICE_CONTROL_NETBINDREMOVE value
    NetBindRemove = SERVICE_CONTROL_NETBINDREMOVE as isize,
    /// Presents SERVICE_CONTROL_PARAMCHANGE value
    ParamChange = SERVICE_CONTROL_PARAMCHANGE as isize,
    /// Presents SERVICE_CONTROL_PAUSE value
    Pause = SERVICE_CONTROL_PAUSE as isize,
    /// Presents SERVICE_CONTROL_STOP value
    Stop = SERVICE_CONTROL_STOP as isize,
}

impl TryFrom<u32> for ServiceControl {
    type Error = std::io::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == (ServiceControl::Continue as u32) {
            Ok(ServiceControl::Continue)
        } else if value == (ServiceControl::Interrogate as u32) {
            Ok(ServiceControl::Interrogate)
        } else if value == (ServiceControl::NetBindAdd as u32) {
            Ok(ServiceControl::NetBindAdd)
        } else if value == (ServiceControl::NetBindDisable as u32) {
            Ok(ServiceControl::NetBindDisable)
        } else if value == (ServiceControl::NetBindEnable as u32) {
            Ok(ServiceControl::NetBindEnable)
        } else if value == (ServiceControl::NetBindRemove as u32) {
            Ok(ServiceControl::NetBindRemove)
        } else if value == (ServiceControl::ParamChange as u32) {
            Ok(ServiceControl::ParamChange)
        } else if value == (ServiceControl::Pause as u32) {
            Ok(ServiceControl::Pause)
        } else if value == (ServiceControl::Stop as u32) {
            Ok(ServiceControl::Stop)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Invalid value."))
        }
    }
}

//endregion

//region --- ServiceState ---

/// Provides dwCurrentState from [SERVICE_STATUS](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status)
#[derive(Debug, PartialEq)]
pub enum ServiceState {
    /// Represents SERVICE_CONTINUE_PENDING value
    ContinuePending = SERVICE_CONTINUE_PENDING.0 as isize,
    /// Represents SERVICE_PAUSE_PENDING value
    PausePending = SERVICE_PAUSE_PENDING.0 as isize,
    /// Represents SERVICE_PAUSED value
    Paused = SERVICE_PAUSED.0 as isize,
    /// Represents SERVICE_RUNNING value
    Running = SERVICE_RUNNING.0 as isize,
    /// Represents SERVICE_START_PENDING value
    StartPending = SERVICE_START_PENDING.0 as isize,
    /// Represents SERVICE_STOP_PENDING value
    StopPending = SERVICE_STOP_PENDING.0 as isize,
    /// Represents SERVICE_STOPPED value
    Stopped = SERVICE_STOPPED.0 as isize,
}

impl TryFrom<u32> for ServiceState {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == SERVICE_CONTINUE_PENDING.0 {
            Ok(ServiceState::ContinuePending)
        } else if value == SERVICE_PAUSE_PENDING.0 {
            Ok(ServiceState::PausePending)
        } else if value == SERVICE_PAUSED.0 {
            Ok(ServiceState::Paused)
        } else if value == SERVICE_RUNNING.0 {
            Ok(ServiceState::Running)
        } else if value == SERVICE_START_PENDING.0 {
            Ok(ServiceState::StartPending)
        } else if value == SERVICE_STOP_PENDING.0 {
            Ok(ServiceState::StopPending)
        } else if value == SERVICE_STOPPED.0 {
            Ok(ServiceState::Stopped)
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid service state value"))
        }
    }
}

//endregion

//region --- ServiceStatus ---

/// Provides Windows API [SERVICE_STATUS](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status)
pub struct ServiceStatus {
    pub service_type: ServiceType,
    pub service_state: ServiceState,
    pub controls_accepted: Vec<ServiceControlsAccepted>,
    pub win32_exit_code: u32,
    pub service_specific_exit_code: u32,
    pub check_point: u32,
    pub wait_hint: u32,
}

impl TryFrom<SERVICE_STATUS> for ServiceStatus {
    type Error = Error;

    fn try_from(value: SERVICE_STATUS) -> Result<Self, Self::Error> {
        Ok(ServiceStatus {
            service_type: ServiceType::try_from(value.dwServiceType.0).unwrap(),
            service_state: ServiceState::try_from(value.dwCurrentState.0).unwrap(),
            controls_accepted: u32_to_controls_accepted(value.dwControlsAccepted),
            win32_exit_code: value.dwWin32ExitCode,
            service_specific_exit_code: value.dwServiceSpecificExitCode,
            check_point: value.dwCheckPoint,
            wait_hint: value.dwWaitHint,
        })
    }
}

//endregion

/// Calls Windows API [OpenSCManagerW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw)
pub fn open_service_manager() -> Result<ServiceHandle, Error> {
    unsafe {
        let result = OpenSCManagerW(PWSTR::NULL, PWSTR::NULL, SC_MANAGER_ALL_ACCESS);
        if result.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(ServiceHandle(result))
        }
    }
}

/// Calls Windows API [OpenServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openservicew)
pub fn open_service(handle: &ServiceHandle, service_name: &str)
                    -> Result<ServiceHandle, Error> {
    unsafe {
        let result = OpenServiceW(handle.0, service_name, SERVICE_ALL_ACCESS);
        if result.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(ServiceHandle(result))
        }
    }
}

/// Calls Windows API [StartServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicew)
pub fn start_service(handle: &ServiceHandle) -> Result<(), Error> {
    unsafe {
        if StartServiceW(handle.0, 0, std::ptr::null_mut()).as_bool() {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

/// Calls Windows API [CloseServiceHandle](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-closeservicehandle)
pub fn close_service_handle(handle: &ServiceHandle) {
    unsafe {
        CloseServiceHandle(handle.0);
    }
}

/// Calls Windows API [ControlService](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-controlservice)
pub fn control_service(handle: &ServiceHandle, control: ServiceControl) -> Result<(), Error> {
    unsafe {
        let mut status = SERVICE_STATUS {
            dwServiceType: SERVICE_FILE_SYSTEM_DRIVER,
            dwCurrentState: SERVICE_STOPPED,
            dwControlsAccepted: 0,
            dwWin32ExitCode: 0,
            dwServiceSpecificExitCode: 0,
            dwCheckPoint: 0,
            dwWaitHint: 0,
        };

        if ControlService(handle.0, control as u32, &mut status).as_bool() {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

/// Calls Windows API [SetServiceStatus](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus)
pub fn set_service_status(handle: &ServiceStatusHandle, current_status: ServiceStatus) -> Result<(), Error> {
    let mut controls_accepted: u32 = 0;
    for it in current_status.controls_accepted {
        controls_accepted |= it as u32;
    }

    let mut status = SERVICE_STATUS {
        dwServiceType: ENUM_SERVICE_TYPE(current_status.service_type as u32),
        dwCurrentState: SERVICE_STATUS_CURRENT_STATE(current_status.service_state as u32),
        dwControlsAccepted: controls_accepted,
        dwWin32ExitCode: current_status.win32_exit_code,
        dwServiceSpecificExitCode: current_status.service_specific_exit_code,
        dwCheckPoint: current_status.check_point,
        dwWaitHint: current_status.wait_hint,
    };

    unsafe {
        if SetServiceStatus(handle.0, &mut status).as_bool() {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

/// Calls Windows API [CreateServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
/// to create a service.
pub fn create_service(
    handle: &ServiceHandle,
    name: &str,
    display_name: &str,
    desired_access: u32,
    service_type: ServiceType,
    start_type: ServiceStartType,
    error_control: ServiceErrorControl,
    binary_path: &str,
    load_order_group: Option<String>,
    tag_id: Option<u32>,
    dependencies: Option<String>,
    service_start_name: Option<String>,
    password: Option<String>,
) -> Result<ServiceHandle, Error> {
    let tag = match tag_id {
        None => {
            std::ptr::null_mut()
        }
        Some(value) => {
            value as *mut u32
        }
    };

    unsafe {
        let h = CreateServiceW(
            handle.0,
            name,
            display_name,
            desired_access,
            service_type.into(),
            start_type.into(),
            error_control.into(),
            binary_path,
            Into::<PWSTR>::into(load_order_group),
            tag,
            Into::<PWSTR>::into(dependencies),
            Into::<PWSTR>::into(service_start_name),
            Into::<PWSTR>::into(password),
        );

        Ok(ServiceHandle(h))
    }
}

/// Update service description value by calling Windows API [ChangeServiceConfig2W](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w)
/// with SERVICE_CONFIG_DESCRIPTION as dwInfoLevel argument
pub fn change_service_config_description(handle: &ServiceHandle, value: &str) -> Result<(), Error> {
    let mut v: Vec<u16> = value.encode_utf16().collect();
    v.push(0);
    let des: *mut c_void = &mut v as *mut _ as *mut c_void;

    unsafe {
        if ChangeServiceConfig2W(handle.0, SERVICE_CONFIG_DESCRIPTION, des).as_bool() {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

/// Calls Windows API [QueryServiceStatus](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-queryservicestatus)
pub fn query_service_status(handle: &ServiceHandle) -> Result<ServiceStatus, Error> {
    unsafe {
        let mut status = SERVICE_STATUS {
            dwServiceType: SERVICE_FILE_SYSTEM_DRIVER,
            dwCurrentState: SERVICE_STOPPED,
            dwControlsAccepted: 0,
            dwWin32ExitCode: 0,
            dwServiceSpecificExitCode: 0,
            dwCheckPoint: 0,
            dwWaitHint: 0,
        };

        if QueryServiceStatus(handle.0, &mut status).as_bool() {
            ServiceStatus::try_from(status)
        } else {
            Err(Error::last_os_error())
        }
    }
}

/// Calls Windows API [DeleteService](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-deleteservice)
pub fn delete_service(handle: &ServiceHandle) -> Result<(), Error> {
    unsafe {
        if DeleteService(handle.0).as_bool() {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

//region --- PWSTR ---

impl Into<PWSTR> for &str {
    fn into(self) -> PWSTR {
        let mut data: Vec<u16> = self.encode_utf16().collect();
        PWSTR(data.as_mut_ptr())
    }
}

impl Into<PWSTR> for String {
    fn into(self) -> PWSTR {
        let mut data: Vec<u16> = self.encode_utf16().collect();
        PWSTR(data.as_mut_ptr())
    }
}

impl Into<PWSTR> for Option<String> {
    fn into(self) -> PWSTR {
        let value: *mut u16 = match self {
            None => {
                PWSTR::NULL.0
            }
            Some(value) => {
                let mut data: Vec<u16> = value.encode_utf16().collect();
                data.as_mut_ptr()
            }
        };

        PWSTR(value)
    }
}

//endregion

pub type ServiceMainFunc = fn(Vec<String>);
pub type ServiceCtrlFunc = fn(ServiceControl);

static mut MAIN_FUNC: Option<ServiceMainFunc> = None;
static mut CTRL_FUNC: Option<ServiceCtrlFunc> = None;

unsafe extern "system" fn svc_main(size: u32, pointer: *mut PWSTR) {
    let values: Vec<String> = pointer_to_string_array((*pointer).0, size);

    if MAIN_FUNC.is_some() {
        // MAIN_FUNC.as_ref().unwrap()(args);
        MAIN_FUNC.as_ref().unwrap()(values);
    }
}

unsafe extern "system" fn svc_ctrl(control: u32) {
    if CTRL_FUNC.is_some() {
        let control = ServiceControl::try_from(control)
            .unwrap();
        CTRL_FUNC.as_ref().unwrap()(control);
    }
}

fn pointer_to_string_array(pointer: *mut u16, count: u32) -> Vec<String> {
    let mut p = pointer;
    let mut result: Vec<String> = vec![];

    for _ in 0..count {
        let len = len(p);

        match pointer_to_string(p, len) {
            None => {}
            Some(value) => {
                result.push(value);
            }
        }

        p = unsafe { p.add(len + 1) };
    }

    result
}

fn len(pointer: *mut u16) -> usize {
    let mut p = pointer;
    let mut len = 0;
    while unsafe { *p } != 0 {
        p = unsafe { p.add(1) };
        len += 1;
    }
    len
}

fn pointer_to_string(pointer: *mut u16, len: usize) -> Option<String> {
    if len > 0 {
        let data = unsafe { std::slice::from_raw_parts(pointer, len) };
        let result = String::from_utf16(data).unwrap();
        Some(result)
    } else {
        None
    }
}

/// Calls Windows API [StartServiceCtrlDispatcherW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicectrldispatcherw)
pub fn start_service_ctrl_dispatcher(service_name: &str, service_proc: Option<ServiceMainFunc>) -> Result<(), Error> {
    unsafe {
        MAIN_FUNC = service_proc;

        let tbl = SERVICE_TABLE_ENTRYW {
            lpServiceName: service_name.into(),
            lpServiceProc: Some(svc_main),
        };

        if StartServiceCtrlDispatcherW(&tbl).as_bool() {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

/// Calls Windows API [RegisterServiceCtrlHandlerW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-registerservicectrlhandlerw)
pub fn register_service_ctrl_handler(service_name: &str, ctrl_proc: Option<ServiceCtrlFunc>) -> Result<ServiceStatusHandle, Error> {
    unsafe {
        CTRL_FUNC = ctrl_proc;
        let handle = RegisterServiceCtrlHandlerW(
            service_name,
            Some(svc_ctrl));

        if handle.0 != 0 {
            Ok(ServiceStatusHandle(handle))
        } else {
            Err(Error::last_os_error())
        }
    }
}
