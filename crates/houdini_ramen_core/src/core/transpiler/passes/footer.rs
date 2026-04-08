use crate::core::transpiler::builder::PythonBuilder;
use std::collections::BTreeSet;

pub fn write_footer(
    builder: &mut PythonBuilder,
    display_var: Option<&str>,
    nested_display_vars: &[&str],
    container_exprs: &BTreeSet<String>,
) {
    builder.empty_line();
    builder.line("# --- Layout & Display Pass ---");
    builder.line("parent.layoutChildren()");
    for expr in container_exprs {
        if expr != "parent" {
            builder.line(&format!("{}.layoutChildren()", expr));
        }
    }

    if let Some(var_name) = display_var {
        builder.empty_line();
        builder.line(&format!("{}.setDisplayFlag(True)", var_name));
        builder.line(&format!("{}.setRenderFlag(True)", var_name));
    }

    for var_name in nested_display_vars {
        builder.empty_line();
        builder.line(&format!("{}.setDisplayFlag(True)", var_name));
        builder.line(&format!("{}.setRenderFlag(True)", var_name));
    }
}
