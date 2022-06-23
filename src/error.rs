use std::ptr;
use thiserror::Error;
use windows::Win32::{
    Foundation::WIN32_ERROR,
    System::{
        Diagnostics::Debug::{
            FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS,
        },
        Memory::LocalFree,
    },
};

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Failed to open the registry key: {0}")]
    FailedToOpen(String),
    #[error("Failed to set the registry key: {0}")]
    FailedToSet(String),
    #[error("Setting HKEY_LOCAL_MACHINE registry keys requires admin privileges")]
    AdminPrivilegesRequired,
}

#[derive(Error, Debug)]
pub enum EventLogError {
    #[error("Couldn't register/open event source: {0}")]
    RegisterFailed(#[from] windows::core::Error),
    #[error("Logger can only be initalized once")]
    InitalizationFailed(#[from] log::SetLoggerError),
    #[error("Failed to set message file registry entry")]
    RegistryError(#[from] RegistryError),
}

pub fn format_win_error(error: WIN32_ERROR) -> Option<String> {
    if error.is_err() {
        unsafe {
            let mut buffer: *mut u16 = ptr::null_mut();
            let lp_buffer = &mut buffer as *mut *mut u16 as *mut u16;
            let output_len = FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                ptr::null_mut(),
                error.0,
                0,
                std::mem::transmute(lp_buffer),
                0,
                ptr::null_mut(),
            );
            let output_buffer = std::slice::from_raw_parts(buffer, output_len as _);
            LocalFree(buffer as isize);

            Some(String::from_utf16_lossy(output_buffer))
        }
    } else {
        None
    }
}

#[test]
fn test_win_error_formatting() {
    let error = format_win_error(WIN32_ERROR(5));
    assert!(error.is_some());
    println!("{:?}", error);
    let error = format_win_error(WIN32_ERROR(6));
    assert!(error.is_some());
    println!("{:?}", error);
}
