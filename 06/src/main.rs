use std::collections::BinaryHeap;
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
        .map(|number| -number.parse::<i16>().unwrap())
        .collect::<BinaryHeap<_>>();

    for current_day in 0..80 {
        'inner: loop {
            if numbers.peek() == Some(&-current_day) {
                numbers.pop();
                numbers.push(-7 - current_day);
                numbers.push(-9 - current_day);
            } else {
                break 'inner;
            }
        }
    }

    println!("{:?}", numbers.len());
}

fn part_2() {
    let file = File::open("input").unwrap();

    let mut counts = [0u128; 9];

    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<i16>().unwrap())
        .for_each(|number| counts[number as usize] += 1);

    for current_day in 0..256 {
        let relative_day = current_day % 9;
        counts[(relative_day + 7) % 9] += counts[relative_day];
    }

    println!("{}", counts.iter().sum::<u128>());
}
