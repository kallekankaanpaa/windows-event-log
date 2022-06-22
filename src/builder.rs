use crate::{registry, *};

pub struct EventLogBuilder {
    set_registry: bool,
    level: Option<log::Level>,
    event_log_key: EventLogKey,
    event_source: Option<String>,
}

impl Default for EventLogBuilder {
    fn default() -> Self {
        Self {
            set_registry: false,
            level: None,
            event_log_key: EventLogKey::default(),
            event_source: None,
        }
    }
}

impl EventLogBuilder {
    pub fn set_registry(mut self, set: bool) -> Self {
        self.set_registry = set;
        self
    }

    pub fn level(mut self, level: log::Level) -> Self {
        self.level = Some(level);
        self
    }

    pub fn event_log_key(mut self, key: EventLogKey) -> Self {
        self.event_log_key = key;
        self
    }

    pub fn event_source(mut self, source: impl Into<String>) -> Self {
        self.event_source = Some(source.into());
        self
    }

    pub fn register(self) -> Result<(), Box<dyn std::error::Error>> {
        if self.event_source.is_none() {
            panic!("event source is required");
        }
        let event_source = self.event_source.unwrap();

        if self.set_registry {
            registry::set_message_file_location(self.event_log_key, &event_source)?;
        }

        let handle = EventLog::register_event_source(&event_source)?;

        if self.level.is_none() {
            panic!("level needs to be set");
        }

        let logger = EventLog {
            handle: handle,
            level: self.level.unwrap(),
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(log::LevelFilter::Trace);
        Ok(())
    }
}

#[test]
fn builder_pattern() {
    assert!(EventLog::builder()
        .level(log::Level::Info)
        .event_source("Builder Test")
        .set_registry(false)
        .register()
        .is_ok());
}
