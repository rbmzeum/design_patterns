pub mod price;
pub mod context;
pub mod mask;
pub mod abstract_expression;
pub mod terminal_expression;
pub mod nonterminal_expression;
pub mod terminal_expressions;
pub mod nonterminal_expressions;

// Example
pub fn behavioral_interpreter() {
    use price::Price;
    use context::Context;

    use crate::patterns::behavioral::interpreter::{
        nonterminal_expressions::{
            sequence_expression::SequenceExpression,
            repetition_expression::RepetitionExpression,
            alternation_expressioin::AlternationExpression
        },
        terminal_expressions::{
            green_candlestick::GreenCandlestick,
            green_hammer_candlestick::GreenHammerCandlestick,
            green_maribozu_candlestick::GreenMaribozuCandlestick,
            red_hammer_candlestick::RedHammerCandlestick,
            doji_candlestick::DojiCandlestick
        },
        abstract_expression::AbstractExpression
    };

    let prices = vec![
        Price { open: 5.5, high: 6.0, low: 4.0, close: 4.5, volume: 1.0 }, // RedCandlestick
        Price { open: 4.5, high: 5.0, low: 2.0, close: 5.0, volume: 1.0 }, // GreenHammerCandlestick
        Price { open: 5.0, high: 6.0, low: 5.0, close: 6.0, volume: 1.0 }, // GreenMaribozuCandlestick
        Price { open: 6.0, high: 7.0, low: 6.0, close: 7.0, volume: 1.0 }, // GreenMaribozuCandlestick
        Price { open: 7.0, high: 8.0, low: 6.0, close: 7.0, volume: 1.0 }, // DojiCandlestick
        Price { open: 7.0, high: 7.0, low: 5.0, close: 5.0, volume: 1.0 }, // RedMaribozuCandlestick
        Price { open: 5.0, high: 5.0, low: 4.0, close: 4.0, volume: 1.0 }, // RedMaribozuCandlestick
        Price { open: 4.0, high: 4.0, low: 2.0, close: 2.0, volume: 1.0 }, // RedMaribozuCandlestick
    ];

    let mut context = Context::new();

    // FIXME: nonterminal_expressions (данная в примере реализация не пригодна для использования в реальных проектах, требуется доработка)
    // expression = [((GreenCandlestick|GreenHammerCandlestick)|GreenMaribozuCandlestick)+, (RedHammerCandlestick|DojiCandlestick)]
    let expression = SequenceExpression::new(vec![
        Box::new(RepetitionExpression::new(
            Box::new(AlternationExpression::new(
                Box::new(AlternationExpression::new(
                    Box::new(GreenCandlestick{}),
                    Box::new(GreenHammerCandlestick{})
                )),
                Box::new(GreenMaribozuCandlestick{})
            )),
        )),
        Box::new(AlternationExpression::new(
            Box::new(RedHammerCandlestick{}),
            Box::new(DojiCandlestick{})
        )),
    ]);

    context.assign(prices);
    let result = expression.interpret(&context);
    println!("Mask: {:#?}", result); // FIXME: ожидается 01111000, а получаем 11111100 из-за некорректной реализации интерпретаторов не терминальных выражений
}