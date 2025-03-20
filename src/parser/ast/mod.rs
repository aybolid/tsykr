mod expr_identifier;
mod program;

use std::fmt::Debug;

pub use expr_identifier::*;
pub use program::*;

pub trait Node
where
    Self: ToString,
    Self: Debug,
{
    /// Returns a token literal for the node.
    fn token_literal(&self) -> &str;
}
