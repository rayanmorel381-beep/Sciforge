#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TomlValue<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Table,
    Array,
    ArrayOfTables,
}

impl<'a> TomlValue<'a> {
    pub const fn is_table(&self) -> bool {
        matches!(self, TomlValue::Table)
    }

    pub const fn is_array(&self) -> bool {
        matches!(self, TomlValue::Array)
    }

    pub const fn as_str(&self) -> Option<&str> {
        match self {
            TomlValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            TomlValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub const fn as_integer(&self) -> Option<i64> {
        match self {
            TomlValue::Integer(v) => Some(*v),
            _ => None,
        }
    }

    pub const fn as_float(&self) -> Option<f64> {
        match self {
            TomlValue::Float(v) => Some(*v),
            _ => None,
        }
    }
}
