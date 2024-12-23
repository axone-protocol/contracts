/// Convert a Rust string to a Prolog atom.
pub fn as_prolog_atom(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len() + 2);
    escaped.push('\'');
    for c in s.chars() {
        if c == '\'' {
            escaped.push('\\');
            escaped.push(c);
        } else {
            escaped.push(c);
        }
    }
    escaped.push('\'');

    escaped
}

#[cfg(test)]
mod tests {
    use super::as_prolog_atom;

    #[test]
    fn test_as_prolog_atom() {
        let test_cases = vec![
            ("empty string", "", "''"),
            ("simple case", "hello", "'hello'"),
            ("space in the string", "hello world", "'hello world'"),
            ("single quote in the middle", "foo'bar", "'foo\\'bar'"),
            ("enclosed single quotes", "'foo bar'", "'\\'foo bar\\''"),
            ("cosmwasm URI", "cosmwasm:name:address?query=%7B%22object_data%22%3A%7B%22id%22%3A%221a88ca1632c7323c0aa594000cda26ed9f48b36351c29c3d1e35e0a0474e862e%22%7D%7D", "'cosmwasm:name:address?query=%7B%22object_data%22%3A%7B%22id%22%3A%221a88ca1632c7323c0aa594000cda26ed9f48b36351c29c3d1e35e0a0474e862e%22%7D%7D'")
        ];

        for (_, input, expected) in test_cases {
            let actual = as_prolog_atom(input);
            assert_eq!(
                actual, expected,
                "as_prolog_atom({:?}) should produce {:?}, but got {:?}",
                input, expected, actual
            );
        }
    }
}
