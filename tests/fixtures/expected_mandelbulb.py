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
n_base_volume_1 = parent.createNode('volume', 'base_volume')
n_mandelbulb_2 = parent.createNode('volumewrangle', 'mandelbulb')
n_convert_to_sdf_3 = parent.createNode('convertvdb', 'convert_to_sdf')
n_smooth_4 = parent.createNode('vdbsmoothsdf', 'smooth')
n_convert_vdb_5 = parent.createNode('convertvdb', 'convert_vdb')
n_point_wrangle_6 = parent.createNode('pointwrangle', 'point_wrangle')
n_color_7 = parent.createNode('color', 'color')

# --- Spare Parameter Pass ---
ptg = n_mandelbulb_2.parmTemplateGroup()
pt = hou.FloatParmTemplate("scale", "Scale", 1, default_value=(0.75,), min=0.0, max=1.0)
ptg.append(pt)
pt = hou.FloatParmTemplate("shift", "Shift", 1, default_value=(0.0,), min=0.0, max=0.5)
ptg.append(pt)
pt = hou.IntParmTemplate("iteration", "Iteration", 1, default_value=(5,), min=0, max=10)
ptg.append(pt)
pt = hou.FloatParmTemplate("n", "N", 1, default_value=(8.0,), min=0.0, max=10.0)
ptg.append(pt)
n_mandelbulb_2.setParmTemplateGroup(ptg)

# --- Parameter Pass ---
n_base_volume_1.parm('name').set("density")
n_base_volume_1.parm('samplediv').set(200)
n_base_volume_1.parmTuple('size').set((3.0000, 3.0000, 3.0000))
n_mandelbulb_2.parm('snippet').set("float scale = chf(\"scale\");\nfloat shift = chf(\"shift\");\nint iteration = chi(\"iteration\");\nfloat n = chf(\"n\");\n\nf@density = 1;\n\nvector shiftv = set(shift, shift, shift);\nvector c = (@P) * scale + shiftv;\nvector v = c;\n\nfor(int i = 0; i < iteration; i++) {\n    float r = length(v);\n    float phi = atan2(v.y, v.x);\n    float theta = atan2(sqrt(v.x * v.x + v.y * v.y), v.z);\n    float vr = pow(r, n);\n    float vx = sin(n * theta) * cos(n * phi);\n    float vy = sin(n * theta) * sin(n * phi);\n    float vz = cos(n * theta);\n    v = set(vx, vy, vz) * vr + c;\n    if (length(v) > 100) {\n        f@density = 0;\n    }\n}\n")
n_convert_to_sdf_3.parm('conversion').set(1)
n_convert_to_sdf_3.parm('vdbclass').set(1)
n_smooth_4.parm('iterations').set(1)
n_convert_vdb_5.parm('conversion').set(2)
n_point_wrangle_6.parm('snippet').set("float dot = dot(@N, set(0, 1.0, 0));\nf@col = dot;\n")
n_color_7.parm('colortype').set(3)
n_color_7.parm('ramp').set(hou.Ramp((hou.rampBasis.Linear, hou.rampBasis.Linear, hou.rampBasis.Linear,), (0.0000, 0.5000, 1.0000,), ((0.1000, 0.2000, 0.8000), (0.8000, 0.2000, 0.8000), (1.0000, 0.8000, 0.1000),)))
n_color_7.parm('rampattribute').set("col")
n_color_7.parmTuple('ramprange').set((-1.0000, 1.0000))

# --- Link Pass ---
n_mandelbulb_2.setInput(0, n_base_volume_1, 0)
n_convert_to_sdf_3.setInput(0, n_mandelbulb_2, 0)
n_smooth_4.setInput(0, n_convert_to_sdf_3, 0)
n_convert_vdb_5.setInput(0, n_smooth_4, 0)
n_point_wrangle_6.setInput(0, n_convert_vdb_5, 0)
n_color_7.setInput(0, n_point_wrangle_6, 0)

# --- Layout & Display Pass ---
parent.layoutChildren()

n_color_7.setDisplayFlag(True)
n_color_7.setRenderFlag(True)
