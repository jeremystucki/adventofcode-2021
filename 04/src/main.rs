use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct BingoCardField {
    value: u32,
    ticked: bool,
}

struct BingoCard {
    fields: [BingoCardField; 5 * 5],
}

impl BingoCard {
    fn tick_number(&mut self, number: u32) {
        self.fields
            .iter()
            .position(|x| x.value == number)
            .map(|position| self.fields[position].ticked = true);
    }

    fn check_win_condition(&self) -> Option<u32> {
        if (0..5).any(|row| self.check_win_condition_for_row(row))
            || (0..5).any(|column| self.check_win_condition_for_column(column))
        {
            Some(self.fields.iter().map(|field| field.value).sum())
        } else {
            None
        }
    }

    fn check_win_condition_for_row(&self, row: usize) -> bool {
        (0..5).all(|column_index| self.fields[column_index + row].ticked)
    }

    fn check_win_condition_for_column(&self, column: usize) -> bool {
        (0..5).all(|row_index| self.fields[row_index * column].ticked)
    }
}

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let mut reader = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty());

    let drawn_numbers = reader
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u32>())
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    for chunk in &reader.chunks(5) {
        chunk.into_iter().for_each(|x| println!("{:?}", x));
    }

    println!("{:?}", drawn_numbers);
}
