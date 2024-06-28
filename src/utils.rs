use std::path::Path;
use std::fs;
use crate::error::BrokerError;

pub fn ensure_directory_exists<P: AsRef<Path>>(path: P) -> Result<(), BrokerError> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| BrokerError::IoError(e))?;
    }
    Ok(())
}

#[cfg(feature = "windows_debug")]
pub fn windows_output_debug_string(message: &str) {
    use std::ffi::CString;
    use winapi::um::debugapi::OutputDebugStringA;

    let c_message = CString::new(message).unwrap();
    unsafe {
        OutputDebugStringA(c_message.as_ptr());
    }
}

#[cfg(feature = "linux_syslog")]
pub fn linux_syslog(message: &str) {
    use syslog::{Facility, Formatter3164, Logger};

    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "lib_broker".into(),
        pid: 0,
    };

    let mut logger = Logger::new(formatter, syslog::LogOption::LOG_PID, Facility::LOG_USER)
        .expect("Failed to create syslog logger");

    logger.err(message).expect("Failed to log to syslog");
}
