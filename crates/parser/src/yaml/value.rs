#[derive(Clone, Debug, PartialEq)]
pub enum YamlValue<'a> {
    Null,
    Bool(bool),
    Number(f64),
    String(&'a str),
    Sequence(Vec<YamlValue<'a>>),
    Mapping(Vec<(&'a str, YamlValue<'a>)>),
}

impl<'a> YamlValue<'a> {
    pub fn is_mapping(&self) -> bool {
        matches!(self, YamlValue::Mapping(_))
    }

    pub fn is_sequence(&self) -> bool {
        matches!(self, YamlValue::Sequence(_))
    }

    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            YamlValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            YamlValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            YamlValue::Number(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        self.as_number().map(|n| n as u32)
    }

    pub fn get(&self, key: &str) -> Option<&YamlValue<'a>> {
        match self {
            YamlValue::Mapping(entries) => entries.iter().find(|(k, _)| *k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn items(&self) -> &[YamlValue<'a>] {
        match self {
            YamlValue::Sequence(items) => items,
            _ => &[],
        }
    }

    pub fn entries(&self) -> &[(&'a str, YamlValue<'a>)] {
        match self {
            YamlValue::Mapping(entries) => entries,
            _ => &[],
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, YamlValue::Null)
    }
}
