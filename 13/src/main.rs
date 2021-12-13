use inline_python::python;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum FoldDirection {
    X,
    Y,
}

fn main() {
    let file = File::open("input").unwrap();

    let mut file_reader = BufReader::new(file).lines().map(Result::unwrap);

    let mut points = file_reader
        .by_ref()
        .map_while(|line| {
            line.split_once(',').map(|(x, y)| Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
        })
        .collect::<Vec<_>>();

    let fold_directions = file_reader
        .map(|line| {
            if line.contains('x') {
                FoldDirection::X
            } else {
                FoldDirection::Y
            }
        })
        .collect::<Vec<_>>();

    let mut width = points.iter().map(|point| point.x).max().unwrap();
    let mut height = points.iter().map(|point| point.y).max().unwrap();

    for (index, fold_direction) in fold_directions.into_iter().enumerate() {
        match fold_direction {
            FoldDirection::X => {
                width = width / 2;
                points = points
                    .into_iter()
                    .filter(|point| point.x != width)
                    .map(|Point { x, y }| Point {
                        x: if x < width { x } else { (width * 2) - x },
                        y,
                    })
                    .collect();
            }
            FoldDirection::Y => {
                height = height / 2;
                points = points
                    .into_iter()
                    .filter(|point| point.y != height)
                    .map(|Point { x, y }| Point {
                        x,
                        y: if y <= height { y } else { (height * 2) - y },
                    })
                    .collect();
            }
        }

        points.sort();
        points.dedup();

        if index == 0 {
            println!("Part 1: Nr of points after first fold: {}", points.len())
        }
    }

    let x = points.iter().map(|point| point.x).collect::<Vec<_>>();
    let y = points
        .iter()
        .map(|point| -(point.y as i32))
        .collect::<Vec<_>>();

    python! {
        import matplotlib.pyplot as plt

        plt.scatter('x, 'y)
        plt.show()
    }

    // for y in 0..height {
    //     for x in 0..width {
    //         print!(
    //             "{}",
    //             if points.contains(&Point { x, y }) {
    //                 'X'
    //             } else {
    //                 ' '
    //             }
    //         );
    //     }

    //     print!("\n");
    // }
}
