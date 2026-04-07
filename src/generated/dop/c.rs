#[derive(Debug, Clone)]
pub struct DopChopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopChopnet {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }
}

impl crate::core::types::HoudiniNode for DopChopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "chopnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopClothconfigureobjectPlanarmodel {
    Isotropic = 0,
    Orthotropic = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopClothconfigureobjectBendmodel {
    Weak = 0,
    Strong = 1,
}

#[derive(Debug, Clone)]
pub struct DopClothconfigureobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopClothconfigureobject {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_shellstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_massdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_massdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shearstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shearstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shearstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shearstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weakbendstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "weakbendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weakbendstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weakbendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strongbendstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "strongbendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strongbendstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strongbendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisou(mut self, val: f32) -> Self {
        self.params.insert(
            "anisou".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisou_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisou".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellanisov(mut self, val: f32) -> Self {
        self.params.insert(
            "shellanisov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellanisov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellanisov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seamangle(mut self, val: f32) -> Self {
        self.params.insert(
            "seamangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seamangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seamangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradius(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fracturethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fracturethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaldrag(mut self, val: f32) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaldrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentdrag(mut self, val: f32) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tangentdrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvscale(mut self, val: f32) -> Self {
        self.params.insert(
            "uvscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uvscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_externalvelocityoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiuscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collisionradiuscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ucolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ucolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ucolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ucolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_planarmodel(mut self, val: DopClothconfigureobjectPlanarmodel) -> Self {
        self.params.insert(
            "planarmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planarmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planarmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendmodel(mut self, val: DopClothconfigureobjectBendmodel) -> Self {
        self.params.insert(
            "bendmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield(mut self, val: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_importrestgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importrestgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtargetgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importtargetgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideindependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideindependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidecodependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidecodependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideself(mut self, val: bool) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideself_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselfcomponent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselfcomponent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselffracturepart(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselffracturepart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createqualityattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createqualityattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createenergyattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createenergyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createforceattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createforceattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createcollisionattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcollisionattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfractureattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfractureattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiusenable(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionradiusenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvenable(mut self, val: bool) -> Self {
        self.params.insert(
            "uvenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uvenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopClothconfigureobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "clothconfigureobject"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopClothobjectInitializebehavior {
    None = 0,
    Chiffon = 1,
    Silk = 2,
    Cotton = 3,
    Denim = 4,
    SoftLeather = 5,
    RubberMat = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopClothobjectBendmodel {
    Weak = 0,
    Strong = 1,
}

#[derive(Debug, Clone)]
pub struct DopClothobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopClothobject {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_stiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_massdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_massdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shearstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shearstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shearstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shearstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weakbendstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "weakbendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weakbendstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weakbendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strongbendstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "strongbendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strongbendstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strongbendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seamangle(mut self, val: f32) -> Self {
        self.params.insert(
            "seamangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seamangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seamangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradius(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fracturethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fracturethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaldrag(mut self, val: f32) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaldrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentdrag(mut self, val: f32) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tangentdrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createframe(mut self, val: f32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_externalvelocityoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiuscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collisionradiuscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_initializebehavior(mut self, val: DopClothobjectInitializebehavior) -> Self {
        self.params.insert(
            "initializebehavior".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initializebehavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initializebehavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendmodel(mut self, val: DopClothobjectBendmodel) -> Self {
        self.params.insert(
            "bendmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield(mut self, val: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_object_name(mut self, val: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_importrestgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importrestgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtargetgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importtargetgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideindependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideindependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidecodependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidecodependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideself(mut self, val: bool) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideself_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselfcomponent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselfcomponent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselffracturepart(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselffracturepart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createqualityattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createqualityattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createenergyattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createenergyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createforceattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createforceattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createcollisionattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcollisionattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfractureattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfractureattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiusenable(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionradiusenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopClothobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "clothobject"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopClothsolverFloatprecision {
    /// Float 32-bit
    Float32MinusBit = 0,
    /// Float 64-bit
    Float64MinusBit = 1,
}

#[derive(Debug, Clone)]
pub struct DopClothsolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopClothsolver {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_unitlength(mut self, val: f32) -> Self {
        self.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitmass(mut self, val: f32) -> Self {
        self.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitmass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxglobalcollisionpasses(mut self, val: i32) -> Self {
        self.params.insert(
            "maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxglobalcollisionpasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_floatprecision(mut self, val: DopClothsolverFloatprecision) -> Self {
        self.params.insert(
            "floatprecision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_floatprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "floatprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopClothsolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "clothsolver"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopClothstitchconstraintType {
    Hard = 0,
    Soft = 1,
}

#[derive(Debug, Clone)]
pub struct DopClothstitchconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopClothstitchconstraint {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_stiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: DopClothstitchconstraintType) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constrainedobject(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalobject(mut self, val: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "goalpoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopClothstitchconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "clothstitchconstraint"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopClothvisualizationSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopClothvisualization {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopClothvisualization {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_velocityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "velocityscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvwscale(mut self, val: f32) -> Self {
        self.params.insert(
            "uvwscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uvwscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvwscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelerationscale(mut self, val: f32) -> Self {
        self.params.insert(
            "accelerationscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_accelerationscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accelerationscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jerkscale(mut self, val: f32) -> Self {
        self.params.insert(
            "jerkscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jerkscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jerkscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_thicknesscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "thicknesscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_thicknesscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknesscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_penetrationcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "penetrationcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_penetrationcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "penetrationcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocitycolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "velocitycolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_velocitycolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocitycolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ucolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ucolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ucolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ucolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelerationcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "accelerationcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_accelerationcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accelerationcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jerkcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "jerkcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_jerkcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jerkcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopClothvisualizationSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_thicknessenable(mut self, val: bool) -> Self {
        self.params.insert(
            "thicknessenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_thicknessenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_penetrationenable(mut self, val: bool) -> Self {
        self.params.insert(
            "penetrationenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_penetrationenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "penetrationenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocityenable(mut self, val: bool) -> Self {
        self.params.insert(
            "velocityenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocityenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvwenable(mut self, val: bool) -> Self {
        self.params.insert(
            "uvwenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uvwenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvwenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelerationenable(mut self, val: bool) -> Self {
        self.params.insert(
            "accelerationenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_accelerationenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accelerationenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jerkenable(mut self, val: bool) -> Self {
        self.params.insert(
            "jerkenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_jerkenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jerkenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopClothvisualization {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "clothvisualization"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopColliderelParmopPriority {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopColliderelDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopColliderelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopColliderel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopColliderel {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_priority(mut self, val: i32) -> Self {
        self.params.insert(
            "priority".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_priority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "priority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_priority(mut self, val: DopColliderelParmopPriority) -> Self {
        self.params.insert(
            "parmop_priority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_priority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_priority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopColliderelDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopColliderelSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affected(mut self, val: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affectors(mut self, val: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relname(mut self, val: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquerelname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquerelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_makemutual(mut self, val: bool) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makemutual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopColliderel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "colliderel"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopColliderlabelParmopColliderlabel {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopColliderlabelDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopColliderlabelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopColliderlabel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopColliderlabel {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_colliderlabel(mut self, val: DopColliderlabelParmopColliderlabel) -> Self {
        self.params.insert(
            "parmop_colliderlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_colliderlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_colliderlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopColliderlabelDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopColliderlabelSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_colliderlabel(mut self, val: &str) -> Self {
        self.params.insert(
            "colliderlabel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_colliderlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colliderlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopColliderlabel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "colliderlabel"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopEnablesoft {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMaxTwist {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMaxOutRotation {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMaxUpRotation {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopSoftness {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopComputeinitialerror {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopAngularlimitstiffness {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopAngularlimitdampingratio {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopCfm {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopBiasFactor {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopRelaxationFactor {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopTwisttranslationrange {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopOuttranslationrange {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopUptranslationrange {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopPositioncfm {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopPositionerp {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopPositionlimitstiffness {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopPositionlimitdampingratio {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopGoalTwistAxis {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopGoalUpAxis {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopConstrainedTwistAxis {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopConstrainedUpAxis {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopRestlength {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorEnabled {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetcurrentpose {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetr {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetp {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorHastargetprev {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetprevr {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetprevp {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetangularstiffness {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetangulardampingratio {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetpositionstiffness {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorTargetpositiondampingratio {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorNormalizemaximpulse {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorMaximpulse {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorErp {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopMotorCfm {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopNumiterations {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelParmopDisablecollisions {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConetwistconrelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopConetwistconrel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopConetwistconrel {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_max_twist(mut self, val: f32) -> Self {
        self.params.insert(
            "max_twist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_twist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_twist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_max_out_rotation(mut self, val: f32) -> Self {
        self.params.insert(
            "max_out_rotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_out_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_out_rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_max_up_rotation(mut self, val: f32) -> Self {
        self.params.insert(
            "max_up_rotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_up_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_up_rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angularlimitstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "angularlimitstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angularlimitstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angularlimitstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angularlimitdampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "angularlimitdampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angularlimitdampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angularlimitdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "cfm".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bias_factor(mut self, val: f32) -> Self {
        self.params.insert(
            "bias_factor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bias_factor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relaxation_factor(mut self, val: f32) -> Self {
        self.params.insert(
            "relaxation_factor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_relaxation_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relaxation_factor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positioncfm(mut self, val: f32) -> Self {
        self.params.insert(
            "positioncfm".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positioncfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positioncfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionerp(mut self, val: f32) -> Self {
        self.params.insert(
            "positionerp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positionerp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionerp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionlimitstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "positionlimitstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positionlimitstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionlimitstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionlimitdampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "positionlimitdampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positionlimitdampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionlimitdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restlength(mut self, val: f32) -> Self {
        self.params.insert(
            "restlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetangularstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_targetangularstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_targetangularstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetangularstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetangulardampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_targetangulardampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_targetangulardampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetangulardampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetpositionstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_targetpositionstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_targetpositionstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetpositionstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetpositiondampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_targetpositiondampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_targetpositiondampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetpositiondampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_maximpulse(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_maximpulse".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_maximpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_maximpulse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_erp(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_erp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_erp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_cfm".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_cfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_size(mut self, val: f32) -> Self {
        self.params.insert(
            "guide_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guide_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_twisttranslationrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "twisttranslationrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_twisttranslationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twisttranslationrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtranslationrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "outtranslationrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_outtranslationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtranslationrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uptranslationrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "uptranslationrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_uptranslationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uptranslationrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_goal_twist_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goal_twist_axis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goal_twist_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_twist_axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goal_up_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goal_up_axis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goal_up_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_up_axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrained_twist_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "constrained_twist_axis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_constrained_twist_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrained_twist_axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrained_up_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "constrained_up_axis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_constrained_up_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrained_up_axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "motor_targetr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_motor_targetr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "motor_targetp".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_motor_targetp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetprevr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "motor_targetprevr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_motor_targetprevr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetprevr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetprevp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "motor_targetprevp".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_motor_targetprevp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetprevp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_twistlimitcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_twistlimitcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_twistlimitcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_twistlimitcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_secondary_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_secondary_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_secondary_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_secondary_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_twistaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_twistaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_twistaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_twistaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_upaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_upaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_upaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_upaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_motortargetcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_motortargetcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_motortargetcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_motortargetcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_enablesoft(mut self, val: DopConetwistconrelParmopEnablesoft) -> Self {
        self.params.insert(
            "parmop_enablesoft".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablesoft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablesoft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_max_twist(mut self, val: DopConetwistconrelParmopMaxTwist) -> Self {
        self.params.insert(
            "parmop_max_twist".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_max_twist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_max_twist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_max_out_rotation(
        mut self,
        val: DopConetwistconrelParmopMaxOutRotation,
    ) -> Self {
        self.params.insert(
            "parmop_max_out_rotation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_max_out_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_max_out_rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_max_up_rotation(
        mut self,
        val: DopConetwistconrelParmopMaxUpRotation,
    ) -> Self {
        self.params.insert(
            "parmop_max_up_rotation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_max_up_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_max_up_rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_softness(mut self, val: DopConetwistconrelParmopSoftness) -> Self {
        self.params.insert(
            "parmop_softness".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_computeinitialerror(
        mut self,
        val: DopConetwistconrelParmopComputeinitialerror,
    ) -> Self {
        self.params.insert(
            "parmop_computeinitialerror".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_computeinitialerror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_computeinitialerror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_angularlimitstiffness(
        mut self,
        val: DopConetwistconrelParmopAngularlimitstiffness,
    ) -> Self {
        self.params.insert(
            "parmop_angularlimitstiffness".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_angularlimitstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_angularlimitstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_angularlimitdampingratio(
        mut self,
        val: DopConetwistconrelParmopAngularlimitdampingratio,
    ) -> Self {
        self.params.insert(
            "parmop_angularlimitdampingratio".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_angularlimitdampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_angularlimitdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_cfm(mut self, val: DopConetwistconrelParmopCfm) -> Self {
        self.params.insert(
            "parmop_cfm".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_bias_factor(mut self, val: DopConetwistconrelParmopBiasFactor) -> Self {
        self.params.insert(
            "parmop_bias_factor".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_bias_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_bias_factor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_relaxation_factor(
        mut self,
        val: DopConetwistconrelParmopRelaxationFactor,
    ) -> Self {
        self.params.insert(
            "parmop_relaxation_factor".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_relaxation_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_relaxation_factor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_twisttranslationrange(
        mut self,
        val: DopConetwistconrelParmopTwisttranslationrange,
    ) -> Self {
        self.params.insert(
            "parmop_twisttranslationrange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_twisttranslationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_twisttranslationrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_outtranslationrange(
        mut self,
        val: DopConetwistconrelParmopOuttranslationrange,
    ) -> Self {
        self.params.insert(
            "parmop_outtranslationrange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_outtranslationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_outtranslationrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_uptranslationrange(
        mut self,
        val: DopConetwistconrelParmopUptranslationrange,
    ) -> Self {
        self.params.insert(
            "parmop_uptranslationrange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_uptranslationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_uptranslationrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_positioncfm(mut self, val: DopConetwistconrelParmopPositioncfm) -> Self {
        self.params.insert(
            "parmop_positioncfm".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_positioncfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_positioncfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_positionerp(mut self, val: DopConetwistconrelParmopPositionerp) -> Self {
        self.params.insert(
            "parmop_positionerp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_positionerp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_positionerp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_positionlimitstiffness(
        mut self,
        val: DopConetwistconrelParmopPositionlimitstiffness,
    ) -> Self {
        self.params.insert(
            "parmop_positionlimitstiffness".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_positionlimitstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_positionlimitstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_positionlimitdampingratio(
        mut self,
        val: DopConetwistconrelParmopPositionlimitdampingratio,
    ) -> Self {
        self.params.insert(
            "parmop_positionlimitdampingratio".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_positionlimitdampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_positionlimitdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_goal_twist_axis(
        mut self,
        val: DopConetwistconrelParmopGoalTwistAxis,
    ) -> Self {
        self.params.insert(
            "parmop_goal_twist_axis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_goal_twist_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_goal_twist_axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_goal_up_axis(mut self, val: DopConetwistconrelParmopGoalUpAxis) -> Self {
        self.params.insert(
            "parmop_goal_up_axis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_goal_up_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_goal_up_axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_constrained_twist_axis(
        mut self,
        val: DopConetwistconrelParmopConstrainedTwistAxis,
    ) -> Self {
        self.params.insert(
            "parmop_constrained_twist_axis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_constrained_twist_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_constrained_twist_axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_constrained_up_axis(
        mut self,
        val: DopConetwistconrelParmopConstrainedUpAxis,
    ) -> Self {
        self.params.insert(
            "parmop_constrained_up_axis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_constrained_up_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_constrained_up_axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_restlength(mut self, val: DopConetwistconrelParmopRestlength) -> Self {
        self.params.insert(
            "parmop_restlength".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_restlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_enabled(mut self, val: DopConetwistconrelParmopMotorEnabled) -> Self {
        self.params.insert(
            "parmop_motor_enabled".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_enabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_enabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetcurrentpose(
        mut self,
        val: DopConetwistconrelParmopMotorTargetcurrentpose,
    ) -> Self {
        self.params.insert(
            "parmop_motor_targetcurrentpose".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetcurrentpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetcurrentpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetr(mut self, val: DopConetwistconrelParmopMotorTargetr) -> Self {
        self.params.insert(
            "parmop_motor_targetr".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetp(mut self, val: DopConetwistconrelParmopMotorTargetp) -> Self {
        self.params.insert(
            "parmop_motor_targetp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_hastargetprev(
        mut self,
        val: DopConetwistconrelParmopMotorHastargetprev,
    ) -> Self {
        self.params.insert(
            "parmop_motor_hastargetprev".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_hastargetprev_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_hastargetprev".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetprevr(
        mut self,
        val: DopConetwistconrelParmopMotorTargetprevr,
    ) -> Self {
        self.params.insert(
            "parmop_motor_targetprevr".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetprevr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetprevr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetprevp(
        mut self,
        val: DopConetwistconrelParmopMotorTargetprevp,
    ) -> Self {
        self.params.insert(
            "parmop_motor_targetprevp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetprevp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetprevp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetangularstiffness(
        mut self,
        val: DopConetwistconrelParmopMotorTargetangularstiffness,
    ) -> Self {
        self.params.insert(
            "parmop_motor_targetangularstiffness".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetangularstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetangularstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetangulardampingratio(
        mut self,
        val: DopConetwistconrelParmopMotorTargetangulardampingratio,
    ) -> Self {
        self.params.insert(
            "parmop_motor_targetangulardampingratio".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetangulardampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetangulardampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetpositionstiffness(
        mut self,
        val: DopConetwistconrelParmopMotorTargetpositionstiffness,
    ) -> Self {
        self.params.insert(
            "parmop_motor_targetpositionstiffness".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetpositionstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetpositionstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_targetpositiondampingratio(
        mut self,
        val: DopConetwistconrelParmopMotorTargetpositiondampingratio,
    ) -> Self {
        self.params.insert(
            "parmop_motor_targetpositiondampingratio".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_targetpositiondampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_targetpositiondampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_normalizemaximpulse(
        mut self,
        val: DopConetwistconrelParmopMotorNormalizemaximpulse,
    ) -> Self {
        self.params.insert(
            "parmop_motor_normalizemaximpulse".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_normalizemaximpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_normalizemaximpulse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_maximpulse(
        mut self,
        val: DopConetwistconrelParmopMotorMaximpulse,
    ) -> Self {
        self.params.insert(
            "parmop_motor_maximpulse".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_maximpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_maximpulse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_erp(mut self, val: DopConetwistconrelParmopMotorErp) -> Self {
        self.params.insert(
            "parmop_motor_erp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_erp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_motor_cfm(mut self, val: DopConetwistconrelParmopMotorCfm) -> Self {
        self.params.insert(
            "parmop_motor_cfm".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_motor_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_motor_cfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_numiterations(mut self, val: DopConetwistconrelParmopNumiterations) -> Self {
        self.params.insert(
            "parmop_numiterations".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_numiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_disablecollisions(
        mut self,
        val: DopConetwistconrelParmopDisablecollisions,
    ) -> Self {
        self.params.insert(
            "parmop_disablecollisions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_disablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopConetwistconrelDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopConetwistconrelSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablesoft(mut self, val: bool) -> Self {
        self.params.insert(
            "enablesoft".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesoft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablesoft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computeinitialerror(mut self, val: bool) -> Self {
        self.params.insert(
            "computeinitialerror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeinitialerror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeinitialerror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_enabled(mut self, val: bool) -> Self {
        self.params.insert(
            "motor_enabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motor_enabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_enabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_targetcurrentpose(mut self, val: bool) -> Self {
        self.params.insert(
            "motor_targetcurrentpose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motor_targetcurrentpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetcurrentpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_hastargetprev(mut self, val: bool) -> Self {
        self.params.insert(
            "motor_hastargetprev".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motor_hastargetprev_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_hastargetprev".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motor_normalizemaximpulse(mut self, val: bool) -> Self {
        self.params.insert(
            "motor_normalizemaximpulse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motor_normalizemaximpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_normalizemaximpulse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_show_soft(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_show_soft".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_show_soft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_show_soft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopConetwistconrel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "conetwistconrel"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConrelationshipReltype {
    HardConstraint = 0,
    SpringConstraint = 1,
    TwoStateConstraint = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConrelationshipTwostateinitial {
    HardConstraint = 0,
    SpringConstraint = 1,
}

#[derive(Debug, Clone)]
pub struct DopConrelationship {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopConrelationship {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_twostateforce(mut self, val: f32) -> Self {
        self.params.insert(
            "twostateforce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twostateforce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twostateforce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twostatemindistance(mut self, val: f32) -> Self {
        self.params.insert(
            "twostatemindistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twostatemindistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twostatemindistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_springstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "springstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_springstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "springstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_springdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "springdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_springdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "springdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_springrestlength(mut self, val: f32) -> Self {
        self.params.insert(
            "springrestlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_springrestlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "springrestlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_hardguidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hardguidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hardguidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hardguidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_springguidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "springguidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_springguidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "springguidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_reltype(mut self, val: DopConrelationshipReltype) -> Self {
        self.params.insert(
            "reltype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_reltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twostateinitial(mut self, val: DopConrelationshipTwostateinitial) -> Self {
        self.params.insert(
            "twostateinitial".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_twostateinitial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twostateinitial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopConrelationship {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "conrelationship"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConstraintSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopConstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopConstraint {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopConstraintSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affected(mut self, val: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affectors(mut self, val: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relname(mut self, val: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquerelname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquerelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_makemutual(mut self, val: bool) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makemutual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopConstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraint"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConstraintnetworkGeosource {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone)]
pub struct DopConstraintnetwork {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopConstraintnetwork {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Constraints to create"
    pub fn set_input_constraints_to_create<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Constraints to create" and specifies the output index of the target node.
    pub fn set_input_constraints_to_create_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Constraint Solvers"
    pub fn set_input_constraint_solvers<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Constraint Solvers" and specifies the output index of the target node.
    pub fn set_input_constraint_solvers_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_reloadfromsops(mut self, val: i32) -> Self {
        self.params.insert(
            "reloadfromsops".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_reloadfromsops_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reloadfromsops".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_geosource(mut self, val: DopConstraintnetworkGeosource) -> Self {
        self.params.insert(
            "geosource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geosource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchbyattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "matchbyattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchbyattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchbyattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relname(mut self, val: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usetransform(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablematchbyattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "enablematchbyattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablematchbyattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablematchbyattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attachinternalconstraints(mut self, val: bool) -> Self {
        self.params.insert(
            "attachinternalconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attachinternalconstraints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attachinternalconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopConstraintnetwork {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintnetwork"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConstraintnetworkrelationshipParmopEnablematchbyattrib {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConstraintnetworkrelationshipParmopMatchbyattrib {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConstraintnetworkrelationshipDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConstraintnetworkrelationshipSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopConstraintnetworkrelationship {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopConstraintnetworkrelationship {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_enablematchbyattrib(
        mut self,
        val: DopConstraintnetworkrelationshipParmopEnablematchbyattrib,
    ) -> Self {
        self.params.insert(
            "parmop_enablematchbyattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablematchbyattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablematchbyattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_matchbyattrib(
        mut self,
        val: DopConstraintnetworkrelationshipParmopMatchbyattrib,
    ) -> Self {
        self.params.insert(
            "parmop_matchbyattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_matchbyattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_matchbyattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(
        mut self,
        val: DopConstraintnetworkrelationshipDefaultparmop,
    ) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopConstraintnetworkrelationshipSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_matchbyattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "matchbyattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchbyattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchbyattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affected(mut self, val: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affectors(mut self, val: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relname(mut self, val: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablematchbyattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "enablematchbyattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablematchbyattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablematchbyattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquerelname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquerelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_makemutual(mut self, val: bool) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makemutual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopConstraintnetworkrelationship {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintnetworkrelationship"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopConstraintnetworkvisualizationSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopConstraintnetworkvisualization {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopConstraintnetworkvisualization {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopConstraintnetworkvisualizationSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopConstraintnetworkvisualization {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintnetworkvisualization"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopContainerSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopContainer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopContainer {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopContainerSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopContainer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "container"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct DopCop2net {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCop2net {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }
}

impl crate::core::types::HoudiniNode for DopCop2net {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "cop2net"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopnetBorder {
    Constant = 0,
    Clamp = 1,
    Mirror = 2,
    Wrap = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopnetPrecision {
    /// 16-bit
    N16MinusBit = 0,
    /// 32-bit
    N32MinusBit = 1,
}

#[derive(Debug, Clone)]
pub struct DopCopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCopnet {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_pixelscale(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_udim(mut self, val: i32) -> Self {
        self.params
            .insert("udim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "udim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vistile(mut self, val: i32) -> Self {
        self.params.insert(
            "vistile".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vistile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vistile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_res(mut self, val: [i32; 2]) -> Self {
        self.params
            .insert("res".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_resmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resmenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_border(mut self, val: DopCopnetBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision(mut self, val: DopCopnetPrecision) -> Self {
        self.params.insert(
            "precision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_precision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_setres(mut self, val: bool) -> Self {
        self.params.insert(
            "setres".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setpixelscale(mut self, val: bool) -> Self {
        self.params.insert(
            "setpixelscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setpixelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setpixelscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setborder(mut self, val: bool) -> Self {
        self.params.insert(
            "setborder".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setborder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setborder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setprecision(mut self, val: bool) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setudim(mut self, val: bool) -> Self {
        self.params.insert(
            "setudim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setudim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setvistile(mut self, val: bool) -> Self {
        self.params.insert(
            "setvistile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setvistile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setvistile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "copnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopydataVareval {
    EvaluateForEachCopy = 0,
    EvaluateOnce = 1,
    /// Evaluate Once, One Token per Copy
    EvaluateOnceOneTokenPerCopy = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopydataVartype {
    Float = 0,
    String = 1,
}

#[derive(Debug, Clone)]
pub struct DopCopydata {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCopydata {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Data to be copied"
    pub fn set_input_data_to_be_copied<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Data to be copied" and specifies the output index of the target node.
    pub fn set_input_data_to_be_copied_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_varfloat_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("varfloat{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_varfloat_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varfloat{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numcopies(mut self, val: i32) -> Self {
        self.params.insert(
            "numcopies".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numcopies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numcopies".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vareval_inst(mut self, index1: usize, val: DopCopydataVareval) -> Self {
        self.params.insert(
            format!("vareval{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vareval_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vareval{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vartype_inst(mut self, index1: usize, val: DopCopydataVartype) -> Self {
        self.params.insert(
            format!("vartype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vartype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vartype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("varname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_varname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varstring_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("varstring{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_varstring_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varstring{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCopydata {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "copydata"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct DopCopyobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCopyobject {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be copied"
    pub fn set_input_objects_to_be_copied<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be copied" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_copied_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numcopies(mut self, val: i32) -> Self {
        self.params.insert(
            "numcopies".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numcopies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numcopies".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_objindex(mut self, val: i32) -> Self {
        self.params.insert(
            "objindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_objindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_objtocopy(mut self, val: &str) -> Self {
        self.params.insert(
            "objtocopy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objtocopy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objtocopy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_objname(mut self, val: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_copyourobjects(mut self, val: bool) -> Self {
        self.params.insert(
            "copyourobjects".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_copyourobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyourobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preserveobjects(mut self, val: bool) -> Self {
        self.params.insert(
            "preserveobjects".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preserveobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preserveobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCopyobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "copyobject"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopyobjectinfoSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopCopyobjectinfo {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCopyobjectinfo {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_copynum(mut self, val: i32) -> Self {
        self.params.insert(
            "copynum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_copynum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copynum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceobjid(mut self, val: i32) -> Self {
        self.params.insert(
            "sourceobjid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sourceobjid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceobjid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopCopyobjectinfoSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCopyobjectinfo {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "copyobjectinfo"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopysolverParmopSrcisme {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopysolverParmopSrcobject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopysolverParmopSrcdata {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopysolverParmopDestdata {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCopysolverDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone)]
pub struct DopCopysolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopCopysolver {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_srcisme(mut self, val: DopCopysolverParmopSrcisme) -> Self {
        self.params.insert(
            "parmop_srcisme".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_srcisme_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_srcisme".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_srcobject(mut self, val: DopCopysolverParmopSrcobject) -> Self {
        self.params.insert(
            "parmop_srcobject".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_srcobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_srcobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_srcdata(mut self, val: DopCopysolverParmopSrcdata) -> Self {
        self.params.insert(
            "parmop_srcdata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_srcdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_srcdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_destdata(mut self, val: DopCopysolverParmopDestdata) -> Self {
        self.params.insert(
            "parmop_destdata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_destdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_destdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopCopysolverDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_srcobject(mut self, val: &str) -> Self {
        self.params.insert(
            "srcobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_srcobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srcdata(mut self, val: &str) -> Self {
        self.params.insert(
            "srcdata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_srcdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_destdata(mut self, val: &str) -> Self {
        self.params.insert(
            "destdata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_destdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_srcisme(mut self, val: bool) -> Self {
        self.params.insert(
            "srcisme".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_srcisme_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcisme".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addaffectors(mut self, val: bool) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addaffectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solverperobject(mut self, val: bool) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solverperobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCopysolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "copysolver"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdfuzzylogicOutputtype {
    Boolean = 0,
    Crisp = 1,
}

#[derive(Debug, Clone)]
pub struct DopCrowdfuzzylogic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCrowdfuzzylogic {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Input 2"
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input 2" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Input 3"
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Input 3" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Input 4"
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Input 4" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: "Sub-Network Input #5"
    pub fn set_input_sub_network_input_5<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "Sub-Network Input #5" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: "Sub-Network Input #6"
    pub fn set_input_sub_network_input_6<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "Sub-Network Input #6" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: "Sub-Network Input #7"
    pub fn set_input_sub_network_input_7<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "Sub-Network Input #7" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: "Sub-Network Input #8"
    pub fn set_input_sub_network_input_8<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "Sub-Network Input #8" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: "Sub-Network Input #9"
    pub fn set_input_sub_network_input_9<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "Sub-Network Input #9" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_9_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: "Sub-Network Input #10"
    pub fn set_input_sub_network_input_10<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "Sub-Network Input #10" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_10_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_booleanthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "booleanthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_booleanthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "booleanthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_outputtype(mut self, val: DopCrowdfuzzylogicOutputtype) -> Self {
        self.params.insert(
            "outputtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_thresholdtest(mut self, val: &str) -> Self {
        self.params.insert(
            "thresholdtest".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thresholdtest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thresholdtest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triggerattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "triggerattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_triggerattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triggerattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCrowdfuzzylogic {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "crowdfuzzylogic"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("popvop")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait DopCrowdfuzzylogicInnerExt {
    fn compare1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn const1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn const2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn parm_booleanthreshold(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> DopCrowdfuzzylogicInnerExt for crate::core::graph::InnerGraph<'a, DopCrowdfuzzylogic> {
    fn compare1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("popvop/compare1")
    }
    fn const1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("popvop/const1")
    }
    fn const2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("popvop/const2")
    }
    fn output(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("popvop/output")
    }
    fn parm_booleanthreshold(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("popvop/parm_booleanthreshold")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("popvop/switch1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdobjectInitialgeometrysource {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdobjectClipgraphsource {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdobjectClippropertiessource {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdobjectTargetgeometrysource {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone)]
pub struct DopCrowdobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCrowdobject {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_createframe(mut self, val: f32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_life(mut self, val: f32) -> Self {
        self.params.insert(
            "life".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_life_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "life".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_length(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_collision_margin(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_collision_margin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_collision_margin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_collision_margin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_adjust_factor(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_adjust_factor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_adjust_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_adjust_factor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_shrink_amount(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_shrink_amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_shrink_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_shrink_amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_linear_sleep_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_linear_sleep_threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_linear_sleep_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_linear_sleep_threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_angular_sleep_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_angular_sleep_threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_angular_sleep_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_angular_sleep_threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inertialtensorstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_deactivated_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bullet_deactivated_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bullet_deactivated_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_deactivated_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_primt(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bullet_primT".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bullet_primt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_primT".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_primr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bullet_primR".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bullet_primr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_primR".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_prims(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bullet_primS".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bullet_prims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_primS".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_initialgeometrysource(mut self, val: DopCrowdobjectInitialgeometrysource) -> Self {
        self.params.insert(
            "initialgeometrysource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initialgeometrysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialgeometrysource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipgraphsource(mut self, val: DopCrowdobjectClipgraphsource) -> Self {
        self.params.insert(
            "clipgraphsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clipgraphsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipgraphsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clippropertiessource(mut self, val: DopCrowdobjectClippropertiessource) -> Self {
        self.params.insert(
            "clippropertiessource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clippropertiessource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clippropertiessource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometrysource(mut self, val: DopCrowdobjectTargetgeometrysource) -> Self {
        self.params.insert(
            "targetgeometrysource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetgeometrysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgeometrysource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object_name(mut self, val: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packtype(mut self, val: &str) -> Self {
        self.params.insert(
            "packtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialgeometry(mut self, val: &str) -> Self {
        self.params.insert(
            "initialgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_initialgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipgraph(mut self, val: &str) -> Self {
        self.params.insert(
            "clipgraph".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipgraph_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipgraph".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipproperties(mut self, val: &str) -> Self {
        self.params.insert(
            "clipproperties".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipproperties_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipproperties".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometry(mut self, val: &str) -> Self {
        self.params.insert(
            "targetgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_georep(mut self, val: &str) -> Self {
        self.params.insert(
            "bullet_georep".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bullet_georep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_georep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_active(mut self, val: bool) -> Self {
        self.params.insert(
            "active".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_active_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "active".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: bool) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableragdoll(mut self, val: bool) -> Self {
        self.params.insert(
            "enableragdoll".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableragdoll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableragdoll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetransform(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doid(mut self, val: bool) -> Self {
        self.params.insert(
            "doid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetargetgeometrytransform(mut self, val: bool) -> Self {
        self.params.insert(
            "usetargetgeometrytransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetargetgeometrytransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetargetgeometrytransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_triangulate(mut self, val: bool) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_triangulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_geoconvexhull(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_geoconvexhull".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_geoconvexhull_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_geoconvexhull".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_groupconnected(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_groupconnected".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_groupconnected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_groupconnected".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_autofit(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_autofit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_autofit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_autofit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_adjust_geometry(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_adjust_geometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_adjust_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_adjust_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_add_impact(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_add_impact".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_add_impact_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_add_impact".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bullet_want_deactivate(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_want_deactivate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_want_deactivate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_want_deactivate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCrowdobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "crowdobject"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdsolverSetupvector {
    Unchanged = 0,
    SetPerpendicularToVelocity = 1,
    SetToTerrainNormal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdsolverLookatsolver {
    /// Simple (Deprecated)
    SimpleDeprecated = 0,
    Advanced = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdsolverMintargetscoremode {
    SetValue = 0,
    SetFromAttribute = 1,
    ScaleByAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdsolverTerrainsource {
    Sop = 0,
    DopData = 1,
    FirstContextGeometry = 2,
    SecondContextGeometry = 3,
    ThirdContextGeometry = 4,
    FourthContextGeometry = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdsolverMode {
    DirectionVector = 0,
    UpAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdsolverSamplingmethod {
    Particle = 0,
    Foot = 1,
}

#[derive(Debug, Clone)]
pub struct DopCrowdsolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCrowdsolver {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Crowd Object"
    pub fn set_input_crowd_object<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Crowd Object" and specifies the output index of the target node.
    pub fn set_input_crowd_object_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Crowd Source"
    pub fn set_input_crowd_source<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Crowd Source" and specifies the output index of the target node.
    pub fn set_input_crowd_source_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "States"
    pub fn set_input_states<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "States" and specifies the output index of the target node.
    pub fn set_input_states_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Transitions"
    pub fn set_input_transitions<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Transitions" and specifies the output index of the target node.
    pub fn set_input_transitions_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_maxforce(mut self, val: f32) -> Self {
        self.params.insert(
            "maxforce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxforce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxforce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_airresist(mut self, val: f32) -> Self {
        self.params.insert(
            "airresist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_airresist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "airresist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "orientthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_orientthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxturnrate(mut self, val: f32) -> Self {
        self.params.insert(
            "maxturnrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxturnrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxturnrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_turnstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "turnstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_turnstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turnstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_turndamping(mut self, val: f32) -> Self {
        self.params.insert(
            "turndamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_turndamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turndamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_turnaccelmax(mut self, val: f32) -> Self {
        self.params.insert(
            "turnaccelmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_turnaccelmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turnaccelmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtiltrate(mut self, val: f32) -> Self {
        self.params.insert(
            "maxtiltrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtiltrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxtiltrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tiltstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "tiltstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tiltstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tiltstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tiltdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "tiltdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tiltdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tiltdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tiltaccelmax(mut self, val: f32) -> Self {
        self.params.insert(
            "tiltaccelmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tiltaccelmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tiltaccelmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_locomotionstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "locomotionstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_locomotionstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "locomotionstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_siminfluence(mut self, val: f32) -> Self {
        self.params.insert(
            "siminfluence".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_siminfluence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "siminfluence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weight(mut self, val: f32) -> Self {
        self.params.insert(
            "weight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlescale(mut self, val: f32) -> Self {
        self.params.insert(
            "particlescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forcescale(mut self, val: f32) -> Self {
        self.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookaheadtime(mut self, val: f32) -> Self {
        self.params.insert(
            "lookaheadtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lookaheadtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookaheadtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ndist(mut self, val: f32) -> Self {
        self.params.insert(
            "ndist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ndist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ndist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mintargetscore(mut self, val: f32) -> Self {
        self.params.insert(
            "mintargetscore".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mintargetscore_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mintargetscore".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headturnstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "headturnstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headturnstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headturnstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headweightscale(mut self, val: f32) -> Self {
        self.params.insert(
            "headweightscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headweightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headweightscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeturnstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "eyeturnstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyeturnstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeturnstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeweightscale(mut self, val: f32) -> Self {
        self.params.insert(
            "eyeweightscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyeweightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeweightscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headturnangle(mut self, val: f32) -> Self {
        self.params.insert(
            "headturnangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headturnangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headturnangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainguidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "terrainguidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_terrainguidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainguidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lockedscale(mut self, val: f32) -> Self {
        self.params.insert(
            "lockedscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lockedscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lockedscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hipoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "hipoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hipoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hipshiftperframe(mut self, val: f32) -> Self {
        self.params.insert(
            "hipshiftperframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hipshiftperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipshiftperframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kneedampingthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "kneedampingthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_kneedampingthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kneedampingthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ankleoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "ankleoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ankleoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ankleoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toeoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "toeoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_toeoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toeoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tiltangleperframe(mut self, val: f32) -> Self {
        self.params.insert(
            "tiltangleperframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tiltangleperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tiltangleperframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mintilt(mut self, val: f32) -> Self {
        self.params.insert(
            "mintilt".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mintilt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mintilt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtilt(mut self, val: f32) -> Self {
        self.params.insert(
            "maxtilt".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtilt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxtilt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cflcond(mut self, val: f32) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cflcond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_turnanglerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "turnanglerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_turnanglerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turnanglerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headweightanglerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "headweightanglerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_headweightanglerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headweightanglerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeweightanglerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "eyeweightanglerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_eyeweightanglerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeweightanglerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_refdir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refup(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refup".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_avoidanceguidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "avoidanceguidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_avoidanceguidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "avoidanceguidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyeoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyeoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projectiondir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "projectiondir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_projectiondir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectiondir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxneighbors(mut self, val: i32) -> Self {
        self.params.insert(
            "maxneighbors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxneighbors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxneighbors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numjoints(mut self, val: i32) -> Self {
        self.params.insert(
            "numjoints".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numjoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numjoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minimumsubsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minimumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framesbeforesolve(mut self, val: i32) -> Self {
        self.params.insert(
            "framesbeforesolve".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_framesbeforesolve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framesbeforesolve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_setupvector(mut self, val: DopCrowdsolverSetupvector) -> Self {
        self.params.insert(
            "setupvector".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_setupvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setupvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatsolver(mut self, val: DopCrowdsolverLookatsolver) -> Self {
        self.params.insert(
            "lookatsolver".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookatsolver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatsolver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mintargetscoremode(mut self, val: DopCrowdsolverMintargetscoremode) -> Self {
        self.params.insert(
            "mintargetscoremode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mintargetscoremode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mintargetscoremode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainsource(mut self, val: DopCrowdsolverTerrainsource) -> Self {
        self.params.insert(
            "terrainsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_terrainsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: DopCrowdsolverMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplingmethod(mut self, val: DopCrowdsolverSamplingmethod) -> Self {
        self.params.insert(
            "samplingmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplingmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplingmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_headweightramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "headweightramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_headweightramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headweightramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeweightramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "eyeweightramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_eyeweightramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeweightramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainguidecolor(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "terrainguidecolor".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_terrainguidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainguidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_avoidancegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "avoidancegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_avoidancegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "avoidancegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mintargetscoreattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "mintargetscoreattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mintargetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mintargetscoreattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetnameattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "targetnameattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetnameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetnameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetposattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "targetposattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetposattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetposattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetscoreattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "targetscoreattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetscoreattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terraingroup(mut self, val: &str) -> Self {
        self.params.insert(
            "terraingroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terraingroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terraingroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainsoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "terrainsoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terrainsoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainsoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terraindopobject(mut self, val: &str) -> Self {
        self.params.insert(
            "terraindopobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terraindopobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terraindopobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terraindopsubobject(mut self, val: &str) -> Self {
        self.params.insert(
            "terraindopsubobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terraindopsubobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terraindopsubobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terraingeogroup(mut self, val: &str) -> Self {
        self.params.insert(
            "terraingeogroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terraingeogroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terraingeogroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainadaptationgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "terrainadaptationgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terrainadaptationgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainadaptationgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_turnconstraint(mut self, val: bool) -> Self {
        self.params.insert(
            "turnconstraint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_turnconstraint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turnconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projectforce(mut self, val: bool) -> Self {
        self.params.insert(
            "projectforce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projectforce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectforce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainangaccel(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainangaccel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainangaccel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainangaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainturnaccel(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainturnaccel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainturnaccel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainturnaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintiltaccel(mut self, val: bool) -> Self {
        self.params.insert(
            "constraintiltaccel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraintiltaccel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintiltaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updateup(mut self, val: bool) -> Self {
        self.params.insert(
            "updateup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updateup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updateup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updateparticles(mut self, val: bool) -> Self {
        self.params.insert(
            "updateparticles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updateparticles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updateparticles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doage(mut self, val: bool) -> Self {
        self.params.insert(
            "doage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doreapparticles(mut self, val: bool) -> Self {
        self.params.insert(
            "doreapparticles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doreapparticles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doreapparticles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_avoidance(mut self, val: bool) -> Self {
        self.params.insert(
            "avoidance".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_avoidance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "avoidance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showavoidanceguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showavoidanceguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showavoidanceguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showavoidanceguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookat(mut self, val: bool) -> Self {
        self.params.insert(
            "lookat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lookat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setnumjoints(mut self, val: bool) -> Self {
        self.params.insert(
            "setnumjoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setnumjoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setnumjoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitheadturn(mut self, val: bool) -> Self {
        self.params.insert(
            "limitheadturn".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitheadturn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitheadturn".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_immediateadjusthead(mut self, val: bool) -> Self {
        self.params.insert(
            "immediateadjusthead".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_immediateadjusthead_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "immediateadjusthead".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtargetnameattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtargetnameattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetnameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtargetnameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtargetposattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtargetposattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetposattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtargetposattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtargetscoreattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtargetscoreattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtargetscoreattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showterrainguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showterrainguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showterrainguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showterrainguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefootlocking(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefootlocking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefootlocking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefootlocking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjusthips(mut self, val: bool) -> Self {
        self.params.insert(
            "adjusthips".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjusthips_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adjusthips".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limithipshift(mut self, val: bool) -> Self {
        self.params.insert(
            "limithipshift".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limithipshift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limithipshift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablekneedamping(mut self, val: bool) -> Self {
        self.params.insert(
            "enablekneedamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablekneedamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablekneedamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainprojection(mut self, val: bool) -> Self {
        self.params.insert(
            "terrainprojection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_terrainprojection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainprojection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projectsamples(mut self, val: bool) -> Self {
        self.params.insert(
            "projectsamples".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projectsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sticktodeforminggeo(mut self, val: bool) -> Self {
        self.params.insert(
            "sticktodeforminggeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sticktodeforminggeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sticktodeforminggeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainadaptation(mut self, val: bool) -> Self {
        self.params.insert(
            "terrainadaptation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_terrainadaptation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainadaptation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableleaning(mut self, val: bool) -> Self {
        self.params.insert(
            "enableleaning".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableleaning_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableleaning".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_agentspeed(mut self, val: bool) -> Self {
        self.params.insert(
            "agentspeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_agentspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "agentspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_agentangularspeed(mut self, val: bool) -> Self {
        self.params.insert(
            "agentangularspeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_agentangularspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "agentangularspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quantize(mut self, val: bool) -> Self {
        self.params.insert(
            "quantize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_quantize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCrowdsolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "crowdsolver"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdstateRbdragdoll {
    Active = 0,
    AnimatedStatic = 1,
    Ignore = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdstateRagdollStiffnessvalue {
    Constant = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdstateRagdollMotorstrengthvalue {
    Constant = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdstateClipassignment {
    None = 0,
    SingleClip = 1,
    RandomDistribution = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdstateCliptype {
    /// In-Place
    InMinusPlace = 0,
    Locomotive = 1,
}

#[derive(Debug, Clone)]
pub struct DopCrowdstate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCrowdstate {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ragdoll_stiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "ragdoll_stiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_stiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffnessscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ragdoll_stiffnessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_stiffnessscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stiffnessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffnesscfm(mut self, val: f32) -> Self {
        self.params.insert(
            "ragdoll_stiffnesscfm".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_stiffnesscfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stiffnesscfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "ragdoll_motorstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_motorstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_motorstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorstrengthscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ragdoll_motorstrengthscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_motorstrengthscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_motorstrengthscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorcorrectiontime(mut self, val: f32) -> Self {
        self.params.insert(
            "ragdoll_motorcorrectiontime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_motorcorrectiontime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_motorcorrectiontime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorcfm(mut self, val: f32) -> Self {
        self.params.insert(
            "ragdoll_motorcfm".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_motorcfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_motorcfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_minstickycollisionimpulse(mut self, val: f32) -> Self {
        self.params.insert(
            "ragdoll_minstickycollisionimpulse".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_minstickycollisionimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_minstickycollisionimpulse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomclipnameseed(mut self, val: f32) -> Self {
        self.params.insert(
            "randomclipnameseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipnameseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomclipnameseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipweight_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("clipweight_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipweight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipweight_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globalclipoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "globalclipoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalclipoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalclipoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomclipoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "randomclipoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomclipoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomclipoffsetseed(mut self, val: f32) -> Self {
        self.params.insert(
            "randomclipoffsetseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipoffsetseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomclipoffsetseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipspeedmultiplier(mut self, val: f32) -> Self {
        self.params.insert(
            "clipspeedmultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedmultiplier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipspeedmultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_locomotionspeedmultiplier(mut self, val: f32) -> Self {
        self.params.insert(
            "locomotionspeedmultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_locomotionspeedmultiplier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "locomotionspeedmultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipspeedvariance(mut self, val: f32) -> Self {
        self.params.insert(
            "clipspeedvariance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedvariance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipspeedvariance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipspeedseed(mut self, val: f32) -> Self {
        self.params.insert(
            "clipspeedseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipspeedseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gaitspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "gaitspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gaitspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gaitspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speedvariance(mut self, val: f32) -> Self {
        self.params.insert(
            "speedvariance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_speedvariance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speedvariance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_ragdoll_stiffnessramprange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ragdoll_stiffnessramprange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ragdoll_stiffnessramprange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stiffnessramprange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorstrengthramprange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ragdoll_motorstrengthramprange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ragdoll_motorstrengthramprange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_motorstrengthramprange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_ragdoll_maxstickycollisionobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "ragdoll_maxstickycollisionobjects".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ragdoll_maxstickycollisionobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_maxstickycollisionobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_maxstickycollisionpoints(mut self, val: i32) -> Self {
        self.params.insert(
            "ragdoll_maxstickycollisionpoints".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ragdoll_maxstickycollisionpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_maxstickycollisionpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_rbdragdoll(mut self, val: DopCrowdstateRbdragdoll) -> Self {
        self.params.insert(
            "rbdragdoll".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rbdragdoll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rbdragdoll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffnessvalue(mut self, val: DopCrowdstateRagdollStiffnessvalue) -> Self {
        self.params.insert(
            "ragdoll_stiffnessvalue".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ragdoll_stiffnessvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stiffnessvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorstrengthvalue(
        mut self,
        val: DopCrowdstateRagdollMotorstrengthvalue,
    ) -> Self {
        self.params.insert(
            "ragdoll_motorstrengthvalue".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ragdoll_motorstrengthvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_motorstrengthvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipassignment(mut self, val: DopCrowdstateClipassignment) -> Self {
        self.params.insert(
            "clipassignment".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clipassignment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipassignment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cliptype(mut self, val: DopCrowdstateCliptype) -> Self {
        self.params.insert(
            "cliptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cliptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cliptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_ragdoll_stiffnessramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "ragdoll_stiffnessramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ragdoll_stiffnessramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stiffnessramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorstrengthramp(
        mut self,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            "ragdoll_motorstrengthramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ragdoll_motorstrengthramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_motorstrengthramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_streamname(mut self, val: &str) -> Self {
        self.params.insert(
            "streamname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_streamname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "streamname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_statename(mut self, val: &str) -> Self {
        self.params.insert(
            "statename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_statename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "statename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffnessgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "ragdoll_stiffnessgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffnessgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stiffnessgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_activegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "ragdoll_activegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdoll_activegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_activegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "ragdoll_motorgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdoll_motorgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_motorgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stickycollisiongroup(mut self, val: &str) -> Self {
        self.params.insert(
            "ragdoll_stickycollisiongroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stickycollisiongroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stickycollisiongroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stickycollisionignore(mut self, val: &str) -> Self {
        self.params.insert(
            "ragdoll_stickycollisionignore".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stickycollisionignore_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_stickycollisionignore".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname(mut self, val: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clippattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("clippattern_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clippattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clippattern_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localexpression(mut self, val: &str) -> Self {
        self.params.insert(
            "localexpression".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_localexpression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localexpression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_ragdoll_enablestiffness(mut self, val: bool) -> Self {
        self.params.insert(
            "ragdoll_enablestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_enablestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_enablestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_enablepartial(mut self, val: bool) -> Self {
        self.params.insert(
            "ragdoll_enablepartial".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_enablepartial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_enablepartial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_enablemotor(mut self, val: bool) -> Self {
        self.params.insert(
            "ragdoll_enablemotor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_enablemotor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_enablemotor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_enablestickycollisiongroup(mut self, val: bool) -> Self {
        self.params.insert(
            "ragdoll_enablestickycollisiongroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_enablestickycollisiongroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_enablestickycollisiongroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_setminstickycollisionimpulse(mut self, val: bool) -> Self {
        self.params.insert(
            "ragdoll_setminstickycollisionimpulse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_setminstickycollisionimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_setminstickycollisionimpulse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_setmaxstickycollisionobjects(mut self, val: bool) -> Self {
        self.params.insert(
            "ragdoll_setmaxstickycollisionobjects".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_setmaxstickycollisionobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_setmaxstickycollisionobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_setmaxstickycollisionpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "ragdoll_setmaxstickycollisionpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_setmaxstickycollisionpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_setmaxstickycollisionpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_setstickycollisionignore(mut self, val: bool) -> Self {
        self.params.insert(
            "ragdoll_setstickycollisionignore".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_setstickycollisionignore_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ragdoll_setstickycollisionignore".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomclipname(mut self, val: bool) -> Self {
        self.params.insert(
            "randomclipname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomclipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomclipname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomclipnametimedep(mut self, val: bool) -> Self {
        self.params.insert(
            "randomclipnametimedep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomclipnametimedep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomclipnametimedep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setinitialclipname(mut self, val: bool) -> Self {
        self.params.insert(
            "setinitialclipname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setinitialclipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setinitialclipname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setinitialcliptime(mut self, val: bool) -> Self {
        self.params.insert(
            "setinitialcliptime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setinitialcliptime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setinitialcliptime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomizecliptime(mut self, val: bool) -> Self {
        self.params.insert(
            "randomizecliptime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizecliptime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomizecliptime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applylocomotionorient(mut self, val: bool) -> Self {
        self.params.insert(
            "applylocomotionorient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applylocomotionorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applylocomotionorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomizeclipspeed(mut self, val: bool) -> Self {
        self.params.insert(
            "randomizeclipspeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizeclipspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomizeclipspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lastframematchesfirst(mut self, val: bool) -> Self {
        self.params.insert(
            "lastframematchesfirst".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lastframematchesfirst_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lastframematchesfirst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablelooping(mut self, val: bool) -> Self {
        self.params.insert(
            "enablelooping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablelooping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablelooping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_retimetoparticle(mut self, val: bool) -> Self {
        self.params.insert(
            "retimetoparticle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_retimetoparticle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "retimetoparticle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitparticlespeed(mut self, val: bool) -> Self {
        self.params.insert(
            "limitparticlespeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitparticlespeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitparticlespeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselocal(mut self, val: bool) -> Self {
        self.params.insert(
            "uselocal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselocal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCrowdstate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "crowdstate"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct DopCrowdtransition {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCrowdtransition {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Trigger or Triggerlogic"
    pub fn set_input_trigger_or_triggerlogic<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Trigger or Triggerlogic" and specifies the output index of the target node.
    pub fn set_input_trigger_or_triggerlogic_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_randomstartoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "randomstartoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomstartoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomstartoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomstartseed(mut self, val: f32) -> Self {
        self.params.insert(
            "randomstartseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomstartseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomstartseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_duration(mut self, val: f32) -> Self {
        self.params.insert(
            "duration".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_duration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "duration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxanimationtime(mut self, val: f32) -> Self {
        self.params.insert(
            "maxanimationtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxanimationtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxanimationtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_blend(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instate(mut self, val: &str) -> Self {
        self.params.insert(
            "instate".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outstate(mut self, val: &str) -> Self {
        self.params.insert(
            "outstate".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outstate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outstate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outclip(mut self, val: &str) -> Self {
        self.params.insert(
            "outclip".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outclip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outclip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localexpression(mut self, val: &str) -> Self {
        self.params.insert(
            "localexpression".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_localexpression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localexpression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transitiondescription(mut self, val: &str) -> Self {
        self.params.insert(
            "transitiondescription".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transitiondescription_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transitiondescription".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useoutclip(mut self, val: bool) -> Self {
        self.params.insert(
            "useoutclip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useoutclip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useoutclip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselocal(mut self, val: bool) -> Self {
        self.params.insert(
            "uselocal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselocal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomstart(mut self, val: bool) -> Self {
        self.params.insert(
            "randomstart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableinterrupt(mut self, val: bool) -> Self {
        self.params.insert(
            "enableinterrupt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableinterrupt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableinterrupt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detachfromparent(mut self, val: bool) -> Self {
        self.params.insert(
            "detachfromparent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_detachfromparent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detachfromparent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useclipgraph(mut self, val: bool) -> Self {
        self.params.insert(
            "useclipgraph".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useclipgraph_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useclipgraph".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animationmatch(mut self, val: bool) -> Self {
        self.params.insert(
            "animationmatch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animationmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animationmatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCrowdtransition {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "crowdtransition"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerType {
    ObjectBounds = 0,
    ObjectAttribute = 1,
    /// Object Distance (position)
    ObjectDistancePosition = 2,
    /// Object Distance (pointcloud)
    ObjectDistancePointcloud = 3,
    ObjectRaycast = 4,
    ParticleSpeed = 5,
    ParticleProximity = 6,
    ParticleAttributeLookup = 7,
    ParticleAttributeComparison = 8,
    /// Time (Current)
    TimeCurrent = 9,
    CurrentStateDuration = 10,
    AnimatedParameter = 11,
    CustomVexpression = 12,
    RbdImpactData = 13,
    AnimationClipLoops = 14,
    LookAtTarget = 15,
    RbdStickyCollisions = 16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerTriggersource {
    Sop = 0,
    DopData = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerBoundscondition {
    Incoming = 0,
    Outgoing = 1,
    Continuous = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerAttributecondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerDistto {
    Centroid = 0,
    ClosestPoint = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerDistancecondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerPointcloudcondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerRaycastdir {
    AgentOrientation = 0,
    AgentVelocity = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerRaycasthitcheck {
    Hit = 0,
    NoHit = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerSpeedcondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerAttributelookupcondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerParticleattributecomparisoncheck {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerTimeunits {
    Frames = 0,
    Seconds = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerGazedurationcomparison {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerTimeframecondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerStatedurationcondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerNumloopscondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerNumstickycollisionscondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerBindinputmenu1 {
    None = 0,
    Sop = 1,
    DopData = 2,
    Myself = 3,
    FirstContextGeometry = 4,
    SecondContextGeometry = 5,
    ThirdContextGeometry = 6,
    FourthContextGeometry = 7,
    /// Myself (No Reads of Outputs)
    MyselfNoReadsOfOutputs = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerBindinputmenu2 {
    None = 0,
    Sop = 1,
    DopData = 2,
    Myself = 3,
    FirstContextGeometry = 4,
    SecondContextGeometry = 5,
    ThirdContextGeometry = 6,
    FourthContextGeometry = 7,
    /// Myself (No Reads of Outputs)
    MyselfNoReadsOfOutputs = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerBindinputmenu3 {
    None = 0,
    Sop = 1,
    DopData = 2,
    Myself = 3,
    FirstContextGeometry = 4,
    SecondContextGeometry = 5,
    ThirdContextGeometry = 6,
    FourthContextGeometry = 7,
    /// Myself (No Reads of Outputs)
    MyselfNoReadsOfOutputs = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerBindinputmenu4 {
    None = 0,
    Sop = 1,
    DopData = 2,
    Myself = 3,
    FirstContextGeometry = 4,
    SecondContextGeometry = 5,
    ThirdContextGeometry = 6,
    FourthContextGeometry = 7,
    /// Myself (No Reads of Outputs)
    MyselfNoReadsOfOutputs = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerImpactcondition {
    LessThan = 0,
    Equal = 1,
    GreaterThan = 2,
}

#[derive(Debug, Clone)]
pub struct DopCrowdtrigger {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCrowdtrigger {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_attributesearchradius(mut self, val: f32) -> Self {
        self.params.insert(
            "attributesearchradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attributesearchradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributesearchradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributecomparisonvalue(mut self, val: f32) -> Self {
        self.params.insert(
            "attributecomparisonvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attributecomparisonvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributecomparisonvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "distance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointcloudsearchdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "pointcloudsearchdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointcloudsearchdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointcloudsearchdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointclouddistance(mut self, val: f32) -> Self {
        self.params.insert(
            "pointclouddistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointclouddistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointclouddistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raycastraylength(mut self, val: f32) -> Self {
        self.params.insert(
            "raycastraylength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_raycastraylength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raycastraylength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlespeed(mut self, val: f32) -> Self {
        self.params.insert(
            "particlespeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlespeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlespeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleproximitysearchradius(mut self, val: f32) -> Self {
        self.params.insert(
            "particleproximitysearchradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particleproximitysearchradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleproximitysearchradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleproximityfov(mut self, val: f32) -> Self {
        self.params.insert(
            "particleproximityfov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particleproximityfov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleproximityfov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributelookupcomparisonvalue(mut self, val: f32) -> Self {
        self.params.insert(
            "attributelookupcomparisonvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attributelookupcomparisonvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributelookupcomparisonvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleattributecomparisonsearchradius(mut self, val: f32) -> Self {
        self.params.insert(
            "particleattributecomparisonsearchradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particleattributecomparisonsearchradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleattributecomparisonsearchradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gazeduration(mut self, val: f32) -> Self {
        self.params.insert(
            "gazeduration".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gazeduration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gazeduration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeframe(mut self, val: f32) -> Self {
        self.params.insert(
            "timeframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeframerandom(mut self, val: f32) -> Self {
        self.params.insert(
            "timeframerandom".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timeframerandom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeframerandom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stateduration(mut self, val: f32) -> Self {
        self.params.insert(
            "stateduration".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stateduration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stateduration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactvalue(mut self, val: f32) -> Self {
        self.params.insert(
            "impactvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impactvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refdir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_particleattributearrayindex(mut self, val: i32) -> Self {
        self.params.insert(
            "particleattributearrayindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_particleattributearrayindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleattributearrayindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animatedparametervalue(mut self, val: i32) -> Self {
        self.params.insert(
            "animatedparametervalue".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_animatedparametervalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animatedparametervalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numloops(mut self, val: i32) -> Self {
        self.params.insert(
            "numloops".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numloops_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numloops".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numstickycollisions(mut self, val: i32) -> Self {
        self.params.insert(
            "numstickycollisions".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numstickycollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numstickycollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: DopCrowdtriggerType) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triggersource(mut self, val: DopCrowdtriggerTriggersource) -> Self {
        self.params.insert(
            "triggersource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_triggersource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triggersource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundscondition(mut self, val: DopCrowdtriggerBoundscondition) -> Self {
        self.params.insert(
            "boundscondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundscondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundscondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributecondition(mut self, val: DopCrowdtriggerAttributecondition) -> Self {
        self.params.insert(
            "attributecondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attributecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributecondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distto(mut self, val: DopCrowdtriggerDistto) -> Self {
        self.params.insert(
            "distto".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_distto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distto".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distancecondition(mut self, val: DopCrowdtriggerDistancecondition) -> Self {
        self.params.insert(
            "distancecondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_distancecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancecondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointcloudcondition(mut self, val: DopCrowdtriggerPointcloudcondition) -> Self {
        self.params.insert(
            "pointcloudcondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pointcloudcondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointcloudcondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raycastdir(mut self, val: DopCrowdtriggerRaycastdir) -> Self {
        self.params.insert(
            "raycastdir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_raycastdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raycastdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raycasthitcheck(mut self, val: DopCrowdtriggerRaycasthitcheck) -> Self {
        self.params.insert(
            "raycasthitcheck".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_raycasthitcheck_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raycasthitcheck".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speedcondition(mut self, val: DopCrowdtriggerSpeedcondition) -> Self {
        self.params.insert(
            "speedcondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_speedcondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speedcondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributelookupcondition(
        mut self,
        val: DopCrowdtriggerAttributelookupcondition,
    ) -> Self {
        self.params.insert(
            "attributelookupcondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attributelookupcondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributelookupcondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleattributecomparisoncheck(
        mut self,
        val: DopCrowdtriggerParticleattributecomparisoncheck,
    ) -> Self {
        self.params.insert(
            "particleattributecomparisoncheck".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_particleattributecomparisoncheck_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleattributecomparisoncheck".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeunits(mut self, val: DopCrowdtriggerTimeunits) -> Self {
        self.params.insert(
            "timeunits".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_timeunits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeunits".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gazedurationcomparison(
        mut self,
        val: DopCrowdtriggerGazedurationcomparison,
    ) -> Self {
        self.params.insert(
            "gazedurationcomparison".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_gazedurationcomparison_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gazedurationcomparison".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeframecondition(mut self, val: DopCrowdtriggerTimeframecondition) -> Self {
        self.params.insert(
            "timeframecondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_timeframecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeframecondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_statedurationcondition(
        mut self,
        val: DopCrowdtriggerStatedurationcondition,
    ) -> Self {
        self.params.insert(
            "statedurationcondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_statedurationcondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "statedurationcondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numloopscondition(mut self, val: DopCrowdtriggerNumloopscondition) -> Self {
        self.params.insert(
            "numloopscondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_numloopscondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numloopscondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numstickycollisionscondition(
        mut self,
        val: DopCrowdtriggerNumstickycollisionscondition,
    ) -> Self {
        self.params.insert(
            "numstickycollisionscondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_numstickycollisionscondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numstickycollisionscondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindinputmenu1(mut self, val: DopCrowdtriggerBindinputmenu1) -> Self {
        self.params.insert(
            "bindinputmenu1".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindinputmenu1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindinputmenu1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindinputmenu2(mut self, val: DopCrowdtriggerBindinputmenu2) -> Self {
        self.params.insert(
            "bindinputmenu2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindinputmenu2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindinputmenu2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindinputmenu3(mut self, val: DopCrowdtriggerBindinputmenu3) -> Self {
        self.params.insert(
            "bindinputmenu3".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindinputmenu3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindinputmenu3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindinputmenu4(mut self, val: DopCrowdtriggerBindinputmenu4) -> Self {
        self.params.insert(
            "bindinputmenu4".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindinputmenu4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindinputmenu4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactcondition(mut self, val: DopCrowdtriggerImpactcondition) -> Self {
        self.params.insert(
            "impactcondition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_impactcondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactcondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triggername(mut self, val: &str) -> Self {
        self.params.insert(
            "triggername".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_triggername_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triggername".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triggerattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "triggerattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_triggerattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triggerattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundingobject(mut self, val: &str) -> Self {
        self.params.insert(
            "boundingobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boundingobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundingobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundingobjectsop(mut self, val: &str) -> Self {
        self.params.insert(
            "boundingobjectsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boundingobjectsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundingobjectsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributeobject(mut self, val: &str) -> Self {
        self.params.insert(
            "attributeobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributeobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributeobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributeobjectsop(mut self, val: &str) -> Self {
        self.params.insert(
            "attributeobjectsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributeobjectsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributeobjectsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributename(mut self, val: &str) -> Self {
        self.params.insert(
            "attributename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distanceobject(mut self, val: &str) -> Self {
        self.params.insert(
            "distanceobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distanceobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distanceobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distanceobjectsop(mut self, val: &str) -> Self {
        self.params.insert(
            "distanceobjectsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distanceobjectsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distanceobjectsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointcloudobject(mut self, val: &str) -> Self {
        self.params.insert(
            "pointcloudobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointcloudobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointcloudobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointcloudobjectsop(mut self, val: &str) -> Self {
        self.params.insert(
            "pointcloudobjectsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointcloudobjectsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointcloudobjectsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raycastobject(mut self, val: &str) -> Self {
        self.params.insert(
            "raycastobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_raycastobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raycastobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raycastbjectsop(mut self, val: &str) -> Self {
        self.params.insert(
            "raycastbjectsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_raycastbjectsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raycastbjectsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleproximitygroupattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "particleproximitygroupattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_particleproximitygroupattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleproximitygroupattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleattributelookupname(mut self, val: &str) -> Self {
        self.params.insert(
            "particleattributelookupname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_particleattributelookupname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleattributelookupname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleattributecomparisonname(mut self, val: &str) -> Self {
        self.params.insert(
            "particleattributecomparisonname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_particleattributecomparisonname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleattributecomparisonname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleattributecomparisongroupattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "particleattributecomparisongroupattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_particleattributecomparisongroupattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleattributecomparisongroupattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetnames(mut self, val: &str) -> Self {
        self.params.insert(
            "targetnames".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetnames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpointgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpointgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickycollisionsxformgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "stickycollisionsxformgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stickycollisionsxformgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickycollisionsxformgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_custom_snippet(mut self, val: &str) -> Self {
        self.params.insert(
            "custom_snippet".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_custom_snippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "custom_snippet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput1(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput1(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput2(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput2(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput3(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput3(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput4(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput4(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath(mut self, val: &str) -> Self {
        self.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "impactobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_impactobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactxformgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "impactxformgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_impactxformgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactxformgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fuzzyoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "fuzzyoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fuzzyoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fuzzyoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlespeedangulartoggle(mut self, val: bool) -> Self {
        self.params.insert(
            "particlespeedangulartoggle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_particlespeedangulartoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlespeedangulartoggle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleproxmityusefov(mut self, val: bool) -> Self {
        self.params.insert(
            "particleproxmityusefov".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_particleproxmityusefov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleproxmityusefov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleproximityignoregroup(mut self, val: bool) -> Self {
        self.params.insert(
            "particleproximityignoregroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_particleproximityignoregroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleproximityignoregroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useparticleattributearrayindex(mut self, val: bool) -> Self {
        self.params.insert(
            "useparticleattributearrayindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useparticleattributearrayindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useparticleattributearrayindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particleattributecomparisonignoregroup(mut self, val: bool) -> Self {
        self.params.insert(
            "particleattributecomparisonignoregroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_particleattributecomparisonignoregroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particleattributecomparisonignoregroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usegazeduration(mut self, val: bool) -> Self {
        self.params.insert(
            "usegazeduration".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegazeduration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegazeduration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopuseself1(mut self, val: bool) -> Self {
        self.params.insert(
            "binddopuseself1".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_binddopuseself1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopuseself1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopuseself2(mut self, val: bool) -> Self {
        self.params.insert(
            "binddopuseself2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_binddopuseself2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopuseself2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopuseself3(mut self, val: bool) -> Self {
        self.params.insert(
            "binddopuseself3".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_binddopuseself3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopuseself3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopuseself4(mut self, val: bool) -> Self {
        self.params.insert(
            "binddopuseself4".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_binddopuseself4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopuseself4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCrowdtrigger {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "crowdtrigger"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopCrowdtriggerlogicOperation {
    And = 0,
    Or = 1,
    NotInput1 = 2,
    NotInput2 = 3,
    Xor = 4,
    Nand = 5,
    Nor = 6,
}

#[derive(Debug, Clone)]
pub struct DopCrowdtriggerlogic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopCrowdtriggerlogic {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Trigger or Triggerlogic"
    pub fn set_input_trigger_or_triggerlogic<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Trigger or Triggerlogic" and specifies the output index of the target node.
    pub fn set_input_trigger_or_triggerlogic_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Trigger or Triggerlogic"
    pub fn set_input_trigger_or_triggerlogic_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Trigger or Triggerlogic" and specifies the output index of the target node.
    pub fn set_input_trigger_or_triggerlogic_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_operation(mut self, val: DopCrowdtriggerlogicOperation) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_operation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_triggerattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "triggerattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_triggerattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triggerattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopCrowdtriggerlogic {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "crowdtriggerlogic"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
