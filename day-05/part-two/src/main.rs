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

#[derive(Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
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
            .parse::<isize>()
            .map_err(|e| InternalError(format!("{}", e)))?;
        let y = splitted_str[1]
            .parse::<isize>()
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
        let points = make_points(p1, p2);

        for p in points {
            venture_map[p.x as usize + p.y as usize * CORR_Y_MAX] += 1;
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

fn make_points(p1: Point, p2: Point) -> Vec<Point> {
    let x_distance = p2.x - p1.x;
    let y_distance = p2.y - p1.y;
    let number_of_new_points = if x_distance == 0 {
        // Vertical line.
        y_distance.abs()
    } else {
        // Horizontal line or diagonal line.
        // For diagonal line `x_distance == y_distance`.
        x_distance.abs()
    };

    (0..=number_of_new_points)
        .into_iter()
        .map(|n| Point {
            x: p1.x + n * x_distance.signum(),
            y: p1.y + n * y_distance.signum(),
        })
        .collect()
}
