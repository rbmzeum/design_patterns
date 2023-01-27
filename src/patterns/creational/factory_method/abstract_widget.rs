use crate::patterns::creational::factory_method::abstract_chart::AbstractChart;

pub trait AbstractWidget {
    fn create_chart(&self) -> Box<dyn AbstractChart>; // This is factory method
}