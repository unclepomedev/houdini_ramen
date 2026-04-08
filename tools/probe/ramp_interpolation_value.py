import hou

node = hou.node("/obj").createNode("null", "ramp_test")

ptg = node.parmTemplateGroup()
ptg.append(hou.RampParmTemplate("test_ramp", "Test Ramp", hou.rampParmType.Float))
node.setParmTemplateGroup(ptg)

bases = (
    hou.rampBasis.Constant,
    hou.rampBasis.Linear,
    hou.rampBasis.CatmullRom,
    hou.rampBasis.MonotoneCubic,
    hou.rampBasis.Bezier,
    hou.rampBasis.BSpline,
    hou.rampBasis.Hermite,
)

print("--- Houdini Internal Ramp Interpolation Values ---")
for basis in bases:
    ramp = hou.Ramp((basis,), (0.0,), (1.0,))
    node.parm("test_ramp").set(ramp)
    basis_int = node.parm("test_ramp1interp").eval()
    print(f"{basis.name():<15} = {basis_int}")

node.destroy()
