use crate::grid::Grid;

pub trait Ticker {
    fn tick(grid: &Grid) -> Grid;
}

pub struct BasicRuleSet {}

impl Ticker for BasicRuleSet {
    fn tick(grid: &Grid) -> Grid {
        // identify live cells with two or three neighbours
        let live_cells = grid
            .alive_positions_iter()
            .filter(|pos| {
                let neighbour_count = grid.neighbour_count(**pos);

                neighbour_count == 2 || neighbour_count == 3
            })
            .chain(grid.neighbour_count_iter().filter_map(|(pos, count)| {
                if *count == 3 {
                    Some(pos)
                } else {
                    None
                }
            }))
            .cloned()
            .collect();

        Grid::new(live_cells)
    }
}
