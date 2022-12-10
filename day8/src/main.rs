use std::fs;

struct VisibilitySides {
    left: bool,
    top: bool,
    right: bool,
    bottom: bool
}

impl VisibilitySides {
    fn any_visible(self) -> bool {
        return self.left || self.top || self.right || self.bottom;
    }
}

fn main() {
    let trees = create_tree_vec();
    let count = count_visible_trees(&trees);

    println!("{}", count);
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

fn count_visible_trees(trees: &Vec<Vec<u32>>) -> u32 {
    let mut counter = 0;
    println!("{}", trees.len());

    for tree_row in 0..trees.len() {
        if tree_row == 0 || tree_row == (trees.len() - 1) {
            counter += 1;
            continue;
        }

        for tree_column in 0..trees[tree_row].len() {
            if tree_column == 0 || tree_column == (trees[tree_row].len() - 1) {
                counter += 1;
                continue;
            }

            let is_visible = check_visibility(tree_column, tree_row, trees);
            if is_visible {
                counter += 1;
            }
        }
    }

    counter
}

fn check_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> bool {
    let mut visibility = VisibilitySides {
        left: true,
        top: true,
        right: true,
        bottom: true
    };

    visibility.top = check_top_visibility(tree_column, tree_row, trees);
    visibility.right = check_right_visibility(tree_column, tree_row, trees);
    visibility.bottom = check_bottom_visibility(tree_column, tree_row, trees);
    visibility.left = check_left_visibility(tree_column, tree_row, trees);

    let is_visible = visibility.any_visible();

    return is_visible;
}


fn check_top_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> bool {
    let tree = &trees[tree_row][tree_column];

    for row in 0..tree_row {
        let comparer = trees[row][tree_column];

        if tree <= &comparer {
            return false;
        }
    }

    return true;
}

fn check_bottom_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> bool {
    let tree = &trees[tree_row][tree_column];

    for row in (tree_row + 1)..trees.len() {
        let comparer = trees[row][tree_column];

        if tree <= &comparer {
            return false;
        }
    }

    return true;
}

fn check_left_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> bool {
    let tree = &trees[tree_row][tree_column];

    for column in 0..tree_column {
        let comparer = trees[tree_row][column];

        if tree <= &comparer {
            return false;
        }
    }

    return true;
}
fn check_right_visibility(tree_column: usize, tree_row: usize, trees: &Vec<Vec<u32>>) -> bool {
    let tree = &trees[tree_row][tree_column];

    for column in (tree_column + 1)..trees.len() {
        let comparer = trees[tree_row][column];

        if tree <= &comparer {
            return false;
        }
    }

    return true;
}
