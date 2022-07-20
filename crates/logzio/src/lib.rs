use chrono::{DateTime, Utc};
use crossbeam_channel::{unbounded, Sender};
use serde::Serialize;
use std::marker::PhantomData;
use std::time::Duration;
use std::{fmt::Debug, time::Instant};
use tracing::field::Visit;
use tracing::{span, Subscriber};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

use log::Record;
use log4rs::append::Append;

// TODO: Use Duration
type LogzioDuration = usize;

#[derive(Serialize)]
pub struct Message<T: Serialize = Noop> {
    #[serde(rename = "@timestamp")]
    timestamp: LogzioTimestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<LogzioDuration>,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<String>,
    #[serde(flatten)]
    custom_fields: T,
}

pub struct LogzioTimestamp(DateTime<Utc>);

impl Serialize for LogzioTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}

enum ChannelMessage<T: Serialize> {
    LogMessage(Message<T>),
    Flush,
}

pub struct LogzIoSender<T: Serialize = Noop> {
    host: String,
    shipping_token: String,
    message_queue: Sender<ChannelMessage<T>>,
}

impl<T: Serialize> Debug for LogzIoSender<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogzIoSender")
            .field("host", &self.host)
            .field("shipping_token", &self.shipping_token)
            .finish()
    }
}

pub struct LogzIoSenderBuilder<T: Serialize> {
    max_buffer_size: usize,
    max_message_count: usize,
    send_interval: Duration,
    max_retry_count: usize,
    host: String,
    shipping_token: String,
    _message_type: PhantomData<T>,
}

impl<T: Serialize + Send + 'static> LogzIoSenderBuilder<T> {
    pub fn new(host: String, shipping_token: String) -> Self {
        Self {
            // 1 MiB
            max_buffer_size: 1024 * 1024,
            max_message_count: 100,
            send_interval: Duration::from_secs(10),
            max_retry_count: 10,
            host,
            shipping_token,
            _message_type: PhantomData,
        }
    }

    pub fn with_buffer_size(mut self, byte_count: usize) -> Self {
        self.max_buffer_size = byte_count;
        self
    }

    pub fn with_max_message_count(mut self, max_message_count: usize) -> Self {
        self.max_message_count = max_message_count;
        self
    }

    pub fn with_send_interval(mut self, send_interval: Duration) -> Self {
        self.send_interval = send_interval;
        self
    }

    pub fn build(self) -> LogzIoSender<T> {
        LogzIoSender::with_options(
            self.host,
            self.shipping_token,
            self.max_message_count,
            self.max_buffer_size,
            self.send_interval,
            self.max_retry_count,
        )
    }
}

impl<T: Serialize + Send + 'static> LogzIoSender<T> {
    fn with_options(
        host: String,
        shipping_token: String,
        max_message_count: usize,
        max_buffer_size: usize,
        send_interval: Duration,
        max_retry_count: usize,
    ) -> Self {
        let (tx, rx) = unbounded();

        let inner_host = host.clone();
        let inner_shipping_token = shipping_token.clone();

        std::thread::spawn(move || {
            let mut log_messages = String::new();
            let mut message_count = 0;
            let mut deadline;

            let client = reqwest::blocking::Client::new();

            loop {
                deadline = Instant::now() + send_interval;

                while let Ok(message) = rx.recv_deadline(deadline) {
                    match message {
                        ChannelMessage::LogMessage(message) => {
                            log_messages += &serde_json::to_string(&message).unwrap();
                            log_messages += "\n";
                            message_count += 1;
                        }
                        ChannelMessage::Flush => break,
                    }

                    if message_count > max_message_count || log_messages.len() > max_buffer_size {
                        break;
                    }
                }
                
                if message_count == 0 {
                    continue;
                }

                let mut retry_count = 0;

                // TODO: Add exponential backoff
                loop {
                    if client
                        .post(format!("https://{}:8071/", inner_host))
                        .query(&[("token", inner_shipping_token.as_ref()), ("type", "rust")])
                        .body(log_messages.clone())
                        .send()
                        .is_ok()
                    {
                        break;
                    } else {
                        retry_count += 1;

                        if retry_count > max_retry_count {
                            break;
                        }

                        std::thread::sleep(Duration::from_secs(1));
                    }
                }

                message_count = 0;
                log_messages.clear();
            }
        });

        Self {
            host,
            shipping_token,
            message_queue: tx,
        }
    }

    pub fn send_message(&self, msg: Message<T>) {
        // Can return an error, but... do we care?
        let _ = self.message_queue.send(ChannelMessage::LogMessage(msg));
    }

    pub fn flush(&self) {
        // Can return an error, but... do we care?
        let _ = self.message_queue.send(ChannelMessage::Flush);
    }
}

impl<T> Append for LogzIoSender<T>
where
    for<'a, 'b> T: Serialize + Send + From<&'b Record<'a>> + 'static,
{
    fn append(&self, record: &Record) -> anyhow::Result<()> {
        let message = Message {
            timestamp: LogzioTimestamp(Utc::now()),
            duration: None,
            message: record.args().to_string(),
            level: Some(record.level().to_string()),
            custom_fields: T::from(record),
        };

        self.send_message(message);

        Ok(())
    }

    fn flush(&self) {
        LogzIoSender::flush(self)
    }
}

#[derive(Default)]
struct Visitor {
    message: String,
}

impl Visit for Visitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_owned();
        }
    }
}

#[derive(Serialize)]
pub struct Noop;

impl<'a> From<&Record<'a>> for Noop {
    fn from(_: &Record<'a>) -> Self {
        Self
    }
}

impl<T, S> Layer<S> for LogzIoSender<T>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    T: FromTracingData + Send + Serialize + 'static,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut event_visitor = Visitor::default();

        event.record(&mut event_visitor);

        let message = Message {
            timestamp: LogzioTimestamp(Utc::now()),
            // Events are always instantaneous, thus always have a duration of None
            duration: None,
            message: event_visitor.message,
            level: Some(event.metadata().level().to_string()),
            custom_fields: T::from_event(event),
        };

        self.send_message(message);
    }

    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        _id: &span::Id,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut event_visitor = Visitor::default();

        attrs.record(&mut event_visitor);

        let message = Message {
            timestamp: LogzioTimestamp(Utc::now()),
            // TODO: Future enhancement
            // Not currently tracking span duration or anything
            duration: None,
            message: attrs.metadata().name().to_string(),
            level: Some(attrs.metadata().level().to_string()),
            custom_fields: T::from_span(attrs),
        };

        self.send_message(message);
    }
}

pub trait FromTracingData {
    fn from_event(event: &tracing::Event) -> Self;
    fn from_span(attrs: &span::Attributes) -> Self;
}
