use crate::patterns::structural::bridge::abstract_chart_imp::AbstractChartImp;

pub struct Gtk4DrawingAreaChartImp {
    grid: (Vec<String>, Vec<String>),
    points: Vec<(f64, f64)>,
}

impl Gtk4DrawingAreaChartImp {
    pub fn new() -> Self {
        Self {
            grid: (vec![], vec![]),
            points: vec![],
        }
    }
}

impl AbstractChartImp for Gtk4DrawingAreaChartImp {
    fn add_grid(&mut self, x_axis: Vec<String>, y_axis: Vec<String>) {
        self.grid = (x_axis, y_axis);
    }
    fn add_poly_line(&mut self, points: Vec<(f64, f64)>) {
        self.points = points;
    }
    fn draw_objects(&self) {
        println!("GRID: {:#?}", self.grid);
        println!("POLY LINE: {:#?}", self.points);
    }
}