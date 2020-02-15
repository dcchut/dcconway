use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition(i64, i64);

impl GridPosition {
    #[inline(always)]
    pub fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }

    #[inline(always)]
    pub fn neighbours(self) -> [GridPosition; 8] {
        let x = self.0;
        let y = self.1;

        [
            GridPosition::new(x - 1, y),
            GridPosition::new(x - 1, y - 1),
            GridPosition::new(x, y - 1),
            GridPosition::new(x + 1, y - 1),
            GridPosition::new(x + 1, y),
            GridPosition::new(x + 1, y + 1),
            GridPosition::new(x, y + 1),
            GridPosition::new(x - 1, y + 1),
        ]
    }
}

macro_rules! impl_into_grid_position {
    ( $tuple_ty: ty) => {
        impl Into<GridPosition> for ($tuple_ty, $tuple_ty) {
            fn into(self) -> GridPosition {
                GridPosition::new(self.0 as i64, self.1 as i64)
            }
        }
    };
    ( $tuple_ty_1: ty, $tuple_ty_2: ty) => {
        impl Into<GridPosition> for ($tuple_ty_1, $tuple_ty_2) {
            fn into(self) -> GridPosition {
                GridPosition::new(self.0 as i64, self.1 as i64)
            }
        }

        impl Into<GridPosition> for ($tuple_ty_2, $tuple_ty_1) {
            fn into(self) -> GridPosition {
                GridPosition::new(self.0 as i64, self.1 as i64)
            }
        }
    };
}

// TODO: expand this as required
impl_into_grid_position!(i64);
impl_into_grid_position!(i64, usize);
impl_into_grid_position!(i64, i32);
impl_into_grid_position!(i32);

#[derive(Clone, Debug)]
pub struct Grid {
    // TODO: think about different data structures that could be used here
    /// A set containing all of the inhabited states
    alive_states: HashSet<GridPosition>,

    /// A map from GridPosition -> {number of neighbouring alive cells}
    neighbour_count: HashMap<GridPosition, usize>,
}

impl Grid {
    #[inline(always)]
    pub fn empty() -> Self {
        Self {
            alive_states: HashSet::new(),
            neighbour_count: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn new(alive_states: HashSet<GridPosition>) -> Self {
        let mut grid = Self::empty();

        // TODO: investigate if we could do this smarter
        for pos in alive_states {
            grid.mark_as_alive(pos);
        }

        grid
    }

    #[inline(always)]
    pub fn _is_live(&self, pos: GridPosition) -> bool {
        self.alive_states.contains(&pos)
    }

    pub fn is_live<P: Into<GridPosition>>(&self, into_pos: P) -> bool {
        self._is_live(into_pos.into())
    }

    #[inline(always)]
    pub fn _is_dead(&self, pos: GridPosition) -> bool {
        !self.is_live(pos)
    }

    pub fn is_dead<P: Into<GridPosition>>(&self, into_pos: P) -> bool {
        self._is_dead(into_pos.into())
    }

    #[inline(always)]
    pub fn _neighbour_count(&self, pos: GridPosition) -> usize {
        *self.neighbour_count.get(&pos).unwrap_or(&0)
    }

    pub fn neighbour_count<P: Into<GridPosition>>(&self, into_pos: P) -> usize {
        self._neighbour_count(into_pos.into())
    }

    #[inline(always)]
    fn _make_live(&mut self, pos: GridPosition) {
        // If pos is already alive, don't do anything
        if self.is_live(pos) {
            return;
        }

        // Otherwise increment the neighbour count for all neighbouring positions
        for neighbour in pos.neighbours().iter() {
            *self.neighbour_count.entry(*neighbour).or_default() += 1;
        }

        // Mark this state as alive
        self.alive_states.insert(pos);
    }

    pub fn mark_as_alive<P: Into<GridPosition>>(&mut self, into_pos: P) {
        self._make_live(into_pos.into())
    }

    #[inline(always)]
    fn _make_dead(&mut self, pos: GridPosition) {
        // If `pos` isn't alive, do nothing.
        if self.is_dead(pos) {
            return;
        }

        // Otherwise decrement the neighbour count for all neighbouring positions
        for neighbour in pos.neighbours().iter() {
            let neighbour_count = self.neighbour_count.entry(*neighbour).or_default();

            if *neighbour_count == 1 {
                // destroy this entry
                self.neighbour_count.remove(neighbour);
            } else {
                // should never be zero
                debug_assert!(*neighbour_count > 0);
                *neighbour_count -= 1;
            }
        }

        // remove this state
        self.alive_states.remove(&pos);
    }

    pub fn mark_as_dead<P: Into<GridPosition>>(&mut self, into_pos: P) {
        self._make_dead(into_pos.into())
    }

    #[inline(always)]
    pub fn alive_positions_iter(&self) -> impl Iterator<Item = &GridPosition> {
        self.alive_states.iter()
    }

    #[inline(always)]
    pub fn neighbour_count_iter(&self) -> impl Iterator<Item = (&GridPosition, &usize)> {
        self.neighbour_count.iter()
    }
}
