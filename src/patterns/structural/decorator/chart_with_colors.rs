use super::abstract_chart::AbstractChart;
use super::abstract_chart_decorator::AbstractChartDecorator;

pub struct ChartWithColors {
    chart: Box<dyn AbstractChart>,
}

impl AbstractChart for ChartWithColors {
    fn draw(&self) {
        println!("Set colors");
        self.chart.draw();
    }
}

impl AbstractChartDecorator for ChartWithColors {
    fn new(chart: Box<dyn AbstractChart>) -> Self {
        Self {
            chart: chart,
        }
    }
}