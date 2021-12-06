use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .ok_or(())?;

        Ok(Self { x, y })
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let count = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.split_once(" -> ")
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        })
        .map(Option::unwrap)
        .flat_map(|(start, end): (Point, Point)| {
            if start.x == end.x {
                (min(start.y, end.y)..=max(start.y, end.y))
                    .map(|y| Point { x: start.x, y })
                    .collect::<Vec<_>>()
            } else if start.y == end.y {
                (min(start.x, end.x)..=max(start.x, end.x))
                    .map(|x| Point { x, y: start.y })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        })
        .fold(HashMap::<Point, usize>::new(), |mut map, point| {
            (*map.entry(point).or_default() += 1);
            map
        })
        .into_iter()
        .map(|(_, count)| count)
        .filter(|count| *count != 1)
        .count();

    println!("{}", count);
}

fn part_2() {
    let file = File::open("input").unwrap();

    let count = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.split_once(" -> ")
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        })
        .map(Option::unwrap)
        .flat_map(|(mut start, mut end): (Point, Point)| -> Vec<_> {
            if start.x == end.x {
                (min(start.y, end.y)..=max(start.y, end.y))
                    .map(|y| Point { x: start.x, y })
                    .collect()
            } else if start.y == end.y {
                (min(start.x, end.x)..=max(start.x, end.x))
                    .map(|x| Point { x, y: start.y })
                    .collect()
            } else {
                if start.x > end.x {
                    mem::swap(&mut start, &mut end);
                }

                if start.y < end.y {
                    (0..=(end.x - start.x))
                        .map(|offset| Point {
                            x: start.x + offset as u32,
                            y: start.y + offset as u32,
                        })
                        .collect()
                } else {
                    (0..=(end.x - start.x))
                        .map(|offset| Point {
                            x: start.x + offset as u32,
                            y: start.y - offset as u32,
                        })
                        .collect()
                }
            }
        })
        .fold(HashMap::<Point, usize>::new(), |mut map, point| {
            (*map.entry(point).or_default() += 1);
            map
        })
        .into_iter()
        .map(|(_, count)| count)
        .filter(|count| *count != 1)
        .count();

    println!("{}", count);
}
