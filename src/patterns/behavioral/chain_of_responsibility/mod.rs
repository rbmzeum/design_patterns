pub mod point;
pub mod event_handler;
pub mod candle;
pub mod grid;
pub mod chart;

// Example
pub fn behavioral_chain_of_responsibility() {
    use chart::Chart;
    use grid::Grid;
    use candle::Candle;
    use event_handler::EventHandler;
    use point::Point;

    let mut chart = Chart::new();
    let mut grid = Grid::new();
    let mut candle = Candle::new();

    chart.set_handler(Box::new(|_e, p| {
        println!("Chart click handler {:#?}", p);
    }));

    grid.set_parent(Box::new(chart));
    candle.set_parent(Box::new(grid));

    candle.handle_click(Point { x: 0.3, y: 0.4 });
}