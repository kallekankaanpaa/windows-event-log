use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::PSID;
use windows::Win32::System::EventLog::{
    DeregisterEventSource, EventSourceHandle, RegisterEventSourceW, ReportEventW, REPORT_EVENT_TYPE,
};

use std::ptr;

mod error;
mod key;
mod messages;
mod registry;
pub use key::*;

struct InnerLogger {
    handle: EventSourceHandle,
    level: log::Level,
}

/// Data struct used to configure and register the event source
pub struct EventLog {
    level: log::Level,
    source: String,
    event_log_key: EventLogKey,
}

impl EventLog {
    /// Constructs a new EventLog
    pub fn new(key: EventLogKey, source: impl Into<String>, level: log::Level) -> Self {
        Self {
            level,
            source: source.into(),
            event_log_key: key,
        }
    }

    /// Sets the `EventMessageFile` registry key to point to current executable
    ///
    /// Requires admin privileges
    /// Calling this function is only necessary once on install, if the executable doesn't move.
    /// Programs which need to be run as Administrator anyways may call this function every time they register their logger
    pub fn set_message_file_location(self) -> Result<Self, Box<dyn std::error::Error>> {
        registry::set_message_file_location(&self.event_log_key, &self.source)?;
        Ok(self)
    }

    /// Registers the event source, and sets up the logger
    pub fn register(self) -> Result<Self, Box<dyn std::error::Error>> {
        let handle = Self::register_event_source(&self.source)?;

        let logger = InnerLogger {
            handle,
            level: self.level,
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(log::LevelFilter::Trace);
        Ok(self)
    }

    /// Calls the win32 (RegisterEventSourceW)[https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-registereventsourcew] function with the configured source
    fn register_event_source(source: &str) -> Result<EventSourceHandle, error::EventLogError> {
        let mut source_char_seq = str::encode_utf16(source).collect::<Vec<u16>>();
        source_char_seq.push(0);
        let handle =
            unsafe { RegisterEventSourceW(PCWSTR(ptr::null()), PCWSTR(source_char_seq.as_ptr()))? };
        Ok(handle)
    }
}

impl log::Log for InnerLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }
    /// Flush is no-op since this logger doesn't hold any buffers
    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = format!("{}\0", record.args());
        let mut code_points: Vec<u16> = message.encode_utf16().collect();
        code_points.push(0);

        let type_and_id = match record.level() {
            log::Level::Error => (EventLogType::Error, messages::ERROR),
            log::Level::Warn => (EventLogType::Warning, messages::WARNING),
            log::Level::Info => (EventLogType::Information, messages::INFO),
            log::Level::Debug => (EventLogType::Information, messages::DEBUG),
            log::Level::Trace => (EventLogType::Information, messages::TRACE),
        };

        unsafe {
            ReportEventW(
                self.handle,
                REPORT_EVENT_TYPE(type_and_id.0 as u16),
                0,
                type_and_id.1,
                PSID(ptr::null_mut()),
                0,
                &[PWSTR(code_points.as_mut_ptr())],
                ptr::null(),
            )
            .as_bool();
        }
    }
}

impl Drop for InnerLogger {
    fn drop(&mut self) {
        unsafe { DeregisterEventSource(self.handle).as_bool() };
    }
}

#[allow(dead_code)]
enum EventLogType {
    Success = 0x0000,
    AuditFailure = 0x0010,
    AuditSuccess = 0x0008,
    Error = 0x0001,
    Information = 0x0004,
    Warning = 0x0002,
}

#[test]
fn log_to_event_log() {
    EventLog::new(
        EventLogKey::Custom("AAPPLICATION".to_string()),
        "AAATEST",
        log::Level::Trace,
    )
    .set_message_file_location()
    .unwrap()
    .register()
    .unwrap();
    log::info!("Test log")
}
