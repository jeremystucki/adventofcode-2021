use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static CODE_WIDTH: usize = 12;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let digits = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let total_count = digits.len();

    let gamma: u32 = (0..CODE_WIDTH)
        .into_iter()
        .map(|index| {
            digits
                .iter()
                .map(move |row| row[index])
                .filter(|character| *character == '1')
                .count()
        })
        .enumerate()
        .filter(|(_, count)| *count > total_count / 2)
        .map(|(index, _)| 2u32.pow((CODE_WIDTH - 1 - index).try_into().unwrap()))
        .sum();

    let epsilon = 2u32.pow(CODE_WIDTH as u32) - 1 - gamma;

    println!("{:?}", epsilon * gamma);
}

fn part_2() {
    let file = File::open("input").unwrap();

    let digits = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let oxygen_rating = part_2_get_rating(digits.clone(), &|count, total_count| {
        if total_count % 2 == 0 {
            count >= total_count / 2
        } else {
            count > total_count / 2
        }
    });

    let scrubber_rating = part_2_get_rating(digits, &|count, total_count| {
        if total_count % 2 == 0 {
            count < total_count / 2
        } else {
            count <= total_count / 2
        }
    });

    println!("{:?}", oxygen_rating * scrubber_rating);
}

fn part_2_get_rating(
    mut digits: Vec<Vec<char>>,
    criteria_fn: &dyn Fn(usize, usize) -> bool,
) -> u32 {
    let mut current_filter_position = 0;

    loop {
        let number_of_ones = digits
            .iter()
            .map(move |row| row[current_filter_position])
            .filter(|character| *character == '1')
            .count();

        let criteria = if criteria_fn(number_of_ones, digits.len()) {
            '1'
        } else {
            '0'
        };

        digits = digits
            .into_iter()
            .filter(|row| row[current_filter_position] == criteria)
            .collect::<Vec<_>>();

        if digits.len() == 1 {
            break digits
                .pop()
                .unwrap()
                .iter()
                .enumerate()
                .filter(|(_, character)| **character == '1')
                .map(|(index, _)| 2u32.pow((CODE_WIDTH - index - 1) as u32))
                .sum();
        }

        current_filter_position += 1;
    }
}
