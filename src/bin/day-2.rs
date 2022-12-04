use advent_of_code_2022::get_input;

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn versus(&self, other: &Shape) -> u64 {
        match self {
            Shape::Rock => match other {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match other {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match other {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        }
    }

    fn score(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn letter_to_shape(letter: &str) -> Shape {
    match letter {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("Unknown letter {}", letter),
    }
}

fn xyz_to_shape(letter: &str, them: &Shape) -> Shape {
    match them {
        Shape::Rock => match letter {
            "X" => Shape::Scissors,
            "Y" => Shape::Rock,
            "Z" => Shape::Paper,
            _ => panic!("Unknown letter {}", letter),
        },
        Shape::Paper => match letter {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Unknown letter {}", letter),
        },
        Shape::Scissors => match letter {
            "X" => Shape::Paper,
            "Y" => Shape::Scissors,
            "Z" => Shape::Rock,
            _ => panic!("Unknown letter {}", letter),
        },
    }
}

fn main() {
    let _demo_input = "A Y
B X
C Z";
    let input = get_input("day-2/input");

    let points: u64 = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|round| {
            let vec = round.split(' ').map(letter_to_shape).collect::<Vec<_>>();
            (round, vec[0], vec[1])
        })
        .map(|(_raw, them, me)| me.score() + me.versus(&them))
        .sum();

    println!("Part one points: {:?}", points);

    let points: u64 = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|round| {
            let vec = round.split(' ').collect::<Vec<_>>();

            let them = letter_to_shape(vec[0]);
            (round, them, xyz_to_shape(vec[1], &them))
        })
        .map(|(_raw, them, me)| me.score() + me.versus(&them))
        .sum();

    println!("Part two points: {:?}", points)
}
