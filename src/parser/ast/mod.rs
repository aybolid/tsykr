mod expr_boolean;
mod expr_float;
mod expr_function;
mod expr_identifier;
mod expr_infixed;
mod expr_integer;
mod expr_prefixed;
mod program;
mod stmt_block;
mod stmt_expr;
mod stmt_function;
mod stmt_let;
mod stmt_return;

use std::fmt::Debug;

pub use expr_boolean::*;
pub use expr_float::*;
pub use expr_function::*;
pub use expr_identifier::*;
pub use expr_infixed::*;
pub use expr_integer::*;
pub use expr_prefixed::*;
pub use program::*;
pub use stmt_block::*;
pub use stmt_expr::*;
pub use stmt_function::*;
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
