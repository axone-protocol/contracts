# Test Style

Use these examples when the task needs concrete guidance for naming or data-driven test structure.

## Test Naming Convention

Prefer descriptive names without a `test_` prefix:

```rust
#[test]
fn should_increment_counter_successfully() { }

#[test]
fn unauthorized_user_cannot_execute_admin_action() { }

#[test]
fn canonical_cosmos_address_conversion() { }

#[test]
fn invalid_bech32_address() { }
```

Avoid weak or generic naming:

```rust
#[test]
fn test_increment() { }

#[test]
fn test_unauthorized() { }
```

## Data-Driven Tests

Use tuples for simple test cases:

```rust
#[test]
fn canonical_cosmos_address_conversion() {
    let cases = vec![
        ("axone", "axone-localnet-1"),
        ("cosmos", "cosmoshub-4"),
        ("osmo", "osmosis-1"),
    ];

    for (input_hrp, chain_id) in cases {
        let result = convert_address(input_hrp, chain_id);
        assert!(result.is_ok());
        assert!(result.unwrap().contains(chain_id));
    }
}
```

Use structs only when cases have more than three fields or when field names add clarity:

```rust
struct TestCase {
    input: &'static str,
    expected_success: bool,
    config: Option<&'static str>,
    should_retry: bool,
    timeout_ms: u64,
}

#[test]
fn complex_scenario_with_multiple_parameters() {
    let cases = vec![
        TestCase {
            input: "valid",
            expected_success: true,
            config: Some("default"),
            should_retry: false,
            timeout_ms: 1000,
        },
        TestCase {
            input: "invalid",
            expected_success: false,
            config: None,
            should_retry: true,
            timeout_ms: 5000,
        },
    ];

    for case in cases {
        let result = process_with_config(case.input, case.config, case.timeout_ms);
        assert_eq!(result.is_ok(), case.expected_success);
        if !case.expected_success && case.should_retry {
            assert!(result.unwrap_err().is_retryable());
        }
    }
}
```
