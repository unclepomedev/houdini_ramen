pub fn python_string_literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' => out.push_str(r"\\"),
            '"' => out.push_str(r#"\""#),
            '\n' => out.push_str(r"\n"),
            '\r' => out.push_str(r"\r"),
            '\t' => out.push_str(r"\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub fn escape_py_key(s: &str) -> String {
    s.replace('\\', r"\\").replace('\'', r"\'")
}

/// Sanitizes a string to be part of a valid Python identifier.
/// Note: This replaces invalid characters with underscores.
/// It does NOT prepend an underscore if the string starts with a digit.
/// Callers are expected to add a valid prefix (e.g., "n_") to the result.
pub fn sanitize_py_ident(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_string_literal() {
        assert_eq!(python_string_literal("hello"), "\"hello\"");
        assert_eq!(
            python_string_literal("hello \"world\""),
            "\"hello \\\"world\\\"\""
        );
        assert_eq!(
            python_string_literal("C:\\path\\to\\file"),
            "\"C:\\\\path\\\\to\\\\file\""
        );
        assert_eq!(python_string_literal("line1\nline2"), "\"line1\\nline2\"");
        assert_eq!(python_string_literal("tab\tspace"), "\"tab\\tspace\"");
    }

    #[test]
    fn test_escape_py_key() {
        assert_eq!(escape_py_key("normal_key"), "normal_key");
        assert_eq!(escape_py_key("key'with'quotes"), "key\\'with\\'quotes");
        assert_eq!(escape_py_key("key\\with\\slash"), "key\\\\with\\\\slash");
    }

    #[test]
    fn test_sanitize_py_ident() {
        assert_eq!(sanitize_py_ident("normal_name123"), "normal_name123");
        assert_eq!(sanitize_py_ident("my-box.1"), "my_box_1");
        assert_eq!(sanitize_py_ident("my color"), "my_color");
        assert_eq!(sanitize_py_ident("!@#invalid_ident"), "___invalid_ident");
        assert_eq!(sanitize_py_ident("123node"), "123node");
    }
}
