//! Settings, read once from the environment (and `.env` in development).

use secrecy::SecretString;

use crate::engine::files::MasterKey;
use crate::engine::llm::AnthropicClient;
use crate::engine::mail::{ConfiguredMailer, MailpitMailer, ResendMailer};
use crate::engine::outbox::SendBudget;

/// Balanse's share of Resend Free's 100 emails a day, which it shares with
/// Kompleto (ADR 0002): 25 reminders, and 5 more for login emails and
/// notices.
pub const RESEND_FREE_SHARE: SendBudget = SendBudget {
    per_day: 25,
    reserve: 5,
};

/// Where email goes.
#[derive(Debug, Clone)]
pub enum MailConfig {
    /// Local Mailpit, which catches everything.
    Mailpit {
        /// e.g. `http://localhost:8025`.
        base_url: String,
    },
    /// Resend, in production.
    Resend {
        /// Normally `https://api.resend.com`.
        base_url: String,
        /// The API key.
        api_key: SecretString,
    },
}

/// Every setting.
#[derive(Debug, Clone)]
pub struct Config {
    /// Postgres connection string.
    pub database_url: String,
    /// Where email goes.
    pub mail: MailConfig,
    /// The address reminders come from, on Balanse's domain.
    pub mail_from_address: String,
    /// The domain of reminders' reply addresses, `CODE@domain`.
    pub reply_domain: String,
    /// The app's address, as `scheme://host[:port]`, for links in notices.
    pub public_base_url: String,
    /// Signs reply addresses (and, later, the Debtor's page links).
    pub master_key_hex: SecretString,
    /// Where the web server listens, e.g. `127.0.0.1:3000`.
    pub bind_addr: String,
    /// Proves inbound email comes from the provider; without it, replies
    /// aren't taken in.
    pub inbound_secret: Option<SecretString>,
    /// Proves delivery events come from Resend (`whsec_…`); without it,
    /// bounces and complaints aren't taken in.
    pub resend_webhook_secret: Option<SecretString>,
    /// The built React app (`WEB_DIR`, default `web/dist`).
    pub web_dir: std::path::PathBuf,
    /// Where encrypted proof files are kept (`UPLOAD_DIR`, default `uploads`).
    pub upload_dir: std::path::PathBuf,
    /// ClamAV's daemon (`CLAMD_ADDR`, `host:port`), which scans every proof
    /// file; required in production.
    pub clamd_addr: Option<String>,
    /// The model that writes reminders' wording; `None` without
    /// `ANTHROPIC_API_KEY`, and every reminder uses the template (E2).
    pub model: Option<ModelConfig>,
}

/// Where and which model writes wording.
#[derive(Debug, Clone)]
pub struct ModelConfig {
    /// Normally `https://api.anthropic.com`.
    pub base_url: String,
    /// The API key: never a Claude subscription (decision of 2026-10-22).
    pub api_key: SecretString,
    /// The model ID, e.g. `claude-sonnet-5-5`.
    pub model: String,
}

/// A setting that is missing or can't be used.
#[derive(Debug, thiserror::Error)]
#[error("{name}: {reason}")]
pub struct ConfigError {
    /// The variable.
    pub name: &'static str,
    /// What's wrong with it.
    pub reason: String,
}

