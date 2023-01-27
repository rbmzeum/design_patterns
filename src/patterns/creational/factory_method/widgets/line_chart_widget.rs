use crate::patterns::creational::factory_method::abstract_widget::AbstractWidget;
use crate::patterns::creational::factory_method::{
    abstract_chart::AbstractChart,
    charts::line_chart::LineChart,
};

pub struct LineChartWidget {}

impl AbstractWidget for LineChartWidget {

    fn create_chart(&self) -> Box<dyn AbstractChart> {
        let chart = LineChart{};
        Box::new(chart)
    }

}