use ruff_formatter::prelude::*;
use ruff_formatter::{write, Format};
use ruff_text_size::{TextRange, TextSize};

use crate::context::ASTFormatContext;
use crate::core::types::Range;
use crate::cst::Stmt;
use crate::shared_traits::AsFormat;

#[derive(Copy, Clone)]
pub struct Block<'a> {
    body: &'a [Stmt],
}

impl Format<ASTFormatContext<'_>> for Block<'_> {
    fn fmt(&self, f: &mut Formatter<ASTFormatContext<'_>>) -> FormatResult<()> {
        for (i, stmt) in self.body.iter().enumerate() {
            if i > 0 {
                write!(f, [hard_line_break()])?;
            }
            write!(f, [stmt.format()])?;
        }
        Ok(())
    }
}

#[inline]
pub fn block(body: &[Stmt]) -> Block {
    Block { body }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Literal {
    range: Range,
}

impl Format<ASTFormatContext<'_>> for Literal {
    fn fmt(&self, f: &mut Formatter<ASTFormatContext<'_>>) -> FormatResult<()> {
        let (text, start, end) = f.context().locator().slice(self.range);
        f.write_element(FormatElement::StaticTextSlice {
            text,
            range: TextRange::new(start.try_into().unwrap(), end.try_into().unwrap()),
        })
    }
}

#[inline]
pub const fn literal(range: Range) -> Literal {
    Literal { range }
}

pub(crate) const fn join_names(names: &[String]) -> JoinNames {
    JoinNames { names }
}

pub(crate) struct JoinNames<'a> {
    names: &'a [String],
}

impl<Context> Format<Context> for JoinNames<'_> {
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let mut join = f.join_with(text(", "));
        for name in self.names {
            join.entry(&dynamic_text(name, TextSize::default()));
        }
        join.finish()
    }
}
