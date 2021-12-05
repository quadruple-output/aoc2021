use eyre::{eyre, Result};
use std::io::BufRead;

pub fn solve_a(input: &mut dyn BufRead) -> Result<usize> {
    let lines = line_segments(input)?;
    let max_coord = lines.iter().fold((0, 0), |max, line| {
        (
            max.0.max(line.0.x).max(line.1.x),
            max.1.max(line.0.y).max(line.1.y),
        )
    });
    let mut area = Area::new(max_coord.0, max_coord.1);
    for line in lines
        .iter()
        .filter(|line| line.0.x == line.1.x || line.0.y == line.1.y)
    {
        line.draw_on(&mut area);
    }
    Ok(area.count_overlaps())
}

fn line_segments(input: &mut dyn BufRead) -> Result<Vec<Line>> {
    let mut line_segments = Vec::new();
    for (line_no, input_line) in input.lines().enumerate() {
        let input_line = input_line?;
        let mut coords = input_line
            .split(|c: char| !c.is_numeric())
            .filter(|str| !str.is_empty())
            .map(|digits| digits.parse::<usize>().map_err(eyre::Report::new));
        if let (Some(x1), Some(y1), Some(x2), Some(y2)) =
            (coords.next(), coords.next(), coords.next(), coords.next())
        {
            line_segments.push(Line::new(x1?, y1?, x2?, y2?));
        } else {
            return Err(eyre!(
                "not enough coordinates in input line {}",
                line_no + 1
            ));
        }
        if coords.next().is_some() {
            return Err(eyre!("too many coordinates in input line {}", line_no));
        }
    }
    Ok(line_segments)
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line(Point, Point);

#[derive(Debug)]
struct Area(Vec<Vec<usize>>);

impl Line {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self(Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
    }

    fn draw_on(&self, area: &mut Area) {
        let dx = self.1.x as isize - self.0.x as isize;
        let dy = self.1.y as isize - self.0.y as isize;
        let num_steps = dx.abs().max(dy.abs());
        (0..=num_steps).for_each(|step| {
            area.plot(
                (self.0.x as isize + step * (dx / num_steps)) as usize,
                (self.0.y as isize + step * (dy / num_steps)) as usize,
            )
        });
    }
}

impl Area {
    fn new(dim_x: usize, dim_y: usize) -> Self {
        let mut x_vec = Vec::with_capacity(dim_x + 1);
        (0..=dim_x).for_each(|_| {
            let mut y_vec = Vec::with_capacity(dim_y + 1);
            (0..=dim_y).for_each(|_| y_vec.push(0));
            x_vec.push(y_vec);
        });
        Self(x_vec)
    }

    fn plot(&mut self, x: usize, y: usize) {
        self.0[x][y] += 1;
    }

    fn count_overlaps(&self) -> usize {
        self.0
            .iter()
            .flat_map(|y| y.iter())
            .filter(|&&xy| xy > 1)
            .count()
    }
}

pub fn solve_b(_input: &mut dyn BufRead) -> Result<usize> {
    Err(eyre!("not implemented"))
}
