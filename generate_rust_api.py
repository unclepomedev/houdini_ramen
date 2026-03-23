import argparse
import json
import logging
import re
import sys
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path

from jinja2 import Environment, FileSystemLoader

logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)

RUST_KEYWORDS = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
    "gen",
}


@dataclass(frozen=True)
class ParsedParam:
    name: str
    method_suffix: str
    r_type: str
    enum_variant: str
    val_converter: str
    is_multiparm: bool
    fn_args: str
    format_args: str


@dataclass(frozen=True)
class ParsedNode:
    struct_name: str
    params: list[ParsedParam] = field(default_factory=list)


def clean_identifier(s: str) -> str:
    s = re.sub(r"[^a-zA-Z0-9]", "_", s)
    s = re.sub(r"_+", "_", s)
    return s.strip("_")


def to_safe_ident(name: str) -> str:
    return f"r#{name}" if name in RUST_KEYWORDS else name


def snake_case(s: str) -> str:
    name = clean_identifier(s).lower()
    if not name:
        return "unknown"
    if name[0].isdigit():
        name = f"n_{name}"
    return name


def pascal_case(s: str) -> str:
    name = "".join(word.capitalize() for word in clean_identifier(s).split("_"))
    if not name:
        return "Unknown"
    if name[0].isdigit():
        return f"N{name}"
    return name


def get_rust_type_info(h_type: str, default_val) -> tuple[str, str, str]:
    if h_type == "Toggle":
        return "bool", "Toggle", "val"
    if h_type == "String":
        return "&str", "String", "val.to_string()"
    if h_type == "Menu":
        return "i32", "Menu", "val"
    if h_type == "Button":
        return "()", "Button", ""
    is_array = isinstance(default_val, list) and len(default_val) > 1
    if h_type == "Int":
        return ("Vec<i32>", "IntArray", "val") if is_array else ("i32", "Int", "val")
    if h_type in ("Float", "Angle"):
        return (
            ("Vec<f32>", "FloatArray", "val") if is_array else ("f32", "Float", "val")
        )
    return "&str", "String", "val.to_string()"


def get_multiparm_args(count: int) -> tuple[str, str]:
    if count == 0:
        return "", ""
    fn_args = ", ".join(f"index{i + 1}: usize" for i in range(count))
    format_args = ", ".join(f"index{i + 1}" for i in range(count))
    return fn_args, format_args


def resolve_unique_suffix(base_suffix: str, seen_suffixes: set) -> str:
    method_suffix = base_suffix
    suffix_counter = 1
    while method_suffix in seen_suffixes:
        method_suffix = f"{base_suffix}_{suffix_counter}"
        suffix_counter += 1
    seen_suffixes.add(method_suffix)
    return method_suffix


def parse_single_param(p: dict[str, object], seen_suffixes: set) -> ParsedParam | None:
    p_name = p.get("name")
    if not p_name:
        return None

    multiparm_count = p_name.count("#")
    is_multiparm = multiparm_count > 0
    fn_args, format_args = get_multiparm_args(multiparm_count)

    r_type, enum_variant, val_converter = get_rust_type_info(
        p.get("type", ""), p.get("default")
    )

    base_suffix = snake_case(p_name.replace("#", ""))
    method_suffix = resolve_unique_suffix(
        f"{base_suffix}_inst" if is_multiparm else base_suffix, seen_suffixes
    )

    return ParsedParam(
        p_name,
        method_suffix,
        r_type,
        enum_variant,
        val_converter,
        is_multiparm,
        fn_args,
        format_args,
    )


def parse_node_data(struct_name: str, node_info: dict[str, object]) -> ParsedNode:
    params = []
    seen_suffixes = set()

    for p in node_info.get("parms", []):
        parsed = parse_single_param(p, seen_suffixes)
        if parsed:
            params.append(parsed)

    return ParsedNode(struct_name, params)


def group_nodes_by_prefix(
    nodes: dict[str, object],
) -> dict[str, list[tuple[str, object]]]:
    groups = defaultdict(list)
    for node_name, node_info in nodes.items():
        key = node_name[0].lower() if node_name and node_name[0].isalpha() else "_"
        groups[key].append((node_name, node_info))
    return groups


def write_category_files(
    cat_name: str,
    nodes: dict[str, object],
    rs_root: Path,
    stub_root: Path,
    template_rs,
    template_stub,
) -> str | None:
    cat_snake = snake_case(cat_name)
    if not cat_snake:
        logger.warning(f"Skipping category with invalid name: {cat_name!r}")
        return None
    cat_pascal = pascal_case(cat_name)

    cat_rs_dir = rs_root / cat_snake
    cat_stub_dir = stub_root / cat_snake
    cat_rs_dir.mkdir(parents=True, exist_ok=True)
    cat_stub_dir.mkdir(parents=True, exist_ok=True)

    groups = group_nodes_by_prefix(nodes)

    cat_mod_lines = []
    for key, group_nodes in groups.items():
        cat_mod_lines.append(f"pub mod {to_safe_ident(key)};")

        rs_blocks = []
        stub_blocks = []

        for node_name, node_info in group_nodes:
            parsed = parse_node_data(f"{cat_pascal}{pascal_case(node_name)}", node_info)
            rs_blocks.append(template_rs.render(node=parsed))
            stub_blocks.append(template_stub.render(node=parsed))

        (cat_rs_dir / f"{key}.rs").write_text("\n\n".join(rs_blocks), encoding="utf-8")
        (cat_stub_dir / f"{key}.stub").write_text(
            "\n\n".join(stub_blocks), encoding="utf-8"
        )

    (cat_rs_dir / "mod.rs").write_text("\n".join(cat_mod_lines), encoding="utf-8")
    return f"pub mod {to_safe_ident(cat_snake)};"


def generate_files(input_json: Path, rs_root: Path, stub_root: Path) -> None:
    env = Environment(
        loader=FileSystemLoader("templates"), trim_blocks=True, lstrip_blocks=True
    )
    template_rs = env.get_template("node.rs.j2")
    template_stub = env.get_template("node.stub.j2")

    try:
        data = json.loads(input_json.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError) as e:
        logger.error(f"Failed to load JSON: {e}")
        sys.exit(1)

    main_rs_mods = []
    for cat_name, nodes in data.items():
        mod_decl = write_category_files(
            cat_name, nodes, rs_root, stub_root, template_rs, template_stub
        )
        if mod_decl:
            main_rs_mods.append(mod_decl)
    (rs_root / "mod.rs").write_text("\n".join(main_rs_mods), encoding="utf-8")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_json", type=Path)
    parser.add_argument("rs_out_dir", type=Path)
    parser.add_argument("stub_out_dir", type=Path)
    args = parser.parse_args()

    logger.info(f"Generating Rust API into {args.rs_out_dir}...")
    generate_files(args.input_json, args.rs_out_dir, args.stub_out_dir)
    logger.info("Generation complete.")


if __name__ == "__main__":
    main()
