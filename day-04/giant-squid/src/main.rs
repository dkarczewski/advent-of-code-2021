use std::env;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
struct InternalError<'a>(&'a str);

impl fmt::Display for InternalError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InternalError<'_> {}

// Board has fixed dimensions.
// It contains 5 rows and 5 columns (25 numbers).
#[derive(Clone, Debug)]
struct Board {
    // TODO: Use two dimensional array instead of Vec?
    cells: Vec<Cell>,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: Vec::with_capacity(25),
        }
    }
}

impl Board {
    fn is_all_numbers_in_row_marked(&self) -> bool {
        for row in self.cells.chunks_exact(5) {
            if row.iter().all(|c| c.is_marked) {
                return true;
            }
        }
        false
    }

    fn is_all_numbers_in_column_marked(&self) -> bool {
        // TODO: Use two dimensional array and transpose it?
        for column_idx in 0..5 {
            let column = self
                .cells
                .iter()
                .skip(column_idx)
                .step_by(5)
                .collect::<Vec<_>>();

            if column.iter().all(|c| c.is_marked) {
                return true;
            }
        }
        false
    }

    fn sum_of_all_unmarked_numbers(&self) -> usize {
        self.cells
            .iter()
            .filter(|c| !c.is_marked)
            .map(|c| c.number)
            .sum()
    }
}

#[derive(Clone, Debug)]
struct Cell {
    number: usize,
    is_marked: bool,
}

impl Cell {
    pub fn new(number: usize) -> Self {
        Cell {
            number,
            is_marked: false,
        }
    }
}

struct WinBoard {
    board: Board,
    last_called_number: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().count() != 2 {
        eprintln!("Program must be executed with one argument: [file_name]");
        return Err(InternalError("Incorrect argument").into());
    }
    let args = env::args().collect::<Vec<String>>();

    // First argument is name of binary file.
    // Usefully is second argument which is `file name`.
    let file_path = args
        .get(1)
        .map(PathBuf::from)
        .ok_or(InternalError("unable to get file name"))?;

    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .flatten()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    // Input numbers are saved as first line in file.
    let first_line = lines
        .first()
        .ok_or(InternalError("unable to read first line"))?;
    let input_numbers = parse_line_into_vector_of_numbers(first_line.as_str());

    // One board has 5 rows and 5 columns (25 numbers).
    let mut boards = lines[1..]
        .chunks_exact(5)
        .into_iter()
        .map(parse_into_board)
        .collect::<Vec<_>>();

    let win_board = process_numbers_and_boards(&input_numbers, &mut boards);

    match win_board {
        Some(wb) => {
            let sum = wb.board.sum_of_all_unmarked_numbers();
            let score = sum * wb.last_called_number;
            println!("Win board has been found. Final score: {}", score);
        }
        None => println!("Win board not found"),
    }

    Ok(())
}

fn parse_line_into_vector_of_numbers(line: &str) -> Vec<usize> {
    line.split_terminator(&[',', ' '])
        .into_iter()
        .flat_map(|item| item.parse::<usize>())
        .collect()
}

fn parse_into_board<T>(lines: &[T]) -> Board
where
    T: AsRef<str>,
{
    let cells = lines
        .iter()
        .fold(Vec::with_capacity(25), |mut acc, line| {
            acc.extend(parse_line_into_vector_of_numbers(line.as_ref()));
            acc
        })
        .into_iter()
        .map(Cell::new)
        .collect::<Vec<_>>();
    Board { cells }
}

// Returns `Some(..)` when found win row or column. Otherwise returns `None`.
fn process_numbers_and_boards(numbers: &[usize], boards: &mut [Board]) -> Option<WinBoard> {
    for number in numbers {
        for board in boards.iter_mut() {
            for cell in &mut board.cells {
                if cell.number == *number {
                    cell.is_marked = true;
                }
            }

            if board.is_all_numbers_in_row_marked() || board.is_all_numbers_in_column_marked() {
                return Some(WinBoard {
                    board: board.clone(),
                    last_called_number: *number,
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_numbers() {
        let data = "1,2,aa,bb,,3,#,4";
        assert_eq!(parse_line_into_vector_of_numbers(data), vec![1, 2, 3, 4]);

        let data = "1 2 aa 3   4";
        assert_eq!(parse_line_into_vector_of_numbers(data), vec![1, 2, 3, 4]);

        let data = "1 2,aa, bb 3,# 4";
        assert_eq!(parse_line_into_vector_of_numbers(data), vec![1, 2, 3, 4]);
    }
}
