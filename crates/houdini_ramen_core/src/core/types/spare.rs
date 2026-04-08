#[derive(Debug, Clone, PartialEq)]
pub enum SpareParam {
    Float {
        name: String,
        label: String,
        default: f32,
        min: f32,
        max: f32,
    },
    Int {
        name: String,
        label: String,
        default: i32,
        min: i32,
        max: i32,
    },
    String {
        name: String,
        label: String,
        default: String,
    },
    Toggle {
        name: String,
        label: String,
        default: bool,
    },
    Color {
        name: String,
        label: String,
        default: [f32; 3],
    },
    Button {
        name: String,
        label: String,
    },
    Menu {
        name: String,
        label: String,
        default: i32,
        menu_items: Vec<(String, String)>,
    },
    File {
        name: String,
        label: String,
        default: String,
    },
    NodePath {
        name: String,
        label: String,
        default: String,
    },
    RampFloat {
        name: String,
        label: String,
    },
    RampColor {
        name: String,
        label: String,
    },
}

pub struct SpareFloat {
    name: String,
    label: String,
    default: f32,
    min: f32,
    max: f32,
}
impl SpareFloat {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: 0.0,
            min: 0.0,
            max: 1.0,
        }
    }
    pub fn with_default(mut self, val: f32) -> Self {
        self.default = val;
        self
    }
    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}

impl From<SpareFloat> for SpareParam {
    fn from(v: SpareFloat) -> Self {
        assert!(
            v.min <= v.max,
            "SpareFloat '{}' has invalid range: min {} > max {}",
            v.name,
            v.min,
            v.max
        );
        assert!(
            v.default >= v.min && v.default <= v.max,
            "SpareFloat '{}' default {} is out of range [{}, {}]",
            v.name,
            v.default,
            v.min,
            v.max
        );
        SpareParam::Float {
            name: v.name,
            label: v.label,
            default: v.default,
            min: v.min,
            max: v.max,
        }
    }
}

pub struct SpareInt {
    name: String,
    label: String,
    default: i32,
    min: i32,
    max: i32,
}
impl SpareInt {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: 0,
            min: 0,
            max: 10,
        }
    }
    pub fn with_default(mut self, val: i32) -> Self {
        self.default = val;
        self
    }
    pub fn with_range(mut self, min: i32, max: i32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}
impl From<SpareInt> for SpareParam {
    fn from(v: SpareInt) -> Self {
        assert!(
            v.min <= v.max,
            "SpareInt '{}' has invalid range: min {} > max {}",
            v.name,
            v.min,
            v.max
        );
        assert!(
            v.default >= v.min && v.default <= v.max,
            "SpareInt '{}' default {} is out of range [{}, {}]",
            v.name,
            v.default,
            v.min,
            v.max
        );
        SpareParam::Int {
            name: v.name,
            label: v.label,
            default: v.default,
            min: v.min,
            max: v.max,
        }
    }
}

pub struct SpareString {
    name: String,
    label: String,
    default: String,
}
impl SpareString {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: String::new(),
        }
    }
    pub fn with_default(mut self, val: &str) -> Self {
        self.default = val.into();
        self
    }
}
impl From<SpareString> for SpareParam {
    fn from(v: SpareString) -> Self {
        SpareParam::String {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareToggle {
    name: String,
    label: String,
    default: bool,
}
impl SpareToggle {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: false,
        }
    }
    pub fn with_default(mut self, val: bool) -> Self {
        self.default = val;
        self
    }
}
impl From<SpareToggle> for SpareParam {
    fn from(v: SpareToggle) -> Self {
        SpareParam::Toggle {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareColor {
    name: String,
    label: String,
    default: [f32; 3],
}
impl SpareColor {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: [1.0, 1.0, 1.0],
        }
    }
    pub fn with_default(mut self, val: [f32; 3]) -> Self {
        self.default = val;
        self
    }
}
impl From<SpareColor> for SpareParam {
    fn from(v: SpareColor) -> Self {
        SpareParam::Color {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareButton {
    name: String,
    label: String,
}
impl SpareButton {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
        }
    }
}
impl From<SpareButton> for SpareParam {
    fn from(v: SpareButton) -> Self {
        SpareParam::Button {
            name: v.name,
            label: v.label,
        }
    }
}

pub struct SpareMenu {
    name: String,
    label: String,
    default: i32,
    menu_items: Vec<(String, String)>,
}
impl SpareMenu {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: 0,
            menu_items: vec![],
        }
    }
    pub fn with_default(mut self, val: i32) -> Self {
        self.default = val;
        self
    }
    pub fn add_item(mut self, key: &str, label: &str) -> Self {
        self.menu_items.push((key.into(), label.into()));
        self
    }
}
impl From<SpareMenu> for SpareParam {
    fn from(v: SpareMenu) -> Self {
        assert!(
            !v.menu_items.is_empty(),
            "SpareMenu '{}' menu_items cannot be empty",
            v.name
        );
        assert!(
            v.default >= 0 && (v.default as usize) < v.menu_items.len(),
            "SpareMenu '{}' default_value {} is out of range [0, {})",
            v.name,
            v.default,
            v.menu_items.len()
        );
        SpareParam::Menu {
            name: v.name,
            label: v.label,
            default: v.default,
            menu_items: v.menu_items,
        }
    }
}

pub struct SpareFile {
    name: String,
    label: String,
    default: String,
}
impl SpareFile {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: String::new(),
        }
    }
    pub fn with_default(mut self, val: &str) -> Self {
        self.default = val.into();
        self
    }
}
impl From<SpareFile> for SpareParam {
    fn from(v: SpareFile) -> Self {
        SpareParam::File {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareNodePath {
    name: String,
    label: String,
    default: String,
}
impl SpareNodePath {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: String::new(),
        }
    }
    pub fn with_default(mut self, val: &str) -> Self {
        self.default = val.into();
        self
    }
}
impl From<SpareNodePath> for SpareParam {
    fn from(v: SpareNodePath) -> Self {
        SpareParam::NodePath {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareRampFloat {
    name: String,
    label: String,
}
impl SpareRampFloat {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
        }
    }
}
impl From<SpareRampFloat> for SpareParam {
    fn from(v: SpareRampFloat) -> Self {
        SpareParam::RampFloat {
            name: v.name,
            label: v.label,
        }
    }
}

pub struct SpareRampColor {
    name: String,
    label: String,
}
impl SpareRampColor {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
        }
    }
}
impl From<SpareRampColor> for SpareParam {
    fn from(v: SpareRampColor) -> Self {
        SpareParam::RampColor {
            name: v.name,
            label: v.label,
        }
    }
}
