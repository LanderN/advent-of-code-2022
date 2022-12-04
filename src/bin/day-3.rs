use advent_of_code_2022::get_input;

fn priority(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        c as u8 - b'a' + 1
    } else {
        27 + (c as u8 - b'A')
    }
}

fn main() {
    let _demo_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let input = get_input("day-3/input");

    let result = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|r| r.split_at(r.len() / 2))
        .map(|(left, right)| left.chars().find(|c| right.contains(*c)).unwrap())
        .map(priority)
        .map(|n| n as u64)
        .sum::<u64>();

    println!("Part one: {:?}", result);

    let mut sacks = input.split('\n').filter(|s| !s.is_empty());
    let mut badge_sum: u64 = 0;
    while let Some(first) = sacks.next() {
        let second = sacks.next().unwrap();
        let third = sacks.next().unwrap();

        let common = first
            .chars()
            .filter(|c| second.contains(*c))
            .find(|c| third.contains(*c))
            .unwrap();

        badge_sum += priority(common) as u64;
    }

    println!("Part two: {:?}", badge_sum)
}
