use std::{cmp::Ordering, collections::HashSet};

use advent_of_code_2022::get_input;
use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => panic!("Unknown direction!"),
        }
    }
}

fn main() {
    let _demo_input = "L 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let input = get_input("day-9/input");

    let mut h: (i32, i32) = (0, 0);
    let mut t: (i32, i32) = (0, 0);

    let mut t_positions = HashSet::<(i32, i32)>::new();
    t_positions.insert(t);

    let motions = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(direction, steps)| {
            (
                direction.parse::<Direction>().unwrap(),
                steps.parse::<i32>().unwrap(),
            )
        });

    for (direction, steps) in motions {
        // println!("{:?} {}", direction, steps);
        for _ in 0..steps {
            match direction {
                Direction::Up => h.1 += 1,
                Direction::Right => h.0 += 1,
                Direction::Down => h.1 -= 1,
                Direction::Left => h.0 -= 1,
            }

            // Calculate chebyshev distance
            let new_distance = t.0.abs_diff(h.0).max(t.1.abs_diff(h.1));
            if new_distance > 1 {
                // Tail needs to follow!

                // Follow axis of travel
                match direction {
                    Direction::Up => {
                        t.1 += 1;
                    }
                    Direction::Right => {
                        t.0 += 1;
                    }
                    Direction::Down => {
                        t.1 -= 1;
                    }
                    Direction::Left => {
                        t.0 -= 1;
                    }
                }
                // We might need to travel cross axis too
                match direction {
                    Direction::Up | Direction::Down => match h.0.cmp(&t.0) {
                        Ordering::Greater => t.0 += 1,
                        Ordering::Less => t.0 -= 1,
                        _ => (),
                    },
                    Direction::Right | Direction::Left => match h.1.cmp(&t.1) {
                        Ordering::Greater => t.1 += 1,
                        Ordering::Less => t.1 -= 1,
                        _ => (),
                    },
                }
                assert_ne!(h, t);

                t_positions.insert(t);
            }

            // let min_x = t_positions.iter().map(|(x, _)| x).min().unwrap();
            // let min_y = t_positions.iter().map(|(_, y)| y).min().unwrap();
            // let max_x = t_positions.iter().map(|(x, _)| x).max().unwrap();
            // let max_y = t_positions.iter().map(|(_, y)| y).max().unwrap();
            // for y in (*min_y - 1..=*max_y + 1).rev() {
            //     for x in *min_x - 1..=*max_x + 1 {
            //         if (x, y) == (0, 0) {
            //             print!("s")
            //         } else if h == (x, y) {
            //             print!("H")
            //         } else if t == (x, y) {
            //             print!("T")
            //         } else {
            //             print!(".")
            //         }
            //     }
            //     println!()
            // }
            // println!();

            assert!(t.0.abs_diff(h.0).max(t.1.abs_diff(h.1)) <= 1);
        }
    }

    println!("H: {:?}, T: {:?}", h, t);
    // println!("T positions: {:?}", t_positions);
    // let max_x = t_positions.iter().map(|(x, _)| x).max().unwrap();
    // let max_y = t_positions.iter().map(|(_, y)| y).max().unwrap();
    // let min_x = t_positions.iter().map(|(x, _)| x).min().unwrap();
    // let min_y = t_positions.iter().map(|(_, y)| y).min().unwrap();

    // for y in (*min_y..=*max_y).rev() {
    //     for x in *min_x..=*max_x + 1 {
    //         if (x, y) == (0, 0) {
    //             print!("s")
    //         } else if t_positions.contains(&(x, y)) {
    //             print!("#")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!()
    // }

    println!("T positions amount: {:?}", t_positions.len());
}
