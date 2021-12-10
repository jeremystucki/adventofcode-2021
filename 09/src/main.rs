use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part_1();
    part_2();
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
                .map(|d| d as i64)
                .collect::<Vec<_>>()
        })
        .collect();

    let width = data.get(0).unwrap().len();
    let height = data.len();

    let result = (0..width)
        .flat_map(|column| (0..height).map(move |row| (row, column)))
        .filter_map(|(row, column)| get_basin(&data, row, column))
        .fold(HashSet::<(usize, usize)>::new(), |mut set, point| {
            set.insert(point);
            set
        })
        .into_iter()
        .map(|(row, column)| 1 + *data.get(row).unwrap().get(column).unwrap() as u128)
        .sum::<u128>();

    println!("{:?}", result);
}

fn part_2() {
    let file = File::open("input").unwrap();

    let data: Vec<_> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10))
                .map(Option::unwrap)
                .map(|d| d as i64)
                .collect::<Vec<_>>()
        })
        .collect();

    let width = data.get(0).unwrap().len();
    let height = data.len();

    let mut basin_sizes = (0..width)
        .flat_map(|column| (0..height).map(move |row| (row, column)))
        .filter_map(|(row, column)| get_basin(&data, row, column))
        .fold(HashMap::<(usize, usize), u128>::new(), |mut map, point| {
            (*map.entry(point).or_default() += 1);
            map
        })
        .into_values()
        .collect::<Vec<_>>();

    basin_sizes.sort();

    println!("{:?}", basin_sizes.iter().rev().take(3).product::<u128>());
}

fn get_basin(data: &Vec<Vec<i64>>, mut row: usize, mut column: usize) -> Option<(usize, usize)> {
    if *data.get(row).unwrap().get(column).unwrap() == 9 {
        return None;
    }

    loop {
        if let Some((neighbor_row, neighbor_column)) = get_lowest_neighbor(data, row, column) {
            row = neighbor_row;
            column = neighbor_column;
        } else {
            break Some((row, column));
        }
    }
}

fn get_lowest_neighbor(data: &Vec<Vec<i64>>, row: usize, column: usize) -> Option<(usize, usize)> {
    let point_value = data.get(row).unwrap().get(column).unwrap();

    match (row, column) {
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
    .filter_map(|(neighbor_row, neighbor_column)| {
        data.get(neighbor_row)
            .and_then(|row| row.get(neighbor_column))
            .map(|value| (neighbor_row, neighbor_column, value - point_value))
    })
    .filter(|(_, _, difference)| *difference < 0)
    .min_by_key(|(_, _, difference)| *difference)
    .map(|(neighbor_row, neighbor_column, _)| (neighbor_row, neighbor_column))
}
