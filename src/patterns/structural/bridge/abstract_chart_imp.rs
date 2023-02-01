pub trait AbstractChartImp {
    fn add_grid(&mut self, x_axis: Vec<String>, y_axis: Vec<String>);
    fn add_poly_line(&mut self, points: Vec<(f64, f64)>);
    fn draw_objects(&self);
}