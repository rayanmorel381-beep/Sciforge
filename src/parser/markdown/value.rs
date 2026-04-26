#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MdValue<'a> {
    Document,
    Heading(u8),
    Paragraph,
    CodeBlock,
    ThematicBreak,
    BlockQuote,
    List,
    Table,
    Text(&'a str),
}

impl<'a> MdValue<'a> {
    pub const fn is_document(&self) -> bool {
        matches!(self, MdValue::Document)
    }

    pub const fn is_heading(&self) -> bool {
        matches!(self, MdValue::Heading(..))
    }

    pub const fn heading_level(&self) -> Option<u8> {
        match self {
            MdValue::Heading(lvl) => Some(*lvl),
            _ => None,
        }
    }

    pub const fn is_code_block(&self) -> bool {
        matches!(self, MdValue::CodeBlock)
    }

    pub const fn is_list(&self) -> bool {
        matches!(self, MdValue::List)
    }

    pub const fn is_table(&self) -> bool {
        matches!(self, MdValue::Table)
    }

    pub const fn as_text(&self) -> Option<&str> {
        match self {
            MdValue::Text(v) => Some(v),
            _ => None,
        }
    }
}
