use crate::patterns::creational::factory_method::abstract_chart::AbstractChart;

pub struct LineChart {}

impl AbstractChart for LineChart {

    fn draw(&self) {
        println!("Draw line chart!");
    }

}