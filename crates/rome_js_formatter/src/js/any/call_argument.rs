//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyCallArgument;
impl Format for JsAnyCallArgument {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyExpression(node) => node.format(formatter),
            Self::JsSpread(node) => node.format(formatter),
        }
    }
}