use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    let data = match parse_file_to_vector(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("unable to parse data from file, {}", e);
            return;
        }
    };
    let incremental_measurements = count_incremental_measurements(data);
    println!("Incremental measurements: {}", incremental_measurements);
}

fn count_incremental_measurements(measurements: Vec<usize>) -> usize {
    measurements
        .windows(4)
        .filter(|item| item[0..3].iter().sum::<usize>() < item[1..4].iter().sum::<usize>())
        .count()
}

fn parse_file_to_vector(file_name: &Path) -> std::io::Result<Vec<usize>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| line.ok().and_then(|s| s.parse::<usize>().ok()).unwrap())
        .collect::<Vec<usize>>())
}

#[cfg(test)]
mod example_data {
    #[test]
    fn example_data() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let incremental_measurements = super::count_incremental_measurements(input);
        assert_eq!(incremental_measurements, 5);
    }
}
