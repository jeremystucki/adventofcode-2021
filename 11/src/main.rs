use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
struct Squid {
    own_position: usize,
    neighbors: Vec<usize>,
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let width = 10;
    let height = 10;

    let file = File::open("input").unwrap();

    let data = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10))
                .map(Option::unwrap)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut energy_levels = [0; 100];
    let squids = (0..width)
        .flat_map(|row| (0..height).map(move |column| (row, column)))
        .map(|(row, column)| {
            let get_index = |row, column| row * width + column;

            energy_levels[get_index(row, column)] = *data.get(row).unwrap().get(column).unwrap();

            Squid {
                own_position: row * width + column,
                neighbors: if row == 0 && column == 0 {
                    vec![
                        get_index(row, column + 1),
                        get_index(row + 1, column),
                        get_index(row + 1, column + 1),
                    ]
                } else if row == (height - 1) && column == (width - 1) {
                    vec![
                        get_index(row, column - 1),
                        get_index(row - 1, column),
                        get_index(row - 1, column - 1),
                    ]
                } else if row == 0 && column == (width - 1) {
                    vec![
                        get_index(row, column - 1),
                        get_index(row + 1, column),
                        get_index(row + 1, column - 1),
                    ]
                } else if row == (height - 1) && column == 0 {
                    vec![
                        get_index(row, column + 1),
                        get_index(row - 1, column),
                        get_index(row - 1, column + 1),
                    ]
                } else if row == 0 {
                    vec![
                        get_index(row, column + 1),
                        get_index(row, column - 1),
                        get_index(row + 1, column),
                        get_index(row + 1, column + 1),
                        get_index(row + 1, column - 1),
                    ]
                } else if column == 0 {
                    vec![
                        get_index(row, column + 1),
                        get_index(row + 1, column),
                        get_index(row - 1, column),
                        get_index(row + 1, column + 1),
                        get_index(row - 1, column + 1),
                    ]
                } else if row == (height - 1) {
                    vec![
                        get_index(row, column + 1),
                        get_index(row, column - 1),
                        get_index(row - 1, column),
                        get_index(row - 1, column + 1),
                        get_index(row - 1, column - 1),
                    ]
                } else if column == (width - 1) {
                    vec![
                        get_index(row, column - 1),
                        get_index(row + 1, column),
                        get_index(row - 1, column),
                        get_index(row + 1, column - 1),
                        get_index(row - 1, column - 1),
                    ]
                } else {
                    vec![
                        get_index(row, column + 1),
                        get_index(row, column - 1),
                        get_index(row + 1, column),
                        get_index(row - 1, column),
                        get_index(row + 1, column + 1),
                        get_index(row + 1, column - 1),
                        get_index(row - 1, column + 1),
                        get_index(row - 1, column - 1),
                    ]
                },
            }
        })
        .collect::<Vec<_>>();

    let mut flashes = 0;

    for _ in 0..100 {
        let mut triggered_squid = Vec::new();

        for index in 0..100 {
            energy_levels[index] += 1;
        }

        loop {
            let squids_to_trigger = squids
                .iter()
                .filter(|squid| energy_levels[squid.own_position] > 9)
                .filter(|squid| !triggered_squid.contains(&squid.own_position))
                .collect::<Vec<_>>();

            if squids_to_trigger.is_empty() {
                break;
            }

            triggered_squid.extend(
                squids_to_trigger
                    .iter()
                    .map(|squid| squid.own_position)
                    .collect::<Vec<_>>(),
            );

            squids_to_trigger.into_iter().for_each(|squid| {
                energy_levels[squid.own_position] = 0;
                flashes += 1;

                squid
                    .neighbors
                    .iter()
                    .filter(|neighbor| !triggered_squid.contains(neighbor))
                    .for_each(|neighbor| {
                        energy_levels[*neighbor] += 1;
                    });
            })
        }
    }

    println!("{:?}", flashes);
}

fn part_2() {
    let width = 10;
    let height = 10;

    let file = File::open("input").unwrap();

    let data = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10))
                .map(Option::unwrap)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut energy_levels = [0; 100];
    let squids = (0..width)
        .flat_map(|row| (0..height).map(move |column| (row, column)))
        .map(|(row, column)| {
            let get_index = |row, column| row * width + column;

            energy_levels[get_index(row, column)] = *data.get(row).unwrap().get(column).unwrap();

            Squid {
                own_position: row * width + column,
                neighbors: if row == 0 && column == 0 {
                    vec![
                        get_index(row, column + 1),
                        get_index(row + 1, column),
                        get_index(row + 1, column + 1),
                    ]
                } else if row == (height - 1) && column == (width - 1) {
                    vec![
                        get_index(row, column - 1),
                        get_index(row - 1, column),
                        get_index(row - 1, column - 1),
                    ]
                } else if row == 0 && column == (width - 1) {
                    vec![
                        get_index(row, column - 1),
                        get_index(row + 1, column),
                        get_index(row + 1, column - 1),
                    ]
                } else if row == (height - 1) && column == 0 {
                    vec![
                        get_index(row, column + 1),
                        get_index(row - 1, column),
                        get_index(row - 1, column + 1),
                    ]
                } else if row == 0 {
                    vec![
                        get_index(row, column + 1),
                        get_index(row, column - 1),
                        get_index(row + 1, column),
                        get_index(row + 1, column + 1),
                        get_index(row + 1, column - 1),
                    ]
                } else if column == 0 {
                    vec![
                        get_index(row, column + 1),
                        get_index(row + 1, column),
                        get_index(row - 1, column),
                        get_index(row + 1, column + 1),
                        get_index(row - 1, column + 1),
                    ]
                } else if row == (height - 1) {
                    vec![
                        get_index(row, column + 1),
                        get_index(row, column - 1),
                        get_index(row - 1, column),
                        get_index(row - 1, column + 1),
                        get_index(row - 1, column - 1),
                    ]
                } else if column == (width - 1) {
                    vec![
                        get_index(row, column - 1),
                        get_index(row + 1, column),
                        get_index(row - 1, column),
                        get_index(row + 1, column - 1),
                        get_index(row - 1, column - 1),
                    ]
                } else {
                    vec![
                        get_index(row, column + 1),
                        get_index(row, column - 1),
                        get_index(row + 1, column),
                        get_index(row - 1, column),
                        get_index(row + 1, column + 1),
                        get_index(row + 1, column - 1),
                        get_index(row - 1, column + 1),
                        get_index(row - 1, column - 1),
                    ]
                },
            }
        })
        .collect::<Vec<_>>();

    for step in 1.. {
        let mut triggered_squid = Vec::new();

        for index in 0..100 {
            energy_levels[index] += 1;
        }

        loop {
            let squids_to_trigger = squids
                .iter()
                .filter(|squid| energy_levels[squid.own_position] > 9)
                .filter(|squid| !triggered_squid.contains(&squid.own_position))
                .collect::<Vec<_>>();

            if squids_to_trigger.is_empty() {
                break;
            }

            triggered_squid.extend(
                squids_to_trigger
                    .iter()
                    .map(|squid| squid.own_position)
                    .collect::<Vec<_>>(),
            );

            if triggered_squid.len() == squids.len() {
                println!("{}", step);
                return;
            }

            squids_to_trigger.into_iter().for_each(|squid| {
                energy_levels[squid.own_position] = 0;

                squid
                    .neighbors
                    .iter()
                    .filter(|neighbor| !triggered_squid.contains(neighbor))
                    .for_each(|neighbor| {
                        energy_levels[*neighbor] += 1;
                    });
            })
        }
    }
}
