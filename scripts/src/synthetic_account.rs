//! Helpers for creating uniquely named synthetic Abstract Accounts.

use abstract_client::{AbstractClient, Account};
use cw_orch::{anyhow, environment::CwEnv};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_NAME_PREFIX: &str = "axone-testnet-identity";
pub const DEFAULT_DESCRIPTION: &str =
    "Synthetic AXONE testnet identity generated for traffic stimulation.";
pub const DEFAULT_LINK: &str = "https://axone.xyz/testnet";
const MAX_ABSTRACT_NAME_LENGTH: usize = 64;

pub fn create<Chain: CwEnv>(
    client: &AbstractClient<Chain>,
    name_prefix: &str,
    marker: &str,
    description: &str,
    link: &str,
) -> anyhow::Result<Account<Chain>> {
    client
        .account_builder()
        .name(account_name(name_prefix, marker))
        .description(description.to_string())
        .link(link.to_string())
        .build()
        .map_err(Into::into)
}

pub fn account_name(prefix: &str, marker: &str) -> String {
    let suffix = format!("-{marker}");
    let max_prefix_len = MAX_ABSTRACT_NAME_LENGTH.saturating_sub(suffix.len());
    format!("{}{}", truncate_utf8(prefix, max_prefix_len), suffix)
}

pub fn default_marker() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    marker(
        std::env::var("GITHUB_RUN_ID").ok().as_deref(),
        std::env::var("GITHUB_RUN_ATTEMPT").ok().as_deref(),
        nanos,
    )
}

fn marker(run_id: Option<&str>, attempt: Option<&str>, nanos: u128) -> String {
    match run_id {
        Some(run_id) => format!("gh{run_id}a{}", attempt.unwrap_or("1")),
        None => format!("local{nanos}"),
    }
}

fn truncate_utf8(value: &str, max_len: usize) -> String {
    let mut result = String::with_capacity(max_len);
    for ch in value.chars() {
        if result.len() + ch.len_utf8() > max_len {
            break;
        }
        result.push(ch);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use cw_orch::prelude::*;

    #[test]
    fn account_name_includes_marker() {
        assert_eq!(
            account_name("axone-testnet-identity", "gh123a1-2"),
            "axone-testnet-identity-gh123a1-2"
        );
    }

    #[test]
    fn account_name_is_capped_to_abstract_limit_with_multibyte_prefix() {
        let name = account_name("identity-🧪🧪🧪🧪🧪🧪🧪🧪🧪🧪", "gh123456789a1-50");

        assert!(name.len() <= MAX_ABSTRACT_NAME_LENGTH);
        assert!(name.ends_with("-gh123456789a1-50"));
        assert!(name.is_char_boundary(name.len()));
    }

    #[test]
    fn marker_uses_github_run_and_attempt_when_available() {
        assert_eq!(marker(Some("123"), Some("4"), 0), "gh123a4");
        assert_eq!(marker(Some("123"), None, 0), "gh123a1");
    }

    #[test]
    fn marker_uses_local_timestamp_without_github_run() {
        assert_eq!(marker(None, None, 42), "local42");
    }

    #[test]
    fn default_marker_uses_a_supported_prefix() {
        let marker = default_marker();

        assert!(marker.starts_with("gh") || marker.starts_with("local"));
    }

    #[test]
    fn creates_a_synthetic_account() -> anyhow::Result<()> {
        let chain = MockBech32::new("mock");
        let client = AbstractClient::builder(chain).build()?;

        let account = create(
            &client,
            DEFAULT_NAME_PREFIX,
            "test-1",
            DEFAULT_DESCRIPTION,
            DEFAULT_LINK,
        )?;

        assert!(account.address()?.as_str().starts_with("mock"));
        Ok(())
    }
}
