use super::context::Context;
use super::mask::Mask;

pub trait AbstractExpression {
    fn interpret(&self, context: &Context) -> Mask;
}