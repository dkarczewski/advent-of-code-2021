use std::env;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

type InternalResult<T> = Result<T, InternalError>;

#[derive(Debug)]
struct InternalError(String);

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl InternalError {
    fn error(error_msg: &str) -> Self {
        InternalError(error_msg.to_owned())
    }
}

impl std::error::Error for InternalError {}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Point {
    type Error = InternalError;

    fn try_from(raw_str: &str) -> Result<Self, Self::Error> {
        // Example input: `123,456`.
        let splitted_str = raw_str.split_terminator(',').collect::<Vec<_>>();

        if splitted_str.len() != 2 {
            return Err(InternalError::error(
                "incorrect number of parts after splitting",
            ));
        }

        let x = splitted_str[0]
            .parse::<usize>()
            .map_err(|e| InternalError(format!("{}", e)))?;
        let y = splitted_str[1]
            .parse::<usize>()
            .map_err(|e| InternalError(format!("{}", e)))?;

        Ok(Point { x, y })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CORR_X_MAX: usize = 1000;
    const CORR_Y_MAX: usize = 1000;

    // Fixed venture map. 1000 by 1000 elements.
    // Input point has coordinate lower than 1000.
    let mut venture_map = vec![0; CORR_X_MAX * CORR_Y_MAX];

    if env::args().count() != 2 {
        eprintln!("Program must be executed with one argument: [file_name]");
        return Err(InternalError::error("Incorrect argument").into());
    }
    let args = env::args().collect::<Vec<String>>();

    // First argument is name of binary file.
    // Usefully is second argument which is `file name`.
    let file_path = args
        .get(1)
        .map(PathBuf::from)
        .ok_or_else(|| InternalError::error("unable to get file name"))?;

    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let points = reader
        .lines()
        .flatten()
        .into_iter()
        .flat_map(|l| parse_line_into_points(&l))
        .collect::<Vec<_>>();

    for (p1, p2) in points {
        // println!("{:?} {:?}", p1, p2);
        match (p1, p2) {
            // Horizontal line
            (p1, p2) if p1.x == p2.x => {
                // The values can appear in any order.
                // "For" need ascending order.
                let (y_min, y_max) = if p1.y <= p2.y {
                    (p1.y, p2.y)
                } else {
                    (p2.y, p1.y)
                };

                for y in y_min..=y_max {
                    venture_map[p1.x + y * CORR_Y_MAX] += 1;
                }
            }
            // Vertical line
            (p1, p2) if p1.y == p2.y => {
                let (x_min, x_max) = if p1.x <= p2.x {
                    (p1.x, p2.x)
                } else {
                    (p2.x, p1.x)
                };

                for x in x_min..=x_max {
                    venture_map[x + p1.y * CORR_Y_MAX] += 1;
                }
            }
            // Main assume: input contains only horizontal and vertical lines.
            // Other lines are skipped.
            _ => (),
        }
    }

    let number_of_overlaps = venture_map.into_iter().filter(|i| *i >= 2).count();
    println!(
        "Number of points where at least two lines overlaps: {:?}",
        number_of_overlaps
    );

    Ok(())
}

fn parse_line_into_points(line: &str) -> InternalResult<(Point, Point)> {
    // Example input: `123,456 -> 589,012`
    let splitted_line = line.split(" -> ").collect::<Vec<_>>();
    let first_point = Point::try_from(splitted_line[0])?;
    let second_point = Point::try_from(splitted_line[1])?;

    Ok((first_point, second_point))
}
