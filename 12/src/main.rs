#![feature(hash_set_entry)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Cave {
    is_big: bool,
    name: String,
}

fn main() {
    part_1();
}

fn is_cave_big(name: &str) -> bool {
    name.chars().next().unwrap().is_ascii_uppercase()
}

fn part_1() {
    let file = File::open("input").unwrap();

    let mut caves = HashSet::<Cave>::new();
    let mut connections = HashMap::<&Cave, Vec<&Cave>>::new();

    for line in BufReader::new(file).lines().map(Result::unwrap) {
        let (from_name, to_name) = line
            .split_once('-')
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .unwrap();

        let from_cave = Cave {
            is_big: is_cave_big(&from_name),
            name: from_name,
        };

        let to_cave = Cave {
            is_big: is_cave_big(&to_name),
            name: to_name,
        };

        let from_cave = caves.get_or_insert(from_cave);
        let to_cave = caves.get_or_insert(to_cave);

        connections
            .entry(&from_cave)
            .and_modify(|values| values.push(&to_cave))
            .or_insert_with(|| vec![&to_cave]);

        connections
            .entry(&to_cave)
            .and_modify(|values| values.push(&from_cave))
            .or_insert_with(|| vec![&from_cave]);
    }

    dbg!(caves);
    dbg!(connections);
}
