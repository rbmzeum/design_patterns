pub trait AbstractChart {

    // template method
    fn draw(&self) {
        self.draw_grid();
        if self.is_chart_defined() {
            self.draw_chart();
        }
    }

    fn draw_grid(&self);
    fn is_chart_defined(&self) -> bool;
    fn draw_chart(&self);

}