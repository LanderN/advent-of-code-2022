use std::collections::VecDeque;

use advent_of_code_2022::get_input;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Move {
    amount: i32,
    from: usize,
    to: usize,
}

fn main() {
    let _demo_input = "    [D]
[N] [C]
[Z] [M] [P]
    1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    let input = get_input("day-5/input");

    let input = input.split('\n');
    let stacks_input = input
        .clone()
        .take_while(|i| !i.is_empty())
        .collect::<Vec<_>>();
    let moves_input = input
        .skip_while(|i| !i.is_empty())
        .skip(1)
        .collect::<Vec<_>>();

    // Put the stacks in actual stacks
    let mut stacks: Vec<VecDeque<char>> = vec![];
    for row in stacks_input {
        let mut char_iterator = row.chars();
        let mut i = 0;
        while let Some((_, v, _)) = char_iterator.next_tuple() {
            if stacks.len() < i + 1 {
                stacks.push(VecDeque::new());
            }
            if v != ' ' {
                stacks[i].push_front(v);
            }
            char_iterator.next();
            i += 1;
        }
    }

    // Parse the moves
    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let mut moves: Vec<Move> = vec![];
    for m in moves_input {
        if let Some(caps) = re.captures(m) {
            moves.push(Move {
                amount: caps.get(1).unwrap().as_str().parse().unwrap(),
                from: caps.get(2).unwrap().as_str().parse().unwrap(),
                to: caps.get(3).unwrap().as_str().parse().unwrap(),
            });
        }
    }

    let mut stacks_part_one = stacks.clone();

    // Do the moves (part 1)
    for m in &moves {
        for _ in 0..m.amount {
            let v = stacks_part_one[m.from - 1].pop_back().unwrap();
            stacks_part_one[m.to - 1].push_back(v);
        }
    }

    let tops = stacks_part_one.iter().map(|s| s.back().unwrap()).join("");

    println!("Part one: {}", tops);

    // Do the moves (part 2)
    for m in &moves {
        let mut tmp_stack = VecDeque::<char>::new();
        for _ in 0..m.amount {
            tmp_stack.push_back(stacks[m.from - 1].pop_back().unwrap());
        }

        for v in tmp_stack.iter().rev() {
            stacks[m.to - 1].push_back(*v);
        }
    }

    let tops = stacks.iter().map(|s| s.back().unwrap()).join("");

    println!("Part two: {}", tops);
}
