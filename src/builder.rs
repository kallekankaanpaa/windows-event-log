use crate::{registry, *};

#[derive(Default)]
pub struct EventLogBuilder {
    set_registry: bool,
    level: Option<log::Level>,
    event_log_key: EventLogKey,
    event_source: String,
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

    pub fn event_source<T: Into<String>>(mut self, source: T) -> Self {
        self.event_source = source.into();
        self
    }

    pub fn register(self) {
        if self.set_registry {
            let result =
                registry::set_message_file_location(self.event_log_key, &self.event_source);
        }
        let handle = register_event_source(&self.event_source).unwrap();

        if self.level.is_none() {
            panic!("level needs to be set");
        }

        let logger = EventLog {
            handle: handle,
            level: self.level.unwrap(),
        };
        log::set_boxed_logger(Box::new(logger));
        log::set_max_level(log::LevelFilter::Trace);
    }
}

#[test]
fn builder_pattern() {
    EventLog::builder()
        .level(log::Level::Info)
        .event_source("Builder Test")
        .set_registry(false)
        .register();
}
