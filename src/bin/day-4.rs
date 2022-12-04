use advent_of_code_2022::get_input;
use itertools::Itertools;

fn main() {
    let _demo_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let input = get_input("day-4/input");

    let result = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|pair| {
            let (first, second) = pair
                .split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|v| v.parse().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .collect_tuple::<((i32, i32), (i32, i32))>()
                .unwrap();

            (first.0 <= second.0 && first.1 >= second.1)
                || (second.0 <= first.0 && second.1 >= first.1)
        })
        .filter(|t| *t)
        .count();

    println!("Part one: {:?}", result);

    let result = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|pair| {
            let (mut first, mut second) = pair
                .split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|v| v.parse().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .collect_tuple::<((i32, i32), (i32, i32))>()
                .unwrap();

            // Make sure the first one is always the leftmost one (visually)
            if first.0 > second.0 {
                std::mem::swap(&mut first, &mut second);
            }

            second.0 <= first.1
        })
        .filter(|t| *t)
        .count();

    println!("Part two: {:?}", result);
}
