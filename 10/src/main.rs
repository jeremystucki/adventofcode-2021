use std::collections::LinkedList;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let result = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .try_fold(LinkedList::new(), |mut list, character| match character {
                    '(' | '[' | '{' | '<' => {
                        list.push_back(character);
                        Ok(list)
                    }
                    _ => match (list.pop_back().ok_or(character)?, character) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => Ok(list),
                        _ => Err(character),
                    },
                })
        })
        .filter_map(Result::err)
        .map(|incorrect_character| match incorrect_character {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Encountered unexpected symbol"),
        })
        .sum::<u128>();

    println!("{}", result);
}

fn part_2() {
    let file = File::open("input").unwrap();

    let mut points = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .try_fold(LinkedList::new(), |mut list, character| match character {
                    '(' | '[' | '{' | '<' => {
                        list.push_front(character);
                        Ok(list)
                    }
                    _ => match (list.pop_front().ok_or(character)?, character) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => Ok(list),
                        _ => Err(character),
                    },
                })
        })
        .filter_map(Result::ok)
        .map(|remaining_opening_characters| {
            remaining_opening_characters
                .into_iter()
                .fold(0u128, |score, character| {
                    (score * 5)
                        + match character {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => panic!("Encountered unexpected symbol"),
                        }
                })
        })
        .collect::<Vec<_>>();

    points.sort();

    println!("{}", points[points.len() / 2]);
}
