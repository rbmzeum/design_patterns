use super::abstract_chart::AbstractChart;
use super::abstract_chart_decorator::AbstractChartDecorator;

pub struct ChartWithSignals {
    chart: Box<dyn AbstractChart>,
}

impl AbstractChart for ChartWithSignals {
    fn draw(&self) {
        self.chart.draw();
        println!("Draw signals");
    }
}

impl AbstractChartDecorator for ChartWithSignals {
    fn new(chart: Box<dyn AbstractChart>) -> Self {
        Self {
            chart: chart,
        }
    }
}