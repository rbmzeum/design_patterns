use crate::patterns::creational::factory_method::abstract_chart::AbstractChart;

pub struct BarChart {}

impl AbstractChart for BarChart {

    fn draw(&self) {
        println!("Draw bar chart!");
    }

}