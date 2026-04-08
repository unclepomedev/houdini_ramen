pub struct PythonBuilder {
    buffer: String,
    indent_level: usize,
}

impl Default for PythonBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonBuilder {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            indent_level: 0,
        }
    }

    pub fn line(&mut self, text: &str) {
        let indent_str = "    ".repeat(self.indent_level);
        self.buffer.push_str(&indent_str);
        self.buffer.push_str(text);
        self.buffer.push('\n');
    }

    pub fn empty_line(&mut self) {
        self.buffer.push('\n');
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    pub fn build(self) -> String {
        self.buffer.replace("\r\n", "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_builder() {
        let mut builder = PythonBuilder::new();
        builder.line("def hello():");
        builder.indent();
        builder.line("print('hello')");
        builder.empty_line();
        builder.line("if True:");
        builder.indent();
        builder.line("pass");
        builder.dedent();
        builder.dedent();
        builder.line("hello()");

        let expected = "def hello():\n    print('hello')\n\n    if True:\n        pass\nhello()\n";
        assert_eq!(builder.build(), expected);
    }
}
