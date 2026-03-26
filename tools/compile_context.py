import json
import sys
import re
from pathlib import Path


def load_json(path: Path) -> dict:
    if not path.exists():
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


def extract_stub(content: str, struct_name: str) -> str:
    pattern = rf"(pub\s+struct\s+{struct_name}\b.*?(?=\n\s*pub\s+struct\b|\Z))"
    matches = re.findall(pattern, content, re.DOTALL)
    if matches:
        return matches[0].strip()
    return content


def compile_context(
    target_id: str, graph: dict, visited: set, project_root: Path
) -> str:
    if target_id in visited:
        return ""

    visited.add(target_id)

    if target_id not in graph:
        return f"\n[ERROR] '{target_id}' not found in graph.\n"

    node = graph[target_id]
    output = ""

    for dep_id in node.get("depends_on", []):
        output += compile_context(dep_id, graph, visited, project_root)

    node_path = node.get("path")
    if node_path:
        file_path = project_root / node_path
        if file_path.exists():
            output += f"\n--- BEGIN: {target_id} ---\n"
            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()

            if node.get("type") == "stub" and "struct_name" in node:
                output += extract_stub(content, node["struct_name"])
            else:
                output += content

            output += f"\n\n--- END: {target_id} ---\n"
        else:
            output += f"\n[ERROR] File missing for '{target_id}': {file_path}\n"

    return output


def main():
    if len(sys.argv) < 2:
        sys.exit(1)

    target = sys.argv[1]
    project_root = Path(__file__).parent.parent

    auto_graph = load_json(project_root / "resources" / "auto_graph.json")
    domain_graph = load_json(project_root / "resources" / "domain_graph.json")

    merged_graph = merge_graphs(auto_graph, domain_graph)

    result = compile_context(target, merged_graph, set(), project_root)
    print(result.strip())


if __name__ == "__main__":
    main()
