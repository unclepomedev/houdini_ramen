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
class ParsedInput:
    index: int
    doc_label: str
    safe_name: str


@dataclass(frozen=True)
class ParsedMenuVariant:
    name: str
    value: int
    doc_label: str


@dataclass(frozen=True)
class ParsedMenuEnum:
    enum_name: str
    variants: list[ParsedMenuVariant]


@dataclass(frozen=True)
class ParsedInnerMethod:
    method_name: str
    rel_path: str


@dataclass(frozen=True)
class ParsedOutput:
    index: int
    raw_name: str
    safe_name: str


@dataclass(frozen=True)
class ParsedNode:
    struct_name: str
    node_type: str
    min_inputs: int
    max_inputs: int
    inputs: list[ParsedInput] = field(default_factory=list)
    outputs: list[ParsedOutput] = field(default_factory=list)
    params: list[ParsedParam] = field(default_factory=list)
    enums: list[ParsedMenuEnum] = field(default_factory=list)
    inner_methods: list[ParsedInnerMethod] = field(default_factory=list)
    dive_target: str | None = None


class SuffixResolver:
    def __init__(self, separator: str = "_"):
        self._seen = set()
        self.separator = separator

    def resolve(self, base_suffix: str) -> str:
        suffix = base_suffix
        counter = 1
        while suffix in self._seen:
            suffix = f"{base_suffix}{self.separator}{counter}"
            counter += 1
        self._seen.add(suffix)
        return suffix


def clean_identifier(s: str) -> str:
    return re.sub(r"_+", "_", re.sub(r"[^a-zA-Z0-9]", "_", s)).strip("_")


def to_safe_ident(name: str) -> str:
    if name in ("Self", "self", "super", "crate"):
        return f"{name}_"
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


def safe_rust_string(s: str) -> str:
    if not s:
        return ""
    return str(s).replace("\\", "\\\\").replace('"', '\\"')


def resolve_rust_type(h_type: str, default_val: Any) -> RustTypeInfo | None:
    # UI-specific parameters to ignore
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


def _build_menu_enum(
    p_name: str, p_data: dict[str, Any], struct_name: str, enum_resolver: SuffixResolver
) -> ParsedMenuEnum | None:
    if p_data.get("type") not in ("Menu", "Int"):
        return None
    if not p_data.get("menu_items") or not p_data.get("menu_labels"):
        return None

    base_enum_name = pascal_case(p_name)
    unique_enum_name = enum_resolver.resolve(base_enum_name)
    enum_name = f"{struct_name}{unique_enum_name}"

    variants = []
    variant_resolver = SuffixResolver(separator="")

    for i, (tok, lab) in enumerate(
        zip(p_data["menu_items"], p_data["menu_labels"], strict=True)
    ):
        raw_lab = str(lab)
        safe_doc_label = re.sub(r"\s+", " ", raw_lab).strip()
        clean_lab = re.sub(r"!\[[^]]*]", "", raw_lab)
        if not clean_lab.strip():
            clean_lab = tok

        clean_lab = clean_lab.replace("+", " Plus ").replace("-", " Minus ")
        clean_lab = re.sub(r"[^a-zA-Z0-9_\s]", " ", clean_lab)

        base_v_name = pascal_case(clean_lab)
        if base_v_name == "Unknown":
            base_v_name = pascal_case(tok)

        safe_v_name = to_safe_ident(base_v_name)
        v_name = variant_resolver.resolve(safe_v_name)
        variants.append(
            ParsedMenuVariant(name=v_name, value=i, doc_label=safe_doc_label)
        )

    return ParsedMenuEnum(enum_name=enum_name, variants=variants)


def parse_param(
    p_name: str,
    p_data: dict[str, Any],
    struct_name: str,
    resolver: SuffixResolver,
    enum_resolver: SuffixResolver,
) -> tuple[ParsedParam | None, ParsedMenuEnum | None]:
    type_info = resolve_rust_type(p_data.get("type", ""), p_data.get("default"))
    if type_info is None:
        return None, None

    multi_info = resolve_multiparm(p_name)
    base_suffix = snake_case(p_name.replace("#", ""))
    target_suffix = f"{base_suffix}_inst" if multi_info.is_multiparm else base_suffix
    method_suffix = resolver.resolve(target_suffix)

    menu_enum = _build_menu_enum(p_name, p_data, struct_name, enum_resolver)
    r_type = menu_enum.enum_name if menu_enum else type_info.r_type
    val_converter = "val as i32" if menu_enum else type_info.val_converter

    parsed_param = ParsedParam(
        name=safe_rust_string(p_name),
        method_suffix=method_suffix,
        r_type=r_type,
        enum_variant=type_info.enum_variant,
        val_converter=val_converter,
        is_multiparm=multi_info.is_multiparm,
        fn_args=multi_info.fn_args,
        format_args=multi_info.format_args,
    )
    return parsed_param, menu_enum


def _parse_inputs(raw_labels: list[str]) -> list[ParsedInput]:
    input_resolver = SuffixResolver()
    parsed = []
    for idx, label in enumerate(raw_labels):
        # remove line breaks and consecutive spaces to create one clean string
        clean_doc = re.sub(
            r"\s+", " ", label.replace("\n", " ").replace("\r", "")
        ).strip()

        # if it is too long, truncate it
        safe_base = snake_case(clean_doc) if clean_doc else f"input_{idx}"
        if len(safe_base) > 40:
            safe_base = safe_base[:40].rstrip("_")
        safe_name = to_safe_ident(input_resolver.resolve(safe_base))
        parsed.append(ParsedInput(index=idx, doc_label=clean_doc, safe_name=safe_name))
    return parsed


