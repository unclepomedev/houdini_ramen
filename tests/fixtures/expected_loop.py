import hou
parent_path = '/obj/geo1'
parent = hou.node(parent_path)
if not parent:
    parts = [p for p in parent_path.split('/') if p]
    curr = hou.node('/')
    curr_path = ''
    for i, part in enumerate(parts):
        curr_path += '/' + part
        child = hou.node(curr_path)
        if not child:
            n_type = 'geo' if i == len(parts) - 1 else 'subnet'
            curr = curr.createNode(n_type, part)
        else:
            curr = child
    parent = curr
if not parent:
    raise RuntimeError(f"Parent node '{parent_path}' not found")
for child in parent.children():
    child.destroy()

# --- Node Creation Pass ---
n_base_box_1 = parent.createNode('box', 'base_box')
n_process_points_begin_2 = parent.createNode('block_begin', 'process_points_begin')
n_inner_process_3 = parent.createNode('attribwrangle', 'inner_process')
n_process_points_end_4 = parent.createNode('block_end', 'process_points_end')
n_post_process_5 = parent.createNode('attribwrangle', 'post_process')

# --- Spare Parameter Pass ---

# --- Parameter Pass ---
n_base_box_1.parmTuple('size').set((2.0000, 2.0000, 2.0000))
n_process_points_begin_2.parm('blockpath').set("../process_points_end")
n_inner_process_3.parm('class').set(1)
n_inner_process_3.parm('snippet').set("@P.y += 1.0;\n")
n_process_points_end_4.parm('blockpath').set("../process_points_begin")
n_post_process_5.parm('class').set(1)
n_post_process_5.parm('snippet').set("@Cd = set(1, 0, 0);\n")

# --- Link Pass ---
n_process_points_begin_2.setInput(0, n_base_box_1, 0)
n_inner_process_3.setInput(0, n_process_points_begin_2, 0)
n_process_points_end_4.setInput(0, n_inner_process_3, 0)
n_post_process_5.setInput(0, n_process_points_end_4, 0)

# --- Layout Pass ---
parent.layoutChildren()
