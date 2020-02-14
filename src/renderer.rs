use crate::grid::{Grid, GridPosition};

pub trait Renderer {
    type Output;

    fn render(&self, grid: &Grid) -> Self::Output;
}

pub struct TextRenderer {
    pub xmin: i64,
    pub width: i64,
    pub ymin: i64,
    pub height: i64,
}

impl TextRenderer {
    pub fn square(mid_length: usize) -> Self {
        assert!(mid_length > 0);

        Self::new(
            -(mid_length as i64),
            2 * mid_length,
            -(mid_length as i64),
            2 * mid_length,
        )
    }

    pub fn new(xmin: i64, width: usize, ymin: i64, height: usize) -> Self {
        Self {
            xmin,
            width: width as i64,
            ymin,
            height: height as i64,
        }
    }
}

impl Renderer for TextRenderer {
    type Output = ();

    fn render(&self, grid: &Grid) -> Self::Output {
        for y in self.ymin..=(self.ymin + self.height) {
            for x in self.xmin..=(self.xmin + self.width) {
                let pos = GridPosition::new(x, y);

                print!("{}", if grid.is_live(pos) { "#" } else { "-" });
            }
            println!();
        }
        println!();
    }
}
