fn main() {
    init();
}

#[cfg(windows)]
fn init() {
    windows::build! {
        Windows::Win32::Security::SC_HANDLE,

        Windows::Win32::Foundation::PWSTR,
        Windows::Win32::System::Services::*,
        Windows::Win32::System::SystemServices::*,

        Windows::Win32::System::Diagnostics::Debug::GetLastError,
        Windows::Win32::System::Diagnostics::Debug::WIN32_ERROR,

        Windows::Win32::System::EventLog::RegisterEventSourceW,
        Windows::Win32::System::EventLog::ReportEventW,

        Windows::Win32::Foundation::PSID,
        Windows::Win32::System::EventLog::EventSourceHandle,
        Windows::Win32::System::EventLog::DeregisterEventSource,
    }
}

#[cfg(unix)]
fn init() {}