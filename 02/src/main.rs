use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (movement, amount_string) = s.split_once(' ').ok_or(())?;
        let amount = amount_string.parse::<u32>().map_err(|_| ())?;

        match movement {
            "forward" => Ok(Movement::Forward(amount)),
            "down" => Ok(Movement::Down(amount)),
            "up" => Ok(Movement::Up(amount)),
            _ => Err(()),
        }
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input").unwrap();

    let (final_horizontal, final_depth) = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<Movement>())
        .map(Result::unwrap)
        .fold((0, 0), |(horizontal, depth), new_value| match new_value {
            Movement::Forward(amount) => (horizontal + amount, depth),
            Movement::Down(amount) => (horizontal, depth + amount),
            Movement::Up(amount) => (horizontal, depth - amount),
        });

    println!("{}", final_horizontal * final_depth);
}

fn part_2() {
    let file = File::open("input").unwrap();

    let ((final_horizontal, final_depth), _) = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<Movement>())
        .map(Result::unwrap)
        .fold(
            ((0, 0), 0),
            |((horizontal, depth), aim), new_value| match new_value {
                Movement::Forward(amount) => ((horizontal + amount, depth + amount * aim), aim),
                Movement::Down(amount) => ((horizontal, depth), aim + amount),
                Movement::Up(amount) => ((horizontal, depth), aim - amount),
            },
        );

    println!("{}", final_horizontal * final_depth);
}
