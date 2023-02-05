use super::abstract_chart::AbstractChart;

pub trait AbstractChartDecorator: AbstractChart {
    fn new(chart: Box<dyn AbstractChart>) -> Self;
}