use crate::patterns::behavioral::interpreter::abstract_expression::AbstractExpression;
use crate::patterns::behavioral::interpreter::mask::Mask;
use crate::patterns::behavioral::interpreter::terminal_expression::TerminalExpression;
use crate::patterns::behavioral::interpreter::context::Context;

pub struct GreenMaribozuCandlestick {
}

impl AbstractExpression for GreenMaribozuCandlestick {
    fn interpret(&self, context: &Context) -> Mask {
        let mut flags = vec![];
        for i in 0..context.len() {
            let p = context.lookup(i);
            flags.push(p.open == p.low && p.open < p.close && p.close == p.high);
        }
        Mask { flags }
    }
}

impl TerminalExpression for GreenMaribozuCandlestick {
}