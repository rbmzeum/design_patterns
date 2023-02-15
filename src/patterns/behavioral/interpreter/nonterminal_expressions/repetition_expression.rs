use crate::patterns::behavioral::interpreter::abstract_expression::AbstractExpression;
use crate::patterns::behavioral::interpreter::mask::Mask;
use crate::patterns::behavioral::interpreter::nonterminal_expression::NonterminalExpression;
use crate::patterns::behavioral::interpreter::context::Context;

pub struct RepetitionExpression {
    expression: Box<dyn AbstractExpression>,
}

impl RepetitionExpression {
    pub fn new(expression: Box<dyn AbstractExpression>) -> Self {
        Self {
            expression,
        }
    }
}

impl AbstractExpression for RepetitionExpression {
    fn interpret(&self, context: &Context) -> Mask {
        let mask = self.expression.interpret(context);
        // current implementation of repetition is not correct
        // ищет все вхождения выражения в контексте, поэтому последовательность автоматически будет добавлена в маску
        mask
    }
}

impl NonterminalExpression for RepetitionExpression {
}