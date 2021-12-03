use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part_1();
}

fn part_1() {
    let code_width = 12;

    let file = File::open("input").unwrap();

    let digits = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let total_count = digits.len();

    let gamma: u32 = (0..code_width)
        .into_iter()
        .map(|index| {
            digits
                .iter()
                .map(move |row| row[index])
                .filter(|&character| character == '1')
                .count()
        })
        .enumerate()
        .filter(|(_, count)| *count > total_count / 2)
        .map(|(index, _)| 2u32.pow((code_width - index - 1) as u32))
        .sum();

    let epsilon = 2u32.pow(code_width as u32) - 1 - gamma;

    println!("{:?}", epsilon * gamma);
}
