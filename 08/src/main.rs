use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let output = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .flat_map(|line| {
            let (input_digits, output_digits) = line.split_once(" | ").unwrap();

            let input_digits = input_digits
                .split(' ')
                .map(|digit| {
                    let mut set = HashSet::new();
                    digit.chars().for_each(|c| {
                        set.insert(c);
                    });
                    set
                })
                .collect::<Vec<_>>();

            let figured_out_digits = figure_out_digits(input_digits);

            output_digits
                .split(' ')
                .map(|digit| {
                    let mut set = HashSet::new();
                    digit.chars().for_each(|c| {
                        set.insert(c);
                    });
                    set
                })
                .map(|digit| {
                    figured_out_digits
                        .iter()
                        .position(|figured_out_digit| figured_out_digit == &digit)
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .filter(|&x| x == 1 || x == 4 || x == 7 || x == 8)
        .count();

    println!("{}", output);
}

fn part_2() {
    let file = File::open("input").unwrap();

    let output = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (input_digits, output_digits) = line.split_once(" | ").unwrap();

            let input_digits = input_digits
                .split(' ')
                .map(|digit| {
                    let mut set = HashSet::new();
                    digit.chars().for_each(|c| {
                        set.insert(c);
                    });
                    set
                })
                .collect::<Vec<_>>();

            let figured_out_digits = figure_out_digits(input_digits);

            output_digits
                .split(' ')
                .map(|digit| {
                    let mut set = HashSet::new();
                    digit.chars().for_each(|c| {
                        set.insert(c);
                    });
                    set
                })
                .map(|digit| {
                    figured_out_digits
                        .iter()
                        .position(|figured_out_digit| figured_out_digit == &digit)
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .map(|digits| (digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]) as u128)
        .sum::<u128>();

    println!("{}", output);
}

fn figure_out_digits(mut input_digits: Vec<HashSet<char>>) -> Vec<HashSet<char>> {
    // 2 lines - 1: cf      -> figured out immediately
    // 3 lines - 7: acf     -> figured out immediately
    // 4 lines - 4: bcdf    -> figured out immediately
    // -----------------------------------------------
    // 5 lines - 2: acdeg   -> figured out after the other two
    //         - 3: acdfg   -> shares all elements of 7
    //         - 5: abdfg   -> shares 3 elements with 4 (figured out after 3)
    // -----------------------------------------------
    // 6 lines - 0: abcefg  -> shares all elements of 7 (figured out after 9)
    //         - 6: abdefg  -> figured out after other two
    //         - 9: abcdfg  -> shares all elements of 4
    // -----------------------------------------------
    // 7 lines - 8: abcdefg -> figured out immediately
    //
    // -> Order: 1,4,7,8,3,5,2,9,0,6

    let mut position = input_digits.iter().position(|x| x.len() == 2).unwrap();
    let one = input_digits.remove(position);

    position = input_digits.iter().position(|x| x.len() == 4).unwrap();
    let four = input_digits.remove(position);

    position = input_digits.iter().position(|x| x.len() == 3).unwrap();
    let seven = input_digits.remove(position);

    position = input_digits.iter().position(|x| x.len() == 7).unwrap();
    let eight = input_digits.remove(position);

    position = input_digits
        .iter()
        .position(|x| x.len() == 5 && x.is_superset(&seven))
        .unwrap();
    let three = input_digits.remove(position);

    position = input_digits
        .iter()
        .position(|x| x.len() == 5 && x.difference(&four).count() == 2)
        .unwrap();
    let five = input_digits.remove(position);

    position = input_digits.iter().position(|x| x.len() == 5).unwrap();
    let two = input_digits.remove(position);

    position = input_digits
        .iter()
        .position(|x| x.is_superset(&four))
        .unwrap();
    let nine = input_digits.remove(position);

    position = input_digits
        .iter()
        .position(|x| x.is_superset(&seven))
        .unwrap();
    let zero = input_digits.remove(position);

    let six = input_digits.remove(0);

    vec![zero, one, two, three, four, five, six, seven, eight, nine]
}
