use crate::grid::Grid;

pub trait Renderer {
    type Output;

    fn render(&mut self, grid: &Grid) -> Self::Output;
}

pub struct TextRenderer {
    buffer: BufferedTextRenderer,
}

impl TextRenderer {
    pub fn square(mid_length: usize) -> Self {
        assert!(mid_length > 0);

        Self {
            buffer: BufferedTextRenderer::square(mid_length),
        }
    }
}

impl Renderer for TextRenderer {
    type Output = ();

    fn render(&mut self, grid: &Grid) -> Self::Output {
        self.buffer.render(grid);
        self.buffer.flush();
    }
}

#[derive(Debug)]
pub struct BufferedTextRenderer {
    lines: Vec<String>,
    xmin: i64,
    width: i64,
    ymin: i64,
    height: i64,
    count: usize,
}

impl BufferedTextRenderer {
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
        let lines = vec![String::new(); height + 1];

        Self {
            lines,
            xmin,
            width: width as i64,
            ymin,
            height: height as i64,
            count: 0,
        }
    }

    pub fn flush(&mut self) {
        // display everything!
        for dy in 0..=self.height as usize {
            println!("{}", self.lines[dy]);
        }
        println!();

        // clear out our buffers
        for s in self.lines.iter_mut() {
            s.clear();
        }

        self.count = 0;
    }
}

impl Renderer for BufferedTextRenderer {
    type Output = ();

    fn render(&mut self, grid: &Grid) -> Self::Output {
        for dy in 0..=self.height as usize {
            if self.count > 0 {
                // add a tab to every line
                self.lines[dy].push_str("\t");
            }
            for x in self.xmin..=(self.xmin + self.width) {
                let y = self.ymin + dy as i64;
                self.lines[dy].push_str(if grid.is_live((x, y)) { "#" } else { "-" });
            }
        }

        if self.lines[0].len() >= 100 {
            self.flush();
        } else {
            self.count += 1;
        }
    }
}
