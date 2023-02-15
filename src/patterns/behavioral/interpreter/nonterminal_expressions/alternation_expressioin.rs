use crate::patterns::behavioral::interpreter::abstract_expression::AbstractExpression;
use crate::patterns::behavioral::interpreter::mask::Mask;
use crate::patterns::behavioral::interpreter::nonterminal_expression::NonterminalExpression;
use crate::patterns::behavioral::interpreter::context::Context;

pub struct AlternationExpression {
    expression1: Box<dyn AbstractExpression>,
    expression2: Box<dyn AbstractExpression>,
}

impl AlternationExpression {
    pub fn new(expression1: Box<dyn AbstractExpression>, expression2: Box<dyn AbstractExpression>) -> Self {
        Self {
            expression1,
            expression2,
        }
    }
}

impl AbstractExpression for AlternationExpression {
    fn interpret(&self, context: &Context) -> Mask {
        let m1 = self.expression1.interpret(context);
        let m2 = self.expression2.interpret(context);

        let mask = Mask::union(&Vec::from([m1, m2])); // incorret alternation for example
        mask.unwrap()
    }
}

impl NonterminalExpression for AlternationExpression {
}