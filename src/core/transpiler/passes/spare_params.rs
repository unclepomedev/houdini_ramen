use crate::core::py_escape::python_string_literal;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::{HoudiniNode, SpareParam};
use std::collections::HashMap;

pub fn write_spare_parameter_pass(
    builder: &mut PythonBuilder,
    nodes: &[Box<dyn HoudiniNode>],
    id_to_var: &HashMap<usize, String>,
) -> Result<(), String> {
    builder.empty_line();
    builder.line("# --- Spare Parameter Pass ---");
    for node in nodes {
        let spares = node.get_spare_params();
        if spares.is_empty() {
            continue;
        }

        let var_name = id_to_var
            .get(&node.get_id())
            .ok_or_else(|| format!("missing variable mapping for node id {}", node.get_id()))?;

        builder.line(&format!("ptg = {}.parmTemplateGroup()", var_name));

        for spare in spares {
            write_single_spare(builder, spare);
        }

        builder.line(&format!("{}.setParmTemplateGroup(ptg)", var_name));
    }
    Ok(())
}

fn write_single_spare(builder: &mut PythonBuilder, spare: &SpareParam) {
    match spare {
        SpareParam::Float {
            name,
            label,
            default,
            min,
            max,
        } => {
            builder.line(&format!(
                "pt = hou.FloatParmTemplate({}, {}, 1, default_value=({:?},), min={:?}, max={:?})",
                python_string_literal(name),
                python_string_literal(label),
                default,
                min,
                max
            ));
        }
        SpareParam::Int {
            name,
            label,
            default,
            min,
            max,
        } => {
            builder.line(&format!(
                "pt = hou.IntParmTemplate({}, {}, 1, default_value=({},), min={}, max={})",
                python_string_literal(name),
                python_string_literal(label),
                default,
                min,
                max
            ));
        }
        SpareParam::String {
            name,
            label,
            default,
        } => {
            builder.line(&format!(
                "pt = hou.StringParmTemplate({}, {}, 1, default_value=({},))",
                python_string_literal(name),
                python_string_literal(label),
                python_string_literal(default)
            ));
        }
        SpareParam::Toggle {
            name,
            label,
            default,
        } => {
            let py_bool = if *default { "True" } else { "False" };
            builder.line(&format!(
                "pt = hou.ToggleParmTemplate({}, {}, default_value={})",
                python_string_literal(name),
                python_string_literal(label),
                py_bool
            ));
        }
        SpareParam::Color {
            name,
            label,
            default,
        } => {
            builder.line(&format!(
                "pt = hou.FloatParmTemplate({}, {}, 3, default_value=({:.4}, {:.4}, {:.4}), look=hou.parmLook.ColorSquare)",
                python_string_literal(name), python_string_literal(label), default[0], default[1], default[2]
            ));
        }
        SpareParam::Button { name, label } => {
            builder.line(&format!(
                "pt = hou.ButtonParmTemplate({}, {})",
                python_string_literal(name),
                python_string_literal(label)
            ));
        }
        SpareParam::Menu {
            name,
            label,
            default,
            menu_items,
        } => {
            if menu_items.is_empty() {
                // Houdini menus require at least one item, but fallback safely for valid Python syntax
                builder.line(&format!(
                    "pt = hou.MenuParmTemplate({}, {}, menu_items=(), menu_labels=(), default_value=0)",
                    python_string_literal(name), python_string_literal(label)
                ));
            } else {
                let keys: Vec<String> = menu_items
                    .iter()
                    .map(|(k, _)| python_string_literal(k))
                    .collect();
                let labels: Vec<String> = menu_items
                    .iter()
                    .map(|(_, v)| python_string_literal(v))
                    .collect();
                builder.line(&format!(
                    "pt = hou.MenuParmTemplate({}, {}, menu_items=({},), menu_labels=({},), default_value={})",
                    python_string_literal(name), python_string_literal(label),
                    keys.join(", "), labels.join(", "), default
                ));
            }
        }
        SpareParam::File {
            name,
            label,
            default,
        } => {
            builder.line(&format!(
                "pt = hou.StringParmTemplate({}, {}, 1, default_value=({},), string_type=hou.stringParmType.FileReference)",
                python_string_literal(name), python_string_literal(label), python_string_literal(default)
            ));
        }
        SpareParam::NodePath {
            name,
            label,
            default,
        } => {
            builder.line(&format!(
                "pt = hou.StringParmTemplate({}, {}, 1, default_value=({},), string_type=hou.stringParmType.NodeReference)",
                python_string_literal(name), python_string_literal(label), python_string_literal(default)
            ));
        }
        SpareParam::RampFloat { name, label } => {
            builder.line(&format!(
                "pt = hou.RampParmTemplate({}, {}, hou.rampParmType.Float)",
                python_string_literal(name),
                python_string_literal(label)
            ));
        }
        SpareParam::RampColor { name, label } => {
            builder.line(&format!(
                "pt = hou.RampParmTemplate({}, {}, hou.rampParmType.Color)",
                python_string_literal(name),
                python_string_literal(label)
            ));
        }
    }
    builder.line("ptg.append(pt)");
}
