use serde::Serialize;
use utoipa::ToSchema;
use veno_core::notifier::{ArtifactSelection, Notifier, Sink};

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct NotifierResponse {
    pub name: String,
    pub sink: SinkDto,
    pub artifact_ids: ArtifactSelectionDto,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(tag = "type")] // Use tag-based enum for sink type
pub enum SinkDto {
    #[serde(rename = "slack")]
    Slack(SlackSink),
    #[serde(rename = "email")]
    Email(EmailSink),
    #[serde(rename = "google_chat")]
    GoogleChat(GoogleChatSink),
    #[serde(rename = "webhook")]
    Webhook(WebhookSink),
    #[serde(rename = "console")]
    Console(ConsoleSink),
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(untagged)]
pub enum ArtifactSelectionDto {
    All(AllMarker),
    Specific(Vec<String>),
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AllMarker {
    All,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SlackSink {
    pub webhook: String,
}

#[derive(Serialize, Clone, Debug, ToSchema)]
pub struct EmailSink {
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub to: Vec<String>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GoogleChatSink {
    pub webhook: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct WebhookSink {
    pub webhook: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ConsoleSink {}

impl From<Notifier> for NotifierResponse {
    fn from(value: Notifier) -> Self {
        Self {
            name: value.name,
            sink: value.sink.into(),
            artifact_ids: value.artifact_ids.into(),
        }
    }
}

impl From<Sink> for SinkDto {
    fn from(value: Sink) -> Self {
        match value {
            Sink::Slack(slack) => SinkDto::Slack(SlackSink {
                webhook: slack.webhook,
            }),
            Sink::Email(email) => SinkDto::Email(EmailSink {
                host: email.host,
                port: email.port,
                username: String::from("[REDACTED]"),
                password: String::from("[REDACTED]"),
                to: email.to,
                subject: email.subject,
            }),
            Sink::GoogleChat(google) => SinkDto::GoogleChat(GoogleChatSink {
                webhook: google.webhook,
            }),
            Sink::Webhook(webhook) => SinkDto::Webhook(WebhookSink {
                webhook: webhook.webhook,
            }),
            Sink::Console(_) => SinkDto::Console(ConsoleSink {}),
        }
    }
}

impl From<ArtifactSelection> for ArtifactSelectionDto {
    fn from(value: ArtifactSelection) -> Self {
        match value {
            ArtifactSelection::All(_) => ArtifactSelectionDto::All(AllMarker::All),
            ArtifactSelection::Specific(x) => ArtifactSelectionDto::Specific(x),
        }
    }
}
