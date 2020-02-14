use conway::grid::Grid;
use conway::renderer::{Renderer, TextRenderer};
use conway::rules::{BasicRuleSet, Ticker};

/// Creates a grid containing a line of alive cells.  The total length of the line is 2 * `span` + 1.
fn line(span: usize) -> Grid {
    let mut grid = Grid::empty();

    for x in 0..=span {
        grid.mark_as_alive((x, 0));
        grid.mark_as_alive((-(x as i64), 0));
    }

    grid
}

fn main() {
    let renderer = TextRenderer::square(3);

    let mut grid = line(3);
    renderer.render(&grid);

    for _ in 0..5 {
        grid = BasicRuleSet::tick(&grid);
        renderer.render(&grid);
    }
}
