pub mod charts;
pub mod widgets;

pub mod abstract_chart;
pub mod abstract_widget;

// Example
pub fn creational_factory_method() {
    use crate::patterns::creational::factory_method::{
        abstract_widget::AbstractWidget,
        widgets::line_chart_widget::LineChartWidget,
        widgets::bar_chart_widget::BarChartWidget,
    };

    let line_chart_widget = LineChartWidget{};
    let line_chart = line_chart_widget.create_chart();
    line_chart.draw();

    let bar_chart_widget = BarChartWidget{};
    let bar_chart = bar_chart_widget.create_chart();
    bar_chart.draw();
}