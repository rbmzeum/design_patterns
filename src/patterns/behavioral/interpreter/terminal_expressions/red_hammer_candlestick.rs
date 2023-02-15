use crate::patterns::behavioral::interpreter::abstract_expression::AbstractExpression;
use crate::patterns::behavioral::interpreter::mask::Mask;
use crate::patterns::behavioral::interpreter::terminal_expression::TerminalExpression;
use crate::patterns::behavioral::interpreter::context::Context;

pub struct RedHammerCandlestick {
}

impl AbstractExpression for RedHammerCandlestick {
    fn interpret(&self, context: &Context) -> Mask {
        let mut flags = vec![];
        for i in 0..context.len() {
            let p = context.lookup(i);
            flags.push(p.close > p.low && p.close - p.low < 2.0 * (p.high - p.close));
        }
        Mask { flags }
    }
}

impl TerminalExpression for RedHammerCandlestick {
}