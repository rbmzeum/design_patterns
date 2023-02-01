use crate::patterns::structural::bridge::{
    abstract_chart::AbstractChart,
    abstract_chart_imp::AbstractChartImp,
};

pub struct Gtk4DrawingAreaChart {
    chart_name: String,
    chart_imp: Box<dyn AbstractChartImp>,
}

impl Gtk4DrawingAreaChart {
    pub fn new() -> Self {
        use crate::patterns::structural::bridge::chart_imp::gtk4_drawing_area_chart_imp::Gtk4DrawingAreaChartImp;
        let chart_imp = Box::new(Gtk4DrawingAreaChartImp::new());

        Self {
            chart_name: "".to_string(),
            chart_imp: chart_imp,
        }
    }
}

impl AbstractChart for Gtk4DrawingAreaChart {
    fn init(&mut self) {
        self.chart_name = "Gtk4DrawingAreaChart".to_string();

        let x_axis = vec!["2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()];
        let y_axis = vec!["3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string()];
        self.chart_imp.add_grid(x_axis, y_axis);

        let points = vec![(2.0, 3.0), (3.0, 4.0), (4.0, 5.0), (7.0, 8.0)];
        self.chart_imp.add_poly_line(points);
    }

    fn draw(&self) {
        println!("CHART: {}", &self.chart_name);
        self.chart_imp.draw_objects();
    }
}