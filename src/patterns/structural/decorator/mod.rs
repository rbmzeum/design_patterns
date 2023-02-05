use crate::patterns::structural::decorator::abstract_chart::AbstractChart;

pub mod abstract_chart;
pub mod abstract_chart_decorator;
pub mod chart;
pub mod chart_with_colors;
pub mod chart_with_signals;

// Example
pub fn structural_decorator() {
    use chart::Chart;
    use abstract_chart_decorator::AbstractChartDecorator;
    use chart_with_colors::ChartWithColors;
    use chart_with_signals::ChartWithSignals;

    let c = Chart{};
    let cc = ChartWithColors::new(Box::new(c.clone()));
    let ccs = ChartWithSignals::new(Box::new(cc));
    ccs.draw();

}