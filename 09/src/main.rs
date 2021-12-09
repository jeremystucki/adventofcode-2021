use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let data: Vec<_> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10))
                .map(Option::unwrap)
                .collect::<Vec<_>>()
        })
        .collect();

    let mut total = 0;
    let height = data.len();
    let width = data.get(0).unwrap().len();

    for row in 0..height {
        for column in 0..width {
            total += get_if_low_point(&data, row, column).unwrap_or(0);
        }
    }

    println!("{:?}", total);
}

fn get_if_low_point(data: &Vec<Vec<u32>>, row: usize, column: usize) -> Option<u32> {
    let point_value = data.get(row).unwrap().get(column).unwrap();

    let is_low_point = match (row, column) {
        (0, 0) => vec![(0, 1), (1, 0)],
        (0, _) => vec![(0, column + 1), (0, column - 1), (1, column)],
        (_, 0) => vec![(row - 1, column), (row + 1, column), (row, column + 1)],
        (_, _) => vec![
            (row - 1, column),
            (row + 1, column),
            (row, column - 1),
            (row, column + 1),
        ],
    }
    .into_iter()
    .all(|(row_index, column_index)| {
        data.get(row_index)
            .and_then(|row| row.get(column_index))
            .map_or(true, |v| v > point_value)
    });

    if is_low_point {
        Some(*point_value + 1)
    } else {
        None
    }
}
