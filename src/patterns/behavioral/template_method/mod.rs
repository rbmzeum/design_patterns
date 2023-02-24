pub mod abstract_chart;
pub mod charts;

// Example
pub fn behavioral_template_method() {
    use abstract_chart::AbstractChart;
    use charts::{ bar_chart::BarChart, line_chart::LineChart };

    let c1 = BarChart{};
    let c2 = LineChart{};

    c1.draw();
    c2.draw();
}