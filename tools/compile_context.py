import json
import logging
import re
import sys
from pathlib import Path

logging.basicConfig(level=logging.WARNING)
logger = logging.getLogger(__name__)


def load_json(path: Path) -> dict:
    if not path.exists():
        logger.warning(f"Graph file not found: {path}")
        return {}
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def merge_graphs(auto_graph: dict, domain_graph: dict) -> dict:
    merged = {k: v.copy() for k, v in auto_graph.items()}

    for key, domain_data in domain_graph.items():
        if key in merged:
            merged[key].setdefault("depends_on", []).extend(
                domain_data.get("depends_on", [])
            )
        else:
            merged[key] = domain_data.copy()

    return merged


def extract_stub(content: str, struct_name: str) -> str | None:
    pattern = (
        rf"(pub\s+struct\s+{re.escape(struct_name)}\b.*?(?=\n\s*pub\s+struct\b|\Z))"
    )
    matches = re.findall(pattern, content, re.DOTALL)
    if matches:
        return matches[0].strip()
    return None


def process_node_file(
    target_id: str, node: dict, project_root: Path, errors: list
) -> str:
    node_path = node.get("path")
    if not node_path:
        return ""

    root = project_root.resolve()
    file_path = (project_root / node_path).resolve()

    try:
        file_path.relative_to(root)
    except ValueError:
        error_msg = f"Path escapes project root for '{target_id}': {node_path}"
        errors.append(error_msg)
        return f"\n[ERROR] {error_msg}\n"

    if not file_path.is_file():
        error_msg = f"File missing for '{target_id}': {file_path}"
        errors.append(error_msg)
        return f"\n[ERROR] {error_msg}\n"

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

    output_content = content

    if node.get("type") == "stub" and "struct_name" in node:
        stub = extract_stub(content, node["struct_name"])
        if stub is None:
            error_msg = f"Struct '{node['struct_name']}' not found in '{file_path}'"
            errors.append(error_msg)
            return f"\n[ERROR] {error_msg}\n"
        output_content = stub

    return (
        f"\n--- BEGIN: {target_id} ---\n{output_content}\n\n--- END: {target_id} ---\n"
    )


def compile_context(
    target_id: str, graph: dict, visited: set, project_root: Path, errors: list
) -> str:
    if target_id in visited:
        return ""

    visited.add(target_id)

    if target_id not in graph:
        errors.append(f"'{target_id}' not found in graph.")
        return f"\n[ERROR] '{target_id}' not found in graph.\n"

    node = graph[target_id]
    output = ""

    for dep_id in node.get("depends_on", []):
        output += compile_context(dep_id, graph, visited, project_root, errors)

    output += process_node_file(target_id, node, project_root, errors)

    return output


def main():
    if len(sys.argv) < 2:
        logger.error("Usage: compile_context.py <target_id>")
        sys.exit(1)

    target = sys.argv[1]
    project_root = Path(__file__).parent.parent

    auto_graph = load_json(project_root / "resources" / "auto_graph.json")
    domain_graph = load_json(project_root / "resources" / "domain_graph.json")

    merged_graph = merge_graphs(auto_graph, domain_graph)

    errors = []
    result = compile_context(target, merged_graph, set(), project_root, errors)

    print(result.strip())

    if errors:
        logger.error("Context compilation finished with errors:")
        for err in errors:
            logger.error(f" - {err}")
        sys.exit(1)


if __name__ == "__main__":
    main()
