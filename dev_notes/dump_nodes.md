### Known Errors & Warnings

When running `dump_nodes.py` (or when using the `--debug` flag), you may notice various errors and warnings printed to the console or the `houdini_warnings.log` file. These are **expected, benign noise** caused by temporarily creating and destroying Houdini nodes in a headless CLI environment (`hython`).

**They do not affect the quality or accuracy of the extracted metadata.** Here is a breakdown of why these messages occur and why they can be safely ignored:

#### 1. GUI-Dependent Module Errors (APEX / Rigging)
> `ImportError: hdefereval is only available in a graphical Houdini`  
> `TypeError: removeUpdateRigNodeCallback() missing...`

* **Cause**: Some newer nodes (like APEX's `autorigbuilder`) attempt to import GUI-specific modules (e.g., `hdefereval`, `statehud`) via their `OnCreated` or `OnDeleted` event callbacks. Because this extraction tool runs in a headless `hython` environment, these UI-building scripts crash.
* **Why it's fine**: The crash only halts the UI initialization callback. The node itself is successfully instantiated in memory, allowing our script to successfully extract the required `inputLabels()`.

#### 2. Dynamic Menu Script Out-of-Bounds Errors (SOPs)
> `IndexError: tuple index out of range` (Operation: `texturemaskpaint1/brushop`)

* **Cause**: Certain node parameters feature dynamic drop-down menus populated by Python scripts. These scripts often assume the node is connected to an input geometry (e.g., attempting to read `node.inputs()[0]`). Since our tool instantiates nodes in complete isolation for metadata extraction, these references fail.
* **Why it's fine**: This error only breaks the generation of parameter menu items. It has absolutely zero impact on the node's static metadata, such as the number of input ports or their labels.

#### 3. Handle Binding Warnings (COPs / SOPs)
> `Warning: error binding handle sidefx_transform2d because it doesn't exist.`

* **Cause**: Houdini attempts to bind viewport interaction handles (like transform gizmos) upon node creation. This fails because there is no active viewer pane in the headless CLI environment.
* **Why it's fine**: These are harmless Houdini C++ core warnings complaining about missing viewport features. They do not interrupt the Python execution or the metadata extraction process.

#### 4. Missing Execution Context Errors (LOPs / TOPs)
> `ValueError: invalid literal for int() with base 10: '___EXTERNAL___'`

* **Cause**: Some PDG (TOPs) or LOP nodes strictly expect an active work item or valid execution context upon creation. When dropped into an empty dummy network, they fail to resolve their expected values, resulting in type conversion errors.
* **Why it's fine**: Even if this error forces the node instantiation to abort, our script's built-in fallback mechanism catches the failure. It automatically pads the missing labels with empty strings `["", "", ...]` up to the node's known `maxNumInputs`. This guarantees that the Rust API generator will still flawlessly output type-safe, sequentially numbered methods (e.g., `set_input_0`, `set_input_1`) without dropping any available ports.
