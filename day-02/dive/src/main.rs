use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod commands {
    pub const FORWARD: &str = "forward";
    pub const DOWN: &str = "down";
    pub const UP: &str = "up";
}

// The structure describes the pilot movement pattern. It contains
// information about how much it moves.
//
// Movement depends on the `commands`.
struct MovementScheme {
    horizontal: isize,
    depth: isize,
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
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("unable to open file, {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        match MovementScheme::try_from(line) {
            Ok(scheme) => {
                horizontal_position += scheme.horizontal;
                depth += scheme.depth;
            }
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    }
    println!(
        "Horizontal position {}, depth {}, product {}",
        horizontal_position,
        depth,
        horizontal_position * depth
    );
}

impl TryFrom<String> for MovementScheme {
    type Error = String;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let split_line = line.split_ascii_whitespace().collect::<Vec<&str>>();
        // Acceptable format: '[command] + [movement]' (two items).
        if split_line.len() != 2 {
            let message = format!("incorrect data in file: '{:?}'", split_line);
            return Err(message);
        }
        let movement = split_line[1].parse::<isize>().unwrap();
        let (horizontal, depth) = match split_line[0] {
            commands::FORWARD => (movement, 0),
            commands::DOWN => (0, movement),
            #[allow(clippy::neg_multiply)]
            commands::UP => (0, -1 * movement),
            _ => {
                let message = format!("unknown command: '{:?}'", split_line[0]);
                return Err(message);
            }
        };
        Ok(MovementScheme { horizontal, depth })
    }
}

#[cfg(test)]
mod example_data {
    #[test]
    fn example_data() {
        let input = vec![
            "forward 5".to_owned(),
            "down 5".to_owned(),
            "forward 8".to_owned(),
            "up 3".to_owned(),
            "down 8".to_owned(),
            "forward 2".to_owned(),
        ];
        let mut horizontal_position = 0;
        let mut depth = 0;
        for line in input {
            let scheme = super::MovementScheme::try_from(line).unwrap();
            horizontal_position += scheme.horizontal;
            depth += scheme.depth;
        }
        assert_eq!(horizontal_position * depth, 150);
    }
}
