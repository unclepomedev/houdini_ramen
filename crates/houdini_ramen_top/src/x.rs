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
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopXmlinput {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input1".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_failmode(mut self, val: TopXmlinputFailmode) -> Self {
        self.params.insert(
            "failmode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_failmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "failmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xmlsource(mut self, val: TopXmlinputXmlsource) -> Self {
        self.params.insert(
            "xmlsource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xmlsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xmlsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_querymethod(mut self, val: TopXmlinputQuerymethod) -> Self {
        self.params.insert(
            "querymethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_querymethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "querymethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_matchdepth(mut self, val: TopXmlinputMatchdepth) -> Self {
        self.params.insert(
            "matchdepth".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchdepth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elemextract(mut self, val: TopXmlinputElemextract) -> Self {
        self.params.insert(
            "elemextract".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_elemextract_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elemextract".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_combinemultimatches(mut self, val: TopXmlinputCombinemultimatches) -> Self {
        self.params.insert(
            "combinemultimatches".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_combinemultimatches_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinemultimatches".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_matchtype(mut self, val: TopXmlinputMatchtype) -> Self {
        self.params.insert(
            "matchtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_storagetype(mut self, val: TopXmlinputStoragetype) -> Self {
        self.params.insert(
            "storagetype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_storagetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "storagetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcefiletag(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcefiletag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcefiletag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefiletag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xmlfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "xmlfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xmlfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xmlfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customxml(mut self, val: &str) -> Self {
        self.params.insert(
            "customxml".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customxml_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customxml".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xpathquery(mut self, val: &str) -> Self {
        self.params.insert(
            "xpathquery".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xpathquery_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xpathquery".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementtag(mut self, val: &str) -> Self {
        self.params.insert(
            "elementtag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementtag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attributetag(mut self, val: &str) -> Self {
        self.params.insert(
            "attributetag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attributetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributetag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datatag(mut self, val: &str) -> Self {
        self.params.insert(
            "datatag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datatag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datatag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementoutputpath(mut self, val: &str) -> Self {
        self.params.insert(
            "elementoutputpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementoutputpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementoutputpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementoutputroot(mut self, val: &str) -> Self {
        self.params.insert(
            "elementoutputroot".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementoutputroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementoutputroot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attributeon(mut self, val: bool) -> Self {
        self.params.insert(
            "attributeon".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attributeon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributeon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_asxpath(mut self, val: bool) -> Self {
        self.params.insert(
            "asxpath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_asxpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "asxpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopXmlinput {
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
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait TopXmlinputOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopXmlinputOutputs for TopXmlinput {}
impl TopXmlinputOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopXmlinput> {}

pub trait TopXmlinputWiringExt {
    fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> TopXmlinputWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, TopXmlinput>
{
    fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("input1", output)
    }
}
