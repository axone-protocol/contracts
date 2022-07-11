fn main() {
    hello_world(&mut std::io::stdout());
}

fn hello_world(mut writer: impl std::io::Write) {
    let _ = writeln!(writer, "Hello, world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_hello_world() {
        let mut result = Vec::new();
        hello_world(&mut result);

        assert_eq!(result, b"Hello, world\n");
    }
}
