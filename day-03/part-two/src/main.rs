use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct FileStatistic {
    lines: usize,
    chars_in_line: usize,
}

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(String::from(
            "Program must be executed with one argument: [file_name]",
        ));
    }

    // First argument is name of binary file.
    // Usefully is second argument which is `file name` with data.
    let file_name = args
        .get(1)
        .ok_or_else(|| String::from("unable to get file name"))?;

    let path = Path::new(file_name);
    let file_statistic = make_file_statistic(path)?;

    // Open file for calculate `oxygen generator rating`.
    let lines = lines_from_file(path)?;
    let lines = lines.iter().map(|line| &**line).collect::<Vec<&str>>();

    // Proposals for another implementation / optimization:
    // 1. Parse file to `Vec<u8>` where each char is separate value, example:
    //   "101" -> [1, 0, 1],
    // 2. Parse file to `Vec<u8>`, then use bitmask to check if bit is set:
    //   "101" -> 5,
    //   "011" -> 3,
    let oxygen_generator_rating = oxygen_generator_rating(lines, &file_statistic)?;
    println!("Oxygen generator rating: {}", oxygen_generator_rating);

    // Re-open file for calculate `CO2 scrubber rating`.
    let lines = lines_from_file(path)?;
    let lines = lines.iter().map(|line| &**line).collect::<Vec<&str>>();

    let co2_scrubber_rating = co2_scrubber_rating(lines, &file_statistic)?;
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);

    println!(
        "Life support rating: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
    Ok(())
}

fn lines_from_file(path: &Path) -> Result<Vec<String>, String> {
    let file = File::open(path).map_err(|e| format!("unable to open file, {}", e))?;
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten().collect::<Vec<String>>();
    Ok(lines)
}

fn oxygen_generator_rating(
    mut lines: Vec<&str>,
    file_statistic: &FileStatistic,
) -> Result<usize, String> {
    for column_number in 0..file_statistic.chars_in_line {
        let bit = most_common_bit_in_column(&lines, column_number);
        lines.retain(|line| line.chars().nth(column_number).unwrap() == bit);
        if lines.len() == 1 {
            break;
        }
    }
    if lines.len() != 1 {
        return Err(String::from("unable to calculate oxygen generator rating"));
    }
    usize::from_str_radix(lines[0], 2).map_err(|e| format!("unable to convert str to digit: {}", e))
}

fn co2_scrubber_rating(
    mut lines: Vec<&str>,
    file_statistic: &FileStatistic,
) -> Result<usize, String> {
    for column_number in 0..file_statistic.chars_in_line {
        let bit = least_common_bit_in_column(&lines, column_number);
        lines.retain(|line| line.chars().nth(column_number).unwrap() == bit);
        if lines.len() == 1 {
            break;
        }
    }
    if lines.len() != 1 {
        return Err(String::from("unable to calculate CO2 scrubber rating"));
    }
    usize::from_str_radix(lines[0], 2).map_err(|e| format!("unable to convert str to digit, {}", e))
}

fn most_common_bit_in_column(lines: &[&str], column_number: usize) -> char {
    let lines_in_vec = lines.len() as u32;
    let ones = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .fold(0, |acc, value| {
            acc + value[column_number].to_digit(10).unwrap()
        });
    // We can use `.filter().count()` instead of `.fold()`, but
    // `.fold()` is faster.
    // Source: https://github.com/rust-lang/rust/issues/33038

    // We assume that vector contains only zeros and ones.
    let zeros = lines_in_vec - ones;
    if ones >= zeros {
        '1'
    } else {
        '0'
    }
}

fn least_common_bit_in_column(lines: &[&str], column_number: usize) -> char {
    let lines_in_vec = lines.len() as u32;
    let ones = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .fold(0, |acc, value| {
            acc + value[column_number].to_digit(10).unwrap()
        });
    // We can use `.filter().count()` instead of `.fold()`, but
    // `.fold()` is faster.
    // Source: https://github.com/rust-lang/rust/issues/33038

    // We assume that vector contains only zeros and ones.
    let zeros = lines_in_vec - ones;
    if ones >= zeros {
        '0'
    } else {
        '1'
    }
}

fn make_file_statistic(path: &Path) -> Result<FileStatistic, String> {
    let file = File::open(path).map_err(|e| format!("unable to open file, {}", e))?;
    let reader = BufReader::new(file);
    let mut statistic = FileStatistic {
        lines: 0,
        chars_in_line: 0,
    };
    for (i, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("read line error, {}", e))?;
        if i == 0 {
            statistic.chars_in_line = line.chars().count();
        }
        // Check that all lines in the file have the same number of characters.
        if line.chars().count() != statistic.chars_in_line {
            return Err(String::from("incorrect line length in file"));
        }
        statistic.lines = i + 1;
    }
    Ok(statistic)
}
