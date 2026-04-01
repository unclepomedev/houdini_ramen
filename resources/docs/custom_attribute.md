# Attribute Rules
Houdini strictly coerces standard attributes (`life`, `P`, `v`, `Cd`, etc.) to specific types.
- **Custom Attributes:** NEVER use standard names for custom variables. It triggers VEX type mismatch errors (e.g., using `i@life` fails as `life` is forced to float). Use unique names (e.g., `sand_life`).
- **Standard Attributes:** When intentionally targeting standard attributes, you MUST use their exact Houdini-defined type signatures (e.g., `f@life`, `v@Cd`).
