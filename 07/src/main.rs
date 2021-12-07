#![feature(int_abs_diff)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let mut numbers = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    numbers.sort();

    let min = *numbers.first().unwrap();
    let max = *numbers.last().unwrap();

    let mut fuel_costs = (min..=max)
        .map(|alignment_point| {
            numbers
                .iter()
                .map(|&number| number.abs_diff(alignment_point))
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    fuel_costs.sort();

    println!("{}", fuel_costs.first().unwrap());
}

fn part_2() {
    let file = File::open("input").unwrap();

    let mut numbers = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    numbers.sort();

    let min = *numbers.first().unwrap();
    let max = *numbers.last().unwrap();

    let mut fuel_costs = (min..=max)
        .map(|alignment_point| {
            numbers
                .iter()
                .map(|&number| {
                    let n = number.abs_diff(alignment_point);
                    n * (n + 1) / 2
                })
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    fuel_costs.sort();

    println!("{}", fuel_costs.first().unwrap());
}
