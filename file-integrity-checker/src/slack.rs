use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::json;

pub fn send_slack_message(webhook_url: &str, message: &str) -> Result<()> {
    let client = Client::new();

    let payload = json!({ "text": message });

    let response = client
        .post(webhook_url)
        .json(&payload)
        .send()
        .context("Failed to send Slack message")?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Slack message failed with status: {}",
            response.status()
        ))
    }
}
