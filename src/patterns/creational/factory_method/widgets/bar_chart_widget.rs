use crate::patterns::creational::factory_method::abstract_widget::AbstractWidget;
use crate::patterns::creational::factory_method::{
    abstract_chart::AbstractChart,
    charts::bar_chart::BarChart,
};

pub struct BarChartWidget {}

impl AbstractWidget for BarChartWidget {

    fn create_chart(&self) -> Box<dyn AbstractChart> {
        let chart = BarChart{};
        Box::new(chart)
    }

}