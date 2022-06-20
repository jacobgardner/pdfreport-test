use logzio::FromTracingData;
use serde::Serialize;
use tracing::{field::Visit, span};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogData {
    source: String,
    request_id: String,
    environment: String,
}

impl Default for LogData {
    fn default() -> Self {
        Self {
            environment: std::env::var("LOGGER_REMOTE_ENV").unwrap_or_else(|_| {
                std::env::var("LOGGER_ENV").unwrap_or_else(|_| {
                    std::env::var("ENV").unwrap_or_else(|_| String::from("local"))
                })
            }),
            source: String::from("api-pdf-generation"),
            request_id: String::from("No Request Id"),
        }
    }
}

impl Visit for LogData {
    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {}

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "request_id" {
            self.request_id = value.to_owned();
        }
    }
}

impl FromTracingData for LogData {
    fn from_event(event: &tracing::Event) -> Self {
        let mut log_data = LogData::default();

        event.record(&mut log_data);

        log_data
    }

    fn from_span(attrs: &span::Attributes) -> Self {
        let mut log_data = LogData::default();

        attrs.record(&mut log_data);

        log_data
    }
}
