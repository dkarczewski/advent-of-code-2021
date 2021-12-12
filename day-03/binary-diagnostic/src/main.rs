use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct FileStatistic {
    lines: usize,
    chars_in_line: usize,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Program must be executed with one argument: [file_name]");
        return;
    }

    // First argument is name of binary file.
    // Usefully is second argument which is `file name` with data.
    let file_name = match args.get(1) {
        Some(name) => name,
        None => {
            eprintln!("unable to get file name");
            return;
        }
    };

    let path = Path::new(file_name);
    let file_statistic = match make_file_statistic(path) {
        Ok(stat) => stat,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Open file for get data.
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("unable to open file, {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);

    let number_of_ones_in_column = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            parse_line_to_vector(&line)
        })
        .fold(vec![0; file_statistic.chars_in_line], |mut acc, line| {
            line.into_iter()
                .enumerate()
                .for_each(|(i, column)| acc[i] += usize::from(column));
            acc
        });
    let gamma_rate = number_of_ones_in_column
        .into_iter()
        .enumerate()
        .map(|(column, ones_in_column)| {
            if ones_in_column > (file_statistic.lines / 2) {
                1 << (file_statistic.chars_in_line - 1 - column)
            } else {
                0
            }
        })
        .sum::<usize>();

    // Mask used to reset unnecessary bits.
    let mask = make_mask(file_statistic.chars_in_line);
    let epsilon_rate = !gamma_rate & mask;
    println!(
        "Gamma rate: {}, epsilon rate {}, product {}",
        gamma_rate,
        epsilon_rate,
        (gamma_rate * epsilon_rate)
    );
}

fn make_mask(number_of_bits: usize) -> usize {
    (0..number_of_bits).into_iter().fold(0, |mut acc, i| {
        acc += 1 << i;
        acc
    })
}
fn make_file_statistic(path: &Path) -> Result<FileStatistic, String> {
    let file = File::open(path).map_err(|e| format!("unable to open file, {}", e))?;
    let reader = BufReader::new(file);
    let mut statistic = FileStatistic {
        lines: 0,
        chars_in_line: 0,
    };
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            statistic.chars_in_line = line.chars().count();
            continue;
        }
        // Check that all lines in the file have the same number of characters.
        if line.chars().count() != statistic.chars_in_line {
            return Err(String::from("incorrect line length in file"));
        }
        statistic.lines = i + 1;
    }
    Ok(statistic)
}

// Parse line to vector of numbers.
//
// Example:
//   input: "101"
//   output: [1, 0, 1]
fn parse_line_to_vector(line: &str) -> Vec<u8> {
    line.chars()
        .map(|letter| match letter {
            '1' => 1,
            '0' => 0,
            _ => {
                eprintln!("unknown character in line: {}", line);
                0
            }
        })
        .collect()
}
