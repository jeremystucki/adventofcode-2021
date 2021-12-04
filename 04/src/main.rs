use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct BingoCardField {
    value: u32,
    ticked: bool,
}

#[derive(Debug)]
struct BingoCard {
    fields: [BingoCardField; 5 * 5],
}

impl BingoCard {
    fn new(numbers: [u32; 5 * 5]) -> Self {
        Self {
            fields: numbers.map(|number| BingoCardField {
                value: number,
                ticked: false,
            }),
        }
    }

    fn tick_number(&mut self, number: u32) {
        self.fields
            .iter()
            .position(|x| x.value == number)
            .map(|position| self.fields[position].ticked = true);
    }

    fn check_win_condition(&self) -> bool {
        (0..5).any(|row| self.check_win_condition_for_row(row))
            || (0..5).any(|column| self.check_win_condition_for_column(column))
    }

    fn sum_of_unchecked_fields(&self) -> u32 {
        self.fields
            .iter()
            .filter(|field| !field.ticked)
            .map(|field| field.value)
            .sum()
    }

    fn check_win_condition_for_row(&self, row: usize) -> bool {
        (0..5).all(|column_index| self.fields[column_index + row * 5].ticked)
    }

    fn check_win_condition_for_column(&self, column: usize) -> bool {
        (0..5).all(|row_index| self.fields[row_index * 5 + column].ticked)
    }
}

fn main() {
    part_1();
    part_2();
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

    let mut bingo_cards = reader
        .chunks(5)
        .into_iter()
        .map(|chunk| {
            let mut numbers = [0u32; 5 * 5];

            chunk
                .collect::<Vec<_>>()
                .iter()
                .map(|number| number.as_str().split_whitespace().collect::<Vec<_>>())
                .flatten()
                .map(|number| number.parse::<u32>())
                .map(Result::unwrap)
                .enumerate()
                .for_each(|(index, number)| numbers[index] = number);

            BingoCard::new(numbers)
        })
        .collect::<Vec<_>>();

    let mut numbers_to_draw = drawn_numbers.iter();

    let (winner, last_drawn_number) = loop {
        let drawn_number = numbers_to_draw.next().unwrap();

        bingo_cards
            .iter_mut()
            .for_each(|card| card.tick_number(*drawn_number));

        if let Some(winner) = bingo_cards
            .iter()
            .filter(|card| card.check_win_condition())
            .next()
        {
            break (winner.sum_of_unchecked_fields(), drawn_number);
        }
    };

    println!("{:?}", winner * last_drawn_number);
}

fn part_2() {
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

    let mut bingo_cards = reader
        .chunks(5)
        .into_iter()
        .map(|chunk| {
            let mut numbers = [0u32; 5 * 5];

            chunk
                .collect::<Vec<_>>()
                .iter()
                .map(|number| number.as_str().split_whitespace().collect::<Vec<_>>())
                .flatten()
                .map(|number| number.parse::<u32>())
                .map(Result::unwrap)
                .enumerate()
                .for_each(|(index, number)| numbers[index] = number);

            BingoCard::new(numbers)
        })
        .collect::<Vec<_>>();

    let mut numbers_to_draw = drawn_numbers.iter();

    let (winner, last_drawn_number) = loop {
        let drawn_number = numbers_to_draw.next().unwrap();

        bingo_cards
            .iter_mut()
            .for_each(|card| card.tick_number(*drawn_number));

        let winner_indices = bingo_cards
            .iter()
            .enumerate()
            .filter(|(_, card)| card.check_win_condition())
            .map(|(index, _)| index)
            .rev()
            .collect::<Vec<_>>();

        if winner_indices.is_empty() {
            continue;
        }

        if bingo_cards.len() == 1 {
            break (
                bingo_cards[winner_indices[0]].sum_of_unchecked_fields(),
                drawn_number,
            );
        } else {
            for winner in winner_indices {
                bingo_cards.remove(winner);
            }
        }
    };

    println!("{:?}", winner * last_drawn_number);
}
