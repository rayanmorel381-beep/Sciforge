#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HtmlValue<'a> {
    Document,
    Element,
    Text(&'a str),
    Comment,
    Doctype,
}

impl<'a> HtmlValue<'a> {
    pub const fn is_document(&self) -> bool {
        matches!(self, HtmlValue::Document)
    }

    pub const fn is_element(&self) -> bool {
        matches!(self, HtmlValue::Element)
    }

    pub const fn is_comment(&self) -> bool {
        matches!(self, HtmlValue::Comment)
    }

    pub const fn is_doctype(&self) -> bool {
        matches!(self, HtmlValue::Doctype)
    }

    pub const fn as_text(&self) -> Option<&str> {
        match self {
            HtmlValue::Text(v) => Some(v),
            _ => None,
        }
    }
}