def _parse_outputs(raw_outputs: list[str]) -> list[ParsedOutput]:
    output_resolver = SuffixResolver()
    parsed = []
    for idx, raw_name in enumerate(raw_outputs):
        if not raw_name:
            continue
        safe_base = snake_case(raw_name)
        safe_name = to_safe_ident(output_resolver.resolve(safe_base))
        parsed.append(
            ParsedOutput(
                index=idx, raw_name=safe_rust_string(raw_name), safe_name=safe_name
            )
        )
    return parsed


def _parse_params(
    raw_params: list[dict[str, Any]], struct_name: str
) -> tuple[list[ParsedParam], list[ParsedMenuEnum]]:
    param_resolver = SuffixResolver()
    enum_resolver = SuffixResolver(separator="")
    params = []
    enums = []
    for p in raw_params:
        p_name = p.get("name")
        if not p_name:
            continue
        parsed_param, menu_enum = parse_param(
            p_name, p, struct_name, param_resolver, enum_resolver
        )
        if parsed_param is not None:
            params.append(parsed_param)
        if menu_enum is not None:
            enums.append(menu_enum)
    return params, enums


def _parse_inner_methods(raw_inner_nodes: dict[str, str]) -> list[ParsedInnerMethod]:
    inner_methods = []
    inner_method_resolver = SuffixResolver()
    for child_name, rel_path in sorted(raw_inner_nodes.items()):
        method_name = to_safe_ident(
            inner_method_resolver.resolve(snake_case(child_name))
        )
        inner_methods.append(
            ParsedInnerMethod(
                method_name=method_name, rel_path=safe_rust_string(rel_path)
            )
        )
    return inner_methods


def parse_node(
    struct_name: str, node_type: str, node_info: dict[str, Any]
) -> ParsedNode:
    params, enums = _parse_params(node_info.get("parms", []), struct_name)

    return ParsedNode(
        struct_name=struct_name,
        node_type=node_type,
        min_inputs=node_info.get("min_inputs", 0),
        max_inputs=node_info.get("max_inputs", 0),
        inputs=_parse_inputs(node_info.get("input_labels", [])),
        outputs=_parse_outputs(node_info.get("output_names", [])),
        params=params,
        enums=enums,
        inner_methods=_parse_inner_methods(node_info.get("builtin_inner_nodes", {})),
        dive_target=node_info.get("dive_target"),
    )


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

    @staticmethod
    def _group_nodes_by_key(nodes: dict[str, Any]) -> dict[str, list[tuple[str, Any]]]:
        groups = defaultdict(list)
        for node_name, node_info in nodes.items():
            key = (
                node_name[0].lower() if node_name and node_name[0].isalpha() else "misc"
            )
            groups[key].append((node_name, node_info))
        return groups

    def _process_node_group(
        self,
        cat_name: str,
        cat_pascal: str,
        group_nodes: list[tuple[str, Any]],
        exported_structs: dict[str, str],
    ) -> tuple[list[str], list[str]]:
        rs_blocks = []
        stub_blocks = []

        for node_name, node_info in group_nodes:
            struct_name = f"{cat_pascal}{pascal_case(node_name)}"

            prev_node = exported_structs.get(struct_name)
            if prev_node is not None:
                raise ValueError(
                    f"Duplicate exported struct '{struct_name}' in category '{cat_name}' "
                    f"(from nodes '{prev_node}' and '{node_name}')"
                )
            exported_structs[struct_name] = node_name

            parsed = parse_node(struct_name, node_name, node_info)
            rs_blocks.append(self.template_rs.render(node=parsed))
            stub_blocks.append(self.template_stub.render(node=parsed))

        return rs_blocks, stub_blocks

    # To speed up the editor's resolve process, split the code into modules a, b, c, ...,
    # but to avoid confusion when calling them with `use`, re-export them so they can be called as modules one level higher.
    def process_category(self, cat_name: str, nodes: dict[str, Any]) -> str | None:
        cat_snake = snake_case(cat_name)
        if not cat_snake:
            logger.warning(f"Skipping category with invalid name: {cat_name!r}")
            return None

        cat_pascal = pascal_case(cat_name)
        cat_rs_dir = self.rs_root / cat_snake
        cat_rs_dir.mkdir(parents=True, exist_ok=True)
        self.stub_root.mkdir(parents=True, exist_ok=True)

        groups = self._group_nodes_by_key(nodes)

        mod_lines = []
        export_lines = []
        exported_structs: dict[str, str] = {}
        all_stub_blocks = []

        for key, group_nodes in groups.items():
            safe_key = to_safe_ident(key)
            mod_lines.append(f"pub mod {safe_key};")
            export_lines.append(f"pub use {safe_key}::*;")

            rs_blocks, stub_blocks = self._process_node_group(
                cat_name, cat_pascal, group_nodes, exported_structs
            )

            (cat_rs_dir / f"{key}.rs").write_text(
                "\n\n".join(rs_blocks), encoding="utf-8"
            )
            all_stub_blocks.extend(stub_blocks)

        mod_content = "\n".join(mod_lines) + "\n\n" + "\n".join(export_lines) + "\n"
        (cat_rs_dir / "mod.rs").write_text(mod_content, encoding="utf-8")
        (self.stub_root / f"{cat_snake}.stub").write_text(
            "\n\n".join(all_stub_blocks), encoding="utf-8"
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
