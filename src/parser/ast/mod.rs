mod expr_identifier;
mod program;
mod stmt_let;
mod stmt_return;

use std::fmt::Debug;

pub use expr_identifier::*;
pub use program::*;
pub use stmt_let::*;
pub use stmt_return::*;

pub trait Node
where
    Self: ToString,
    Self: Debug,
{
    /// Returns a token literal for the node.
    #[allow(unused)]
    fn token_literal(&self) -> &str;
    #[allow(unused)]
    fn as_any(&self) -> &dyn std::any::Any;
}
