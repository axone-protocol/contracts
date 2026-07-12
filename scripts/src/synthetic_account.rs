//! Helpers for creating uniquely named synthetic Abstract Accounts.

use abstract_client::{AbstractClient, Account};
use cw_orch::{anyhow, daemon::Daemon};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_NAME_PREFIX: &str = "axone-testnet-identity";
pub const DEFAULT_DESCRIPTION: &str =
    "Synthetic AXONE testnet identity generated for traffic stimulation.";
pub const DEFAULT_LINK: &str = "https://axone.xyz/testnet";
const MAX_ABSTRACT_NAME_LENGTH: usize = 64;

pub fn create(
    client: &AbstractClient<Daemon>,
    name_prefix: &str,
    marker: &str,
    description: &str,
    link: &str,
) -> anyhow::Result<Account<Daemon>> {
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
    if let Ok(run_id) = std::env::var("GITHUB_RUN_ID") {
        let attempt = std::env::var("GITHUB_RUN_ATTEMPT").unwrap_or_else(|_| "1".to_string());
        return format!("gh{run_id}a{attempt}");
    }

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("local{nanos}")
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
}
