use super::abstract_chart::AbstractChart;

#[derive(Clone)]
pub struct Chart {}

impl AbstractChart for Chart {
    fn draw(&self) {
        println!("Draw chart");
    }
}