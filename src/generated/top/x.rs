#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopXmlinputXmlsource {
    UpstreamXmlFile = 0,
    CustomFilePath = 1,
    UpstreamXmlAttribute = 2,
    CustomString = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopXmlinputQuerymethod {
    Xpath = 0,
    ByElementName = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopXmlinputMatchdepth {
    AnyElements = 0,
    ChildElements = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopXmlinputElemextract {
    Element = 0,
    ElementText = 1,
    Attribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopXmlinputFailmode {
    AddWarning = 0,
    RaiseError = 1,
    GenerateItem = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopXmlinputCombinemultimatches {
    ForEachMatch = 0,
    ForAllMatches = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopXmlinputMatchtype {
    Element = 0,
    String = 1,
    Integer = 2,
    Float = 3,
    File = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopXmlinputStoragetype {
    AsStringAttribute = 0,
    WriteToFile = 1,
}

#[derive(Debug, Clone)]
pub struct TopXmlinput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopXmlinput {
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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_failmode(mut self, val: TopXmlinputFailmode) -> Self {
        self.params.insert(
            "failmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_failmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "failmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xmlsource(mut self, val: TopXmlinputXmlsource) -> Self {
        self.params.insert(
            "xmlsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xmlsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xmlsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_querymethod(mut self, val: TopXmlinputQuerymethod) -> Self {
        self.params.insert(
            "querymethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_querymethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "querymethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchdepth(mut self, val: TopXmlinputMatchdepth) -> Self {
        self.params.insert(
            "matchdepth".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elemextract(mut self, val: TopXmlinputElemextract) -> Self {
        self.params.insert(
            "elemextract".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_elemextract_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elemextract".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinemultimatches(mut self, val: TopXmlinputCombinemultimatches) -> Self {
        self.params.insert(
            "combinemultimatches".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_combinemultimatches_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinemultimatches".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchtype(mut self, val: TopXmlinputMatchtype) -> Self {
        self.params.insert(
            "matchtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_storagetype(mut self, val: TopXmlinputStoragetype) -> Self {
        self.params.insert(
            "storagetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_storagetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "storagetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcefiletag(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcefiletag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcefiletag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefiletag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xmlfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "xmlfilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xmlfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xmlfilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customxml(mut self, val: &str) -> Self {
        self.params.insert(
            "customxml".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customxml_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customxml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xpathquery(mut self, val: &str) -> Self {
        self.params.insert(
            "xpathquery".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xpathquery_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xpathquery".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementtag(mut self, val: &str) -> Self {
        self.params.insert(
            "elementtag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementtag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributetag(mut self, val: &str) -> Self {
        self.params.insert(
            "attributetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datatag(mut self, val: &str) -> Self {
        self.params.insert(
            "datatag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datatag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datatag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementoutputpath(mut self, val: &str) -> Self {
        self.params.insert(
            "elementoutputpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementoutputpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementoutputpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementoutputroot(mut self, val: &str) -> Self {
        self.params.insert(
            "elementoutputroot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementoutputroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementoutputroot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_attributeon(mut self, val: bool) -> Self {
        self.params.insert(
            "attributeon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attributeon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributeon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_asxpath(mut self, val: bool) -> Self {
        self.params.insert(
            "asxpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_asxpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "asxpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopXmlinput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "xmlinput"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
