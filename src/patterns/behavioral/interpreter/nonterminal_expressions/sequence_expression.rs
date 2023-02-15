use crate::patterns::behavioral::interpreter::abstract_expression::AbstractExpression;
use crate::patterns::behavioral::interpreter::mask::Mask;
use crate::patterns::behavioral::interpreter::nonterminal_expression::NonterminalExpression;
use crate::patterns::behavioral::interpreter::context::Context;

pub struct SequenceExpression {
    sequence: Vec<Box<dyn AbstractExpression>>,
}

impl SequenceExpression {
    pub fn new(sequence: Vec<Box<dyn AbstractExpression>>) -> Self {
        Self {
            sequence,
        }
    }
}

impl AbstractExpression for SequenceExpression {
    fn interpret(&self, context: &Context) -> Mask {
        let mut mask: Option<Mask> = None;
        for sub_expression in &self.sequence {
            let new_mask = sub_expression.interpret(context);
            
            match &mask {
                Some(m) => {
                    let splitted_mask = &m.split();
                    let splitted_new_mask = &new_mask.split();

                    let mut masks: Vec<Mask> = vec![];
                    for sm in splitted_mask {
                        for snm in splitted_new_mask {
                            let appended_mask = sm.append(snm);
                            match appended_mask {
                                Some(am) => {
                                    masks.push(am);
                                },
                                None => {},
                            };
                        }
                    }

                    mask = Mask::union(&masks); // может вернуть None, тогда итоговый unwrap() вызовет ошибку
                },
                None => {
                    mask = Some(new_mask.clone());
                },
            };
        }

        mask.unwrap()
    }
}

impl NonterminalExpression for SequenceExpression {
}