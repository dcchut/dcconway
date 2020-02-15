use dcconway::grid::Grid;
use dcconway::renderer::{BufferedTextRenderer, Renderer};
use dcconway::rules::{BasicRuleSet, Ticker};

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
    let mut renderer = BufferedTextRenderer::square(3, Some(100));

    let mut grid = line(2);
    renderer.render(&grid);

    for _ in 0..25 {
        grid = BasicRuleSet::tick(&grid);
        renderer.render(&grid);
    }

    // flush out anything remaining in the buffer
    renderer.flush();
}
