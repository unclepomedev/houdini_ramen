import argparse
import json
import logging
import re
import sys
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

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
class RustTypeInfo:
    r_type: str
    enum_variant: str
    val_converter: str


@dataclass(frozen=True)
class MultiparmInfo:
    is_multiparm: bool
    fn_args: str
    format_args: str


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
    node_type: str
    params: list[ParsedParam] = field(default_factory=list)


class SuffixResolver:
    def __init__(self):
        self._seen = set()

    def resolve(self, base_suffix: str) -> str:
        suffix = base_suffix
        counter = 1
        while suffix in self._seen:
            suffix = f"{base_suffix}_{counter}"
            counter += 1
        self._seen.add(suffix)
        return suffix


def clean_identifier(s: str) -> str:
    return re.sub(r"_+", "_", re.sub(r"[^a-zA-Z0-9]", "_", s)).strip("_")


def to_safe_ident(name: str) -> str:
    return f"r#{name}" if name in RUST_KEYWORDS else name


def snake_case(s: str) -> str:
    name = clean_identifier(s).lower()
    if not name:
        return "unknown"
    return f"n_{name}" if name[0].isdigit() else name


def pascal_case(s: str) -> str:
    name = "".join(word.capitalize() for word in clean_identifier(s).split("_"))
    if not name:
        return "Unknown"
    return f"N{name}" if name[0].isdigit() else name


def resolve_rust_type(h_type: str, default_val: Any) -> RustTypeInfo | None:
    if h_type in ("Separator", "Label"):
        return None

    if h_type == "Toggle":
        return RustTypeInfo("bool", "Toggle", "val")
    if h_type in ("String", "Data"):
        return RustTypeInfo("&str", h_type, "val.to_string()")
    if h_type == "Menu":
        return RustTypeInfo("i32", "Menu", "val")
    if h_type == "Button":
        return RustTypeInfo("()", "Button", "")
    if h_type == "Ramp":
        return RustTypeInfo("Vec<crate::core::types::RampPoint>", "Ramp", "val")

    is_list = isinstance(default_val, list)
    length = len(default_val) if is_list else 1

    if h_type == "Int":
        if not is_list or length <= 1:
            return RustTypeInfo("i32", "Int", "val")
        if length == 2:
            return RustTypeInfo("[i32; 2]", "Int2", "val")
        if length == 3:
            return RustTypeInfo("[i32; 3]", "Int3", "val")
        if length == 4:
            return RustTypeInfo("[i32; 4]", "Int4", "val")
        return RustTypeInfo("Vec<i32>", "IntArray", "val")

    if h_type == "Float":
        if not is_list or length <= 1:
            return RustTypeInfo("f32", "Float", "val")
        if length == 2:
            return RustTypeInfo("[f32; 2]", "Float2", "val")
        if length == 3:
            return RustTypeInfo("[f32; 3]", "Float3", "val")
        if length == 4:
            return RustTypeInfo("[f32; 4]", "Float4", "val")
        return RustTypeInfo("Vec<f32>", "FloatArray", "val")

    raise ValueError(f"Unsupported Houdini parameter type: {h_type!r}")


def resolve_multiparm(name: str) -> MultiparmInfo:
    count = name.count("#")
    if count == 0:
        return MultiparmInfo(False, "", "")

    fn_args = ", ".join(f"index{i + 1}: usize" for i in range(count))
    format_args = ", ".join(f"index{i + 1}" for i in range(count))
    return MultiparmInfo(True, fn_args, format_args)


def parse_param(
    p_name: str, p_data: dict[str, Any], resolver: SuffixResolver
) -> ParsedParam | None:
    type_info = resolve_rust_type(p_data.get("type", ""), p_data.get("default"))

    if type_info is None:
        return None

    multi_info = resolve_multiparm(p_name)

    base_suffix = snake_case(p_name.replace("#", ""))
    target_suffix = f"{base_suffix}_inst" if multi_info.is_multiparm else base_suffix
    method_suffix = resolver.resolve(target_suffix)

    return ParsedParam(
        name=p_name,
        method_suffix=method_suffix,
        r_type=type_info.r_type,
        enum_variant=type_info.enum_variant,
        val_converter=type_info.val_converter,
        is_multiparm=multi_info.is_multiparm,
        fn_args=multi_info.fn_args,
        format_args=multi_info.format_args,
    )


