use advent_of_code_2022::get_input;

fn main() {
    let input = get_input("day-1/day-1.txt");

    /* PART ONE */
    let max_elf = (input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .filter(|value| !value.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .enumerate()
        .max_by(|x, y| x.1.cmp(&y.1)))
    .unwrap();

    println!(
        "The elf with the most calories ({}) is {}",
        max_elf.1, max_elf.0
    );

    /* PART TWO */
    let mut max_elves = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .filter(|value| !value.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    max_elves.sort();

    println!(
        "The top 3 elves carry {:?} calories",
        max_elves.iter().rev().take(3).sum::<i32>()
    )
}
