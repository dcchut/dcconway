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
            buffer: BufferedTextRenderer::square(mid_length, None),
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
    count: usize,
    line_length_threshold: Option<usize>,
    xmin: i64,
    ymin: i64,
    width: usize,
    height: usize,
}

impl BufferedTextRenderer {
    pub fn square(mid_length: usize, line_length_threshold: Option<usize>) -> Self {
        assert!(mid_length > 0);

        Self::new(
            -(mid_length as i64),
            2 * mid_length,
            -(mid_length as i64),
            2 * mid_length,
            line_length_threshold,
        )
    }

    pub fn new(
        xmin: i64,
        width: usize,
        ymin: i64,
        height: usize,
        line_length_threshold: Option<usize>,
    ) -> Self {
        let lines = vec![String::new(); height + 1];

        Self {
            lines,
            count: 0,
            line_length_threshold,
            xmin,
            ymin,
            width,
            height,
        }
    }

    pub fn flush(&mut self) {
        // don't flush unless we've got flushables
        if self.count == 0 {
            return;
        }

        // display everything!
        for dy in 0..=self.height {
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
        for dy in 0..=self.height {
            let y = self.ymin + dy as i64;
            if self.count > 0 {
                // add a tab to every line
                self.lines[dy].push('\t');
            }
            for dx in 0..=self.width {
                let x = self.xmin + dx as i64;

                self.lines[dy].push_str(if grid.is_live((x, y)) { "#" } else { "-" });
            }
        }

        // if we've exceeded the line length threshold, flush the buffer
        if let Some(line_width_threshold) = self.line_length_threshold {
            if self.lines[0].len() >= line_width_threshold {
                self.flush();
                return;
            }
        }

        // otherwise increase our count by 1
        self.count += 1;
    }
}