def parse_node(
    struct_name: str, node_type: str, parms_data: list[dict[str, Any]]
) -> ParsedNode:
    resolver = SuffixResolver()
    params = []

    for p in parms_data:
        p_name = p.get("name")
        if not p_name:
            continue

        parsed = parse_param(p_name, p, resolver)
        if parsed is not None:
            params.append(parsed)

    return ParsedNode(struct_name, node_type, params)


class CodeGenerator:
    def __init__(self, rs_root: Path, stub_root: Path):
        self.rs_root = rs_root
        self.stub_root = stub_root
        template_dir = Path(__file__).parent.parent / "templates"
        env = Environment(
            loader=FileSystemLoader(str(template_dir)),
            trim_blocks=True,
            lstrip_blocks=True,
        )
        self.template_rs = env.get_template("node.rs.j2")
        self.template_stub = env.get_template("node.stub.j2")

    def write_module(
        self,
        cat_snake: str,
        key: str,
        nodes: list[tuple[str, dict[str, Any]]],
        cat_pascal: str,
    ):
        rs_blocks = []
        stub_blocks = []

        for node_name, node_info in nodes:
            struct_name = f"{cat_pascal}{pascal_case(node_name)}"
            parsed = parse_node(struct_name, node_name, node_info.get("parms", []))
            rs_blocks.append(self.template_rs.render(node=parsed))
            stub_blocks.append(self.template_stub.render(node=parsed))

        cat_rs_dir = self.rs_root / cat_snake
        cat_stub_dir = self.stub_root / cat_snake

        (cat_rs_dir / f"{key}.rs").write_text("\n\n".join(rs_blocks), encoding="utf-8")
        (cat_stub_dir / f"{key}.stub").write_text(
            "\n\n".join(stub_blocks), encoding="utf-8"
        )

    def process_category(self, cat_name: str, nodes: dict[str, Any]) -> str | None:
        cat_snake = snake_case(cat_name)
        if not cat_snake:
            logger.warning(f"Skipping category with invalid name: {cat_name!r}")
            return None

        cat_pascal = pascal_case(cat_name)
        (self.rs_root / cat_snake).mkdir(parents=True, exist_ok=True)
        (self.stub_root / cat_snake).mkdir(parents=True, exist_ok=True)

        groups = defaultdict(list)
        for node_name, node_info in nodes.items():
            key = (
                node_name[0].lower() if node_name and node_name[0].isalpha() else "misc"
            )
            groups[key].append((node_name, node_info))

        mod_lines = []
        for key, group_nodes in groups.items():
            mod_lines.append(f"pub mod {to_safe_ident(key)};")
            self.write_module(cat_snake, key, group_nodes, cat_pascal)

        (self.rs_root / cat_snake / "mod.rs").write_text(
            "\n".join(mod_lines), encoding="utf-8"
        )
        return f"pub mod {to_safe_ident(cat_snake)};"

    def generate_all(self, data: dict[str, Any]):
        main_mods = []
        for cat_name, nodes in data.items():
            mod_decl = self.process_category(cat_name, nodes)
            if mod_decl:
                main_mods.append(mod_decl)
        (self.rs_root / "mod.rs").write_text("\n".join(main_mods), encoding="utf-8")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_json", type=Path)
    parser.add_argument("rs_out_dir", type=Path)
    parser.add_argument("stub_out_dir", type=Path)
    args = parser.parse_args()

    try:
        data = json.loads(args.input_json.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError) as e:
        logger.error(f"Failed to load JSON: {e}")
        sys.exit(1)

    logger.info(f"Generating Rust API into {args.rs_out_dir}...")
    try:
        generator = CodeGenerator(args.rs_out_dir, args.stub_out_dir)
        generator.generate_all(data)
    except Exception as e:
        logger.error(f"Generation failed: {e}", exc_info=True)
        sys.exit(1)
    logger.info("Generation complete.")


if __name__ == "__main__":
    main()
