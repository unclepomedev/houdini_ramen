use crate::core::py_escape::escape_py_key;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::ContainerType;

pub fn write_header(
    builder: &mut PythonBuilder,
    parent_path: &str,
    auto_create_type: Option<ContainerType>,
    auto_clear: bool,
) {
    let safe_path = escape_py_key(parent_path);
    builder.line("import hou");
    builder.line(&format!("parent_path = '{}'", safe_path));
    builder.line("parent = hou.node(parent_path)");

    if let Some(ctype) = auto_create_type {
        let safe_ctype = escape_py_key(ctype.as_str());
        builder.line("if not parent:");
        builder.indent();
        builder.line("parts = [p for p in parent_path.split('/') if p]");
        builder.line("curr = hou.node('/')");
        builder.line("curr_path = ''");
        builder.line("for i, part in enumerate(parts):");
        builder.indent();
        builder.line("curr_path += '/' + part");
        builder.line("child = hou.node(curr_path)");
        builder.line("if not child:");
        builder.indent();
        builder.line(&format!(
            "n_type = '{}' if i == len(parts) - 1 else 'subnet'",
            safe_ctype
        ));
        builder.line("curr = curr.createNode(n_type, part)");
        builder.dedent();
        builder.line("else:");
        builder.indent();
        builder.line("curr = child");
        builder.dedent();
        builder.dedent();
        builder.line("parent = curr");
        builder.dedent();
    }

    builder.line("if not parent:");
    builder.indent();
    builder.line("raise RuntimeError(f\"Parent node '{parent_path}' not found\")");
    builder.dedent();

    if auto_clear {
        builder.line("for child in parent.children():");
        builder.indent();
        builder.line("child.destroy()");
        builder.dedent();
    }

    builder.empty_line();
    builder.line("# --- Helper: Dive Target Resolution ---");
    builder.line("def _get_insert_target(container):");
    builder.indent();
    builder.line("hda_def = container.type().definition()");
    builder.line("if hda_def and 'DiveTarget' in hda_def.sections():");
    builder.indent();
    builder.line("dive_path = hda_def.sections()['DiveTarget'].contents().strip()");
    builder.line("if dive_path:");
    builder.indent();
    builder.line("target = container.node(dive_path)");
    builder.line("if target:");
    builder.indent();
    builder.line("return target");
    builder.dedent();
    builder.dedent();
    builder.dedent();
    builder.line("return container");
    builder.dedent();
    builder.empty_line();
}
