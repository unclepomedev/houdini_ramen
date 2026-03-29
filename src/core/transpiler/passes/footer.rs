use crate::core::transpiler::builder::PythonBuilder;

pub fn write_footer(builder: &mut PythonBuilder, display_var: Option<&str>) {
    builder.empty_line();
    builder.line("# --- Layout & Display Pass ---");
    builder.line("parent.layoutChildren()");

    if let Some(var_name) = display_var {
        builder.empty_line();
        builder.line(&format!("{}.setDisplayFlag(True)", var_name));
        builder.line(&format!("{}.setRenderFlag(True)", var_name));
    }
}
