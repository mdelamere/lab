mod hash;
mod notify;
mod slack;
mod compare;
mod logger;

use std::env;
use std::path::Path;
use serde::Deserialize;
use log::{info, error};

#[derive(Debug, Deserialize)]
struct Config {
    general: GeneralConfig,
    notifications: NotificationConfig,
    slack: SlackConfig,
    email: EmailConfig,
}

#[derive(Debug, Deserialize)]
struct GeneralConfig {
    client_name: String,
    server_name: String,
    wordpress_dir: String,
    baseline_file: String,
}

#[derive(Debug, Deserialize)]
struct NotificationConfig {
    send_via: String,
}

#[derive(Debug, Deserialize)]
struct SlackConfig {
    webhook_url: String,
}

#[derive(Debug, Deserialize)]
struct EmailConfig {
    admin_email: String,
    sender_email: String,
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
}

fn load_config() -> Config {
    config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
}

fn main() -> std::io::Result<()> {
    logger::init_logging("integrity_checker.log");

    info!("Starting File Integrity Checker...");

    let config = load_config();
    let wordpress_dir = Path::new(&config.general.wordpress_dir);
    let baseline_file = Path::new(&config.general.baseline_file);

    let args: Vec<String> = env::args().collect();
    if !baseline_file.exists() || args.len() > 1 && args[1] == "--update-baseline" {
        update_baseline(
            wordpress_dir,
            baseline_file,
            &config,
            args.len() > 1 && args[1] == "--update-baseline",
        )?;
        if args.len() > 1 && args[1] == "--update-baseline" {
            return Ok(());
        }
    }

    info!("Checking for changes...");
    let baseline_hashes = match hash::load_hashes(baseline_file) {
        Ok(hashes) => hashes,
        Err(err) => {
            error!("Failed to load baseline file: {}", err);
            return Err(err);
        }
    };

    let current_hashes = match hash::hash_directory(wordpress_dir) {
        Ok(hashes) => hashes,
        Err(err) => {
            error!("Failed to hash directory: {}", err);
            return Err(err);
        }
    };

    let (modified, new_files, deleted) = compare::compare_hashes(&current_hashes, &baseline_hashes);

    if !modified.is_empty() || !new_files.is_empty() || !deleted.is_empty() {
        let body = build_notification_body(
            &config.general.client_name,
            &config.general.server_name,
            &modified,
            &new_files,
            &deleted,
        );

        match config.notifications.send_via.as_str() {
            "slack" => {
                if let Err(err) = slack::send_slack_message(&config.slack.webhook_url, &body) {
                    error!("Failed to send Slack message: {}", err);
                } else {
                    info!("Slack notification sent successfully.");
                }
            }
            "smtp" => {
                if let Err(err) = notify::send_email(
                    "[File Checker] Changes Detected",
                    &body,
                    &config.email.admin_email,
                    &config.email.sender_email,
                    &config.email.smtp_server,
                    config.email.smtp_port,
                    &config.email.smtp_username,
                    &config.email.smtp_password,
                ) {
                    error!("Failed to send email: {}", err);
                } else {
                    info!("Email sent successfully.");
                }
            }
            _ => {
                error!("Unknown notification method specified: {}", config.notifications.send_via);
            }
        }
    } else {
        info!("No changes detected.");
    }

    info!("File Integrity Checker finished.");
    Ok(())
}

fn build_notification_body(
    client_name: &str,
    server_name: &str,
    modified: &[String],
    new_files: &[String],
    deleted: &[String],
) -> String {
    let mut body = format!(
        "Warning for {} on {}: File integrity issues detected.\n\n",
        client_name, server_name
    );

    if !modified.is_empty() {
        body.push_str("Modified Files:\n");
        for file in modified {
            body.push_str(&format!("  - {}\n", file));
        }
    }

    if !new_files.is_empty() {
        body.push_str("\nNew Files:\n");
        for file in new_files {
            body.push_str(&format!("  - {}\n", file));
        }
    }

    if !deleted.is_empty() {
        body.push_str("\nDeleted Files:\n");
        for file in deleted {
            body.push_str(&format!("  - {}\n", file));
        }
    }

    body
}

fn update_baseline(
    wordpress_dir: &Path,
    baseline_file: &Path,
    config: &Config,
    notify_update: bool,
) -> std::io::Result<()> {
    info!("Generating or updating baseline...");
    let file_hashes = match hash::hash_directory(wordpress_dir) {
        Ok(hashes) => hashes,
        Err(err) => {
            error!("Failed to hash directory: {}", err);
            return Err(err);
        }
    };

    if let Err(err) = hash::save_hashes(&file_hashes, baseline_file) {
        error!("Failed to save baseline file: {}", err);
        return Err(err);
    }

    info!("Baseline successfully saved.");

    if notify_update && config.notifications.send_via == "slack" {
        let message = format!(
            "Info for {} on {}: Baseline has been updated successfully.",
            config.general.client_name, config.general.server_name
        );

        if let Err(err) = slack::send_slack_message(&config.slack.webhook_url, &message) {
            error!("Failed to send Slack message: {}", err);
        } else {
            info!("Slack notification sent for baseline update.");
        }
    }

    Ok(())
}
