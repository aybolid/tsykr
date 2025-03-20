mod expr_identifier;
mod program;
mod stmt_expr;
mod stmt_let;
mod stmt_return;

use std::fmt::Debug;

pub use expr_identifier::*;
pub use program::*;
pub use stmt_expr::*;
pub use stmt_let::*;
pub use stmt_return::*;

pub trait Node
where
    Self: ToString,
    Self: Debug,
{
    /// Returns a token literal of the node.
    #[allow(unused)]
    fn token_literal(&self) -> String;
    #[allow(unused)]
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Statement
where
    Self: Node,
{
}

pub trait Expression
where
    Self: Node,
{
}