impl Config {
    /// Reads every setting from the environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        let mail = match optional("MAIL_PROVIDER").as_deref().unwrap_or("mailpit") {
            "mailpit" => MailConfig::Mailpit {
                base_url: optional("MAILPIT_URL").unwrap_or_else(|| "http://localhost:8025".to_owned()),
            },
            "resend" => MailConfig::Resend {
                base_url: optional("RESEND_BASE_URL").unwrap_or_else(|| "https://api.resend.com".to_owned()),
                api_key: SecretString::from(required("RESEND_API_KEY")?),
            },
            other => {
                return Err(ConfigError {
                    name: "MAIL_PROVIDER",
                    reason: format!("{other:?} is not one of: mailpit, resend"),
                });
            }
        };
        let config = Self {
            database_url: required("DATABASE_URL")?,
            mail,
            mail_from_address: optional("MAIL_FROM_ADDRESS").unwrap_or_else(|| "reminders@balanseapp.test".to_owned()),
            reply_domain: optional("REPLY_DOMAIN").unwrap_or_else(|| "reply.balanseapp.test".to_owned()),
            public_base_url: optional("PUBLIC_BASE_URL").unwrap_or_else(|| "http://127.0.0.1:3000".to_owned()),
            master_key_hex: SecretString::from(required("MASTER_KEY")?),
            bind_addr: optional("BIND_ADDR").unwrap_or_else(|| "127.0.0.1:3000".to_owned()),
            inbound_secret: optional("INBOUND_SECRET").map(SecretString::from),
            resend_webhook_secret: optional("RESEND_WEBHOOK_SECRET").map(SecretString::from),
            web_dir: optional("WEB_DIR").map_or_else(|| "web/dist".into(), Into::into),
            upload_dir: optional("UPLOAD_DIR").map_or_else(|| "uploads".into(), Into::into),
            clamd_addr: optional("CLAMD_ADDR"),
            model: anthropic_key(optional).map(|key| ModelConfig {
                base_url: optional("ANTHROPIC_BASE_URL").unwrap_or_else(|| "https://api.anthropic.com".to_owned()),
                api_key: SecretString::from(key),
                model: drafting_model(optional("LLM_MODEL")),
            }),
        };
        config.master_key()?;
        Ok(config)
    }

    /// The master key that signs reply addresses.
    pub fn master_key(&self) -> Result<MasterKey, ConfigError> {
        use secrecy::ExposeSecret;
        MasterKey::from_hex(self.master_key_hex.expose_secret()).map_err(|err| ConfigError {
            name: "MASTER_KEY",
            reason: err.to_string(),
        })
    }

    /// The mailer for the configured provider.
    pub fn mailer(&self) -> Result<ConfiguredMailer, ConfigError> {
        let invalid = |err: reqwest::Error| ConfigError {
            name: "MAIL_PROVIDER",
            reason: err.to_string(),
        };
        Ok(match &self.mail {
            MailConfig::Mailpit { base_url } => {
                ConfiguredMailer::Mailpit(MailpitMailer::new(base_url, &self.mail_from_address).map_err(invalid)?)
            }
            MailConfig::Resend { base_url, api_key } => ConfiguredMailer::Resend(
                ResendMailer::new(base_url, api_key.clone(), &self.mail_from_address).map_err(invalid)?,
            ),
        })
    }

    /// The client for the wording model, if one is configured. It gives up
    /// after 15 seconds, and the reminder uses the template (E2).
    pub fn language_model(&self) -> Result<Option<AnthropicClient>, ConfigError> {
        self.model
            .as_ref()
            .map(|model| {
                AnthropicClient::new(
                    &model.base_url,
                    model.api_key.clone(),
                    &model.model,
                    std::time::Duration::from_secs(15),
                )
                .map_err(|err| ConfigError {
                    name: "ANTHROPIC_BASE_URL",
                    reason: err.to_string(),
                })
            })
            .transpose()
    }

    /// The daily send budget: Balanse's share of Resend Free, or none for
    /// Mailpit.
    pub fn send_budget(&self) -> Option<SendBudget> {
        match self.mail {
            MailConfig::Mailpit { .. } => None,
            MailConfig::Resend { .. } => Some(RESEND_FREE_SHARE),
        }
    }
}

fn optional(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.trim().is_empty())
}

/// The model that words reminders: `LLM_MODEL`, or Claude Sonnet, which the
/// blind Taglish test of 2026-11-09 chose over Haiku (docs/decisions.md).
fn drafting_model(setting: Option<String>) -> String {
    setting.unwrap_or_else(|| "claude-sonnet-5-5".to_owned())
}

/// The owner's API key: `ANTHROPIC_API_KEY`, or in a cloud session
/// `OWNER_ANTHROPIC_API_KEY`, because Claude Code itself reads
/// `ANTHROPIC_API_KEY` and the owner's key must pay for this app only.
fn anthropic_key(lookup: impl Fn(&str) -> Option<String>) -> Option<String> {
    lookup("ANTHROPIC_API_KEY").or_else(|| lookup("OWNER_ANTHROPIC_API_KEY"))
}

fn required(name: &'static str) -> Result<String, ConfigError> {
    optional(name).ok_or(ConfigError {
        name,
        reason: "is not set".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e1_reminders_are_worded_by_sonnet_unless_llm_model_says_otherwise() {
        // The blind Taglish test of 2026-11-09 chose Sonnet (docs/decisions.md).
        assert_eq!(drafting_model(None), "claude-sonnet-5-5");
        assert_eq!(drafting_model(Some("claude-haiku-5-5".to_owned())), "claude-haiku-5-5");
    }

    #[test]
    fn the_owners_key_is_read_under_its_cloud_name_when_anthropic_api_key_is_unset() {
        let only = |set: &'static str| move |name: &str| (name == set).then(|| format!("{set}-value"));
        assert_eq!(
            anthropic_key(only("OWNER_ANTHROPIC_API_KEY")).as_deref(),
            Some("OWNER_ANTHROPIC_API_KEY-value")
        );
        assert_eq!(
            anthropic_key(|name: &str| Some(format!("{name}-value"))).as_deref(),
            Some("ANTHROPIC_API_KEY-value")
        );
        assert_eq!(anthropic_key(|_: &str| None), None);
    }
}
