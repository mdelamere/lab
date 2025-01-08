# File Integrity Checker

A Rust-based tool to monitor and validate the integrity of files in a WordPress directory. The tool calculates cryptographic hashes of files to detect unauthorized changes and can notify administrators via Slack or email.

---

## Features

- **File Integrity Monitoring**: Detects modified, new, or deleted files.
- **Notifications**: Sends alerts via Slack or email when changes are detected.
- **Baseline Management**: Supports generating and updating baselines for file integrity checks.
- **Configuration via TOML**: Flexible and easy configuration through `config.toml`.

---

## Requirements

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Slack Webhook** (if using Slack notifications)
- **SMTP Server** (if using email notifications)

---

## Installation

1. Clone the repository:
   ```bash
   git clone git@github.com:mdelamere/lab.git
   cd file-integrity-checker
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Create the configuration file:
   ```bash
   cp config.toml.example config.toml
   ```

4. Edit the `config.toml` file to set up your specific details (e.g., Slack webhook, email settings).

---

## Usage

### **Run the Integrity Check**

```bash
cargo run
```

The tool will:
1. Generate a baseline if it doesn’t exist.
2. Compare the current state of files to the baseline.
3. Notify via Slack or email if changes are detected.

### **Update the Baseline**

Use the `--update-baseline` flag to regenerate the baseline:

```bash
cargo run -- --update-baseline
```

A Slack notification will be sent (if configured) to confirm the update.

---

## Configuration

### **Example `config.toml`**

The `config.toml` file allows you to configure general settings, notification preferences, and Slack or email details.

```toml
[general]
client_name = "Your Client Name"
server_name = "Your Server Name"
wordpress_dir = "/path/to/wordpress"
baseline_file = "baseline.json"

[notifications]
send_via = "slack"

[slack]
webhook_url = "https://hooks.slack.com/services/your-webhook-url"

[email]
admin_email = "admin@example.com"
sender_email = "file-checker@example.com"
smtp_server = "smtp.example.com"
smtp_port = 587
smtp_username = "your-smtp-username"
smtp_password = "your-smtp-password"
```

### **Setup Instructions**

1. Copy the example configuration:
   ```bash
   cp config.toml.example config.toml
   ```

2. Update the `config.toml` file with your project-specific details.

---

## Notifications

### **Slack**
- Add a Slack webhook URL to the `[slack]` section in `config.toml`.
- Notifications will be sent to the configured Slack channel.

### **Email**
- Provide SMTP server details in the `[email]` section of `config.toml`.
- Ensure the `send_via` field is set to `smtp`.

---

## Development

### **Run Tests**
To run tests for the project:
```bash
cargo test
```

### **Linting**
Use `clippy` to check for code issues:
```bash
cargo clippy
```

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
