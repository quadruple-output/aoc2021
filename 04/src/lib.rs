use eyre::{eyre, Result};
use std::io::BufRead;

enum BoardEvent {
    Miss,
    Found,
    Bingo(usize),
}

pub fn solve_a(input: &mut dyn BufRead) -> Result<usize> {
    let Input {
        numbers,
        mut boards,
    } = Input::read(input)?;

    for number in numbers {
        for board in &mut boards {
            match board.play(number) {
                BoardEvent::Miss | BoardEvent::Found => (),
                BoardEvent::Bingo(score) => return Ok(score),
            }
        }
    }
    Err(eyre!("No board won"))
}

pub fn solve_b(input: &mut dyn BufRead) -> Result<usize> {
    let Input {
        numbers,
        mut boards,
    } = Input::read(input)?;
    let mut last_score = None;

    for number in numbers {
        let mut winning_boards = Vec::<*const Board>::new();
        for board in &mut boards {
            match board.play(number) {
                BoardEvent::Miss | BoardEvent::Found => (),
                BoardEvent::Bingo(score) => {
                    last_score = Some(score);
                    winning_boards.push(board);
                }
            }
        }
        boards.retain(|board| !winning_boards.contains(&(board as *const Board)))
    }
    if let Some(score) = last_score {
        Ok(score)
    } else {
        Err(eyre!("No board won"))
    }
}

struct Input {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

struct Board([[Cell; 5]; 5]);

#[derive(Clone, Copy, Default)]
struct Cell {
    value: usize,
    marked: bool,
}

impl Board {
    pub fn new() -> Self {
        Self([[Cell::default(); 5]; 5])
    }

    fn play(&mut self, value: usize) -> BoardEvent {
        let mut found = None;
        for (x, column) in self.0.iter_mut().enumerate() {
            for (y, mut cell) in column.iter_mut().enumerate() {
                if cell.value == value {
                    cell.marked = true;
                    found = Some((x, y));
                }
            }
        }
        if let Some(found_at) = found {
            if self.column_full(found_at.0) || self.row_full(found_at.1) {
                BoardEvent::Bingo(self.score(value))
            } else {
                BoardEvent::Found
            }
        } else {
            BoardEvent::Miss
        }
    }

    fn row_full(&self, y: usize) -> bool {
        !self.0.iter().any(|column| !column[y].marked)
    }

    fn column_full(&self, x: usize) -> bool {
        !self.0[x].iter().any(|cell| !cell.marked)
    }

    fn score(&self, value: usize) -> usize {
        value
            * self
                .0
                .iter()
                .flat_map(|column| column.iter())
                .filter(|cell| !cell.marked)
                .fold(0, |accu, cell| accu + cell.value)
    }
}

impl Input {
    fn read(file: &mut dyn BufRead) -> Result<Self> {
        let mut lines = file.lines();
        let first_line = lines.next().ok_or_else(|| eyre!("input is empty"))??;
        let numbers = first_line
            .split(',')
            .map(|digits| digits.parse::<usize>().map_err(eyre::Report::new))
            .collect::<Result<Vec<_>>>()?;
        let mut boards = Vec::new();
        let mut board_line = 0;
        let mut board = Board::new();
        for line in lines {
            let line = line?;
            if !line.is_empty() {
                line.split(' ')
                    .filter(|s| !s.is_empty())
                    .take(5)
                    .map(|s| s.parse::<usize>().map_err(eyre::Report::new))
                    .enumerate()
                    .try_for_each::<_, Result<_>>(|(n, value)| {
                        board.0[board_line][n].value = value?;
                        Ok(())
                    })?;
                if board_line == 4 {
                    boards.push(board);
                    board = Board::new();
                    board_line = 0;
                } else {
                    board_line += 1;
                }
            }
        }
        if board_line == 0 {
            Ok(Self { numbers, boards })
        } else {
            Err(eyre!("superfluous board lines"))
        }
    }
}
