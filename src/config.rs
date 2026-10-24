//! Settings, read once from the environment (and `.env` in development).

use secrecy::SecretString;

use crate::engine::files::MasterKey;
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
    /// The domain of reminders' reply addresses, `reply+CODE@domain`.
    pub reply_domain: String,
    /// The app's address, as `scheme://host[:port]`, for links in notices.
    pub public_base_url: String,
    /// Signs reply addresses (and, later, the Debtor's page links).
    pub master_key_hex: SecretString,
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

fn required(name: &'static str) -> Result<String, ConfigError> {
    optional(name).ok_or(ConfigError {
        name,
        reason: "is not set".to_owned(),
    })
}
