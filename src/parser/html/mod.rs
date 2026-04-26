mod entity;
mod error;
mod lexer;
mod parser;
mod value;
pub mod writer;

pub use error::{HtmlError, HtmlErrorKind, HtmlErrorPosition};
pub use parser::{
    DEFAULT_HTML_LIMITS, HtmlLimits, HtmlParser, parse_html, parse_html_with_limits,
    parse_html_with_max_depth, validate_html,
};
pub use value::HtmlValue;
pub use writer::{push_html_escaped, push_html_row, push_js_escaped};
