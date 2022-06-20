use std::ptr;
use thiserror::Error;
use windows::Win32::{
    Foundation::WIN32_ERROR,
    System::Diagnostics::Debug::{
        FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
    },
};

use windows::core::PWSTR;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Failed to open the registry key: {0}")]
    FailedToOpen(String),
    #[error("Failed to set the registry key: {0}")]
    FailedToSet(String),
}

pub fn format_win_error(error: WIN32_ERROR) -> Option<String> {
    if error.is_err() {
        unsafe {
            println!("{}", error.0);
            let output = PWSTR::default();
            let output_len = FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER & FORMAT_MESSAGE_FROM_SYSTEM,
                ptr::null(),
                error.0,
                0,
                output,
                0,
                ptr::null(),
            );
            let output_buffer = std::slice::from_raw_parts(output.0, output_len as _);
            println!("{:?}", output_buffer);

            Some(String::from_utf16_lossy(output_buffer))
        }
    } else {
        None
    }
}
