use std::collections::VecDeque;

use advent_of_code_2022::get_input;
use itertools::Itertools;

fn has_dup<T: PartialEq>(w: &VecDeque<T>) -> bool {
    for i in 1..w.len() {
        if w.range(i..).contains(&w[i - 1]) {
            return true;
        }
    }
    false
}

fn find_marker(input: &str, unique_chars_required: usize) -> usize {
    let mut sliding_window = VecDeque::<char>::new();
    for (i, c) in input.char_indices() {
        sliding_window.push_back(c);
        if sliding_window.len() > unique_chars_required {
            sliding_window.pop_front();
            if !has_dup(&sliding_window) {
                return i + 1;
            }
        }
    }

    0
}

fn main() {
    let _demo_inputs = vec![
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    let input = get_input("day-6/input");

    for i in &_demo_inputs {
        println!("Part one demo input: {}", find_marker(i, 4));
    }
    println!("Part one: {}", find_marker(&input, 4));

    for i in &_demo_inputs {
        println!("Part two demo input: {}", find_marker(i, 14));
    }
    println!("Part two: {}", find_marker(&input, 14));
}
