use crate::patterns::behavioral::template_method::abstract_chart::AbstractChart;

pub struct LineChart {}

impl AbstractChart for LineChart {

    fn draw_grid(&self) {
        println!("Draw grid");
    }

    fn is_chart_defined(&self) -> bool {
        true
    }

    fn draw_chart(&self) {
        println!("Draw line chart");
    }

}