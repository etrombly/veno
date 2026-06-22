# Notifiers

Each notifier declares which artifacts it should check via `artifact_ids`.

Use an array to target specific artifacts:

```json
"artifact_ids": ["rust", "nginx_dockerhub"]
```

Use `"all"` to target every configured artifact:

```json
"artifact_ids": "all"
```

## Email

```jsonc
{
  "name": "private_email",
  "sink": {
    "type": "email",
    "host": "smtp.gmail.com",
    "port": 587, // optional - default: 587
    "username": "username",
    "password": "${EMAIL_PASSWORD}",
    "to": "recipient@somemail.com",
    "subject": "New version available" // optional - default: "VENO: New version available"
  },
  "artifact_ids": ["rust"]
}
```

## Webhook

The webhook value is just the `url` to the webhook. `Slack` and `Google Chat` use the webhook logic under the hood but they will offer a default chat card in the future.

```json
{
  "name": "generic_webhook",
  "sink": {
    "type": "webhook",
    "webhook": ".."
  },
  "artifact_ids": ["rust", "nginx_dockerhub"]
}
```

## Slack

```json
{
  "name": "team_slack",
  "sink": {
    "type": "slack",
    "webhook": ".."
  },
  "artifact_ids": ["keycloak_helm_chart", "nginx_dockerhub"]
}
```

## Google Chat

```json
{
  "name": "team_google_chat",
  "sink": {
    "type": "google_chat",
    "webhook": "..."
  },
  "artifact_ids": "all"
}
```

## Console

```json
{
  "name": "local_console",
  "sink": {
    "type": "console"
  },
  "artifact_ids": "all"
}
```
