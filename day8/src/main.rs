use std::fs;

struct TreeSideValues {
    is_visible: bool,
    scenic_score: u32
}

struct Result {
    visible_counter: u32,
    scenic_score: u32
}


fn main() {
    let trees = create_tree_vec();
    let result = count_visible_trees(&trees);

    println!("Visible trees: {}", result.visible_counter);
    println!("Best scenic score: {}", result.scenic_score);
}

fn read_file_lines() -> Vec<String> {
    let file_content = fs::read_to_string("input")
        .expect("Cannot read the given file");
    let mut lines: Vec<String> = file_content.split("\n")
        .map(str::to_string)
        .collect();
    lines.pop();

    lines
}

fn create_tree_vec() -> Vec<Vec<u32>> {
    let lines = read_file_lines();
    let mut tree_vec: Vec<Vec<u32>> = Vec::new();
    let mut row = 0;

    for line in lines {
        tree_vec.push(Vec::new());
        for char in line.chars() {
            tree_vec[row].push(char.to_digit(10).expect("No unsigned integer"));
        }
        row += 1;
    }

    tree_vec
}

fn count_visible_trees(trees: &Vec<Vec<u32>>) -> Result {
    let mut counter = 0;
    let mut best_scenic_score = 0;

    for tree_row in 0..trees.len() {
        for tree_column in 0..trees[tree_row].len() {
            if tree_row == 0 || tree_row == (trees.len() - 1) {
                counter += 1;
                continue;
            }

            if tree_column == 0 || tree_column == (trees[tree_row].len() - 1) {
                counter += 1;
                continue;
            }
            
            let tree_values = check_visibility(tree_column, tree_row, trees);

            if tree_values.is_visible {
                counter += 1;
            }

            if tree_values.scenic_score > best_scenic_score {
                best_scenic_score = tree_values.scenic_score;
            }
        }
    }

    let result = Result { visible_counter: counter, scenic_score: best_scenic_score };

    result
}

fn check_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> TreeSideValues {
    let mut tree = TreeSideValues {
        is_visible: false,
        scenic_score: 0
    };

    let top = check_top_visibility(tree_column, tree_row, trees);
    let right = check_right_visibility(tree_column, tree_row, trees);
    let bottom = check_bottom_visibility(tree_column, tree_row, trees);
    let left = check_left_visibility(tree_column, tree_row, trees);
    let score = top.scenic_score * right.scenic_score * bottom.scenic_score * left.scenic_score;
    let is_visible = top.is_visible || right.is_visible || bottom.is_visible || left.is_visible;

    println!("V{}| S{}", trees[tree_row][tree_column], score);

    tree.is_visible = is_visible;
    tree.scenic_score = score;

    return tree;
}

fn check_top_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> TreeSideValues {
    let mut side_tree = TreeSideValues { is_visible: true, scenic_score: 0 };
    let tree = &trees[tree_row][tree_column];

    for row in (0..tree_row).rev() {
        let comparer = trees[row][tree_column];
        side_tree.scenic_score += 1;
        println!("T{}", side_tree.scenic_score);

        if tree <= &comparer {
            side_tree.is_visible = false;
            return side_tree;
        }
    }

    return side_tree;
}

fn check_bottom_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> TreeSideValues {
    let mut side_tree = TreeSideValues { is_visible: true, scenic_score: 0 };
    let tree = &trees[tree_row][tree_column];

    for row in (tree_row + 1)..trees.len() {
        let comparer = trees[row][tree_column];
        side_tree.scenic_score += 1;
        println!("B{}", side_tree.scenic_score);

        if tree <= &comparer {
            side_tree.is_visible = false;
            return side_tree;
        }
    }

    return side_tree;
}

fn check_left_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> TreeSideValues {
    let mut side_tree = TreeSideValues { is_visible: true, scenic_score: 0 };
    let tree = &trees[tree_row][tree_column];

    for column in (0..tree_column).rev() {
        let comparer = trees[tree_row][column];
        side_tree.scenic_score += 1;
        println!("L{}", side_tree.scenic_score);

        if tree <= &comparer {
            side_tree.is_visible = false;
            return side_tree;
        }
    }

    return side_tree;
}

fn check_right_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> TreeSideValues {
    let mut side_tree = TreeSideValues { is_visible: true, scenic_score: 0 };
    let tree = &trees[tree_row][tree_column];

    for column in (tree_column + 1)..trees.len() {
        let comparer = trees[tree_row][column];
        side_tree.scenic_score += 1;
        println!("R{}", side_tree.scenic_score);

        if tree <= &comparer {
            side_tree.is_visible = false;
            return side_tree;
        }
    }

    return side_tree;
}
