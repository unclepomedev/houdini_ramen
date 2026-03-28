use crate::core::transpiler::builder::PythonBuilder;

pub fn write_footer(builder: &mut PythonBuilder) {
    builder.empty_line();
    builder.line("# --- Layout Pass ---");
    builder.line("parent.layoutChildren()");
}
