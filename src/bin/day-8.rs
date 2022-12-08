use advent_of_code_2022::get_input;

fn main() {
    let _demo_input = "30373
25512
65332
33549
35390";
    let input = get_input("day-8/input");

    let mut trees: Vec<Vec<(i32, bool)>> = vec![];

    for row in input.lines() {
        let mut tree_row = vec![];
        for tree in row.chars() {
            tree_row.push((tree.to_string().parse().unwrap(), true));
        }
        trees.push(tree_row);
    }

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            if i == 0 || j == 0 || i == trees.len() - 1 || j == trees[i].len() - 1 {
                continue;
            }

            let tree_value = trees[i][j].0;

            // Visible from the left?
            let mut visible = true;
            for l in 0..j {
                if trees[i][l].0 >= tree_value {
                    visible &= false;
                    break;
                }
            }

            // Visible from the right?
            if !visible {
                visible = true;
                for l in (j + 1..trees[i].len()).rev() {
                    if trees[i][l].0 >= tree_value {
                        visible &= false;
                        break;
                    }
                }
            }
            // Visible from above
            if !visible {
                visible = true;
                for l in 0..i {
                    if trees[l][j].0 >= tree_value {
                        visible &= false;
                        break;
                    }
                }
            }
            // Visible from below
            if !visible {
                visible = true;
                for l in (i + 1..trees.len()).rev() {
                    if trees[l][j].0 >= tree_value {
                        visible &= false;
                        break;
                    }
                }
            }

            trees[i][j].1 = visible;
        }
    }

    // for row in &trees {
    //     for (tree, visible) in row {
    //         if *visible {
    //             print!(" {} ", tree);
    //         } else {
    //             print!("({})", tree);
    //         }
    //     }
    //     println!();
    // }

    let mut visible_total = 0;
    for row in &trees {
        for (_tree, visible) in row {
            if *visible {
                visible_total += 1;
            }
        }
    }
    println!("Part One: {}", visible_total);

    let mut max_scenic_score = 0;
    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            if i == 0 || j == 0 || i == trees.len() - 1 || j == trees[i].len() - 1 {
                continue;
            }

            let tree_value = trees[i][j].0;

            // Looking left
            let mut viewing_distance_left = 0;
            for l in (0..j).rev() {
                viewing_distance_left += 1;
                if trees[i][l].0 >= tree_value {
                    break;
                }
            }
            // Looking up
            let mut viewing_distance_up = 0;
            for l in (0..i).rev() {
                viewing_distance_up += 1;
                if trees[l][j].0 >= tree_value {
                    break;
                }
            }

            // Looking right
            let mut viewing_distance_right = 0;
            for l in j + 1..trees[i].len() {
                viewing_distance_right += 1;
                if trees[i][l].0 >= tree_value {
                    break;
                }
            }
            // Looking down
            let mut viewing_distance_down = 0;
            for l in i + 1..trees.len() {
                viewing_distance_down += 1;
                if trees[l][j].0 >= tree_value {
                    break;
                }
            }

            let scenic_score = viewing_distance_left
                * viewing_distance_up
                * viewing_distance_right
                * viewing_distance_down;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }

            // println!(
            //     "tree with value {} score left: {}, score up: {}, score right: {}, score down: {}, score: {}",
            //     tree_value,
            //     viewing_distance_left,
            //     viewing_distance_up,
            //     viewing_distance_right,
            //     viewing_distance_down,
            //     scenic_score
            // );
        }
    }
    println!("Part two: {}", max_scenic_score);
}
