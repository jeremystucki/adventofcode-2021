use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let (final_count, _) = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u32>())
        .map(Result::unwrap)
        .fold((-1, 0), |(count, last_value), new_value| {
            if new_value > last_value {
                (count + 1, new_value)
            } else {
                (count, new_value)
            }
        });

    println!("{}", final_count);
}

fn part_2() {
    let file = File::open("input").unwrap();

    let (final_count, _) = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u32>())
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .windows(3)
        .map(|window| window.iter().sum())
        .fold((-1, 0), |(count, last_value), new_value| {
            if new_value > last_value {
                (count + 1, new_value)
            } else {
                (count, new_value)
            }
        });

    println!("{}", final_count);
}

// I wanted to extract the fold thingy, but it got a bit ugly
//
// trait IncraseCounter<U> {
//     fn count_increases(self, start_value: U) -> i32;
// }
//
// impl<T, U> IncraseCounter<U> for T where T: Iterator<Item=U>, U: Ord {
//     fn count_increases(self, start_value: U) -> i32 {
//         self.fold((-1, start_value), |(count, last_value), new_value| {
//             if new_value > last_value {
//                 (count + 1, new_value)
//             } else {
//                 (count, new_value)
//             }
//         }).0
//     }
// }
