use std::fs;

fn main() {
    let part1 = part1();

    println!("Part1: {}", part1);
}

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
}

struct Cave {
    left_border: usize,
    right_border: usize,
    bottom_border: usize,
    structure: Vec<Vec<String>>,
}

enum SandState {
    Falling,
    Resting,
    Flowing
}

fn read_file_lines() -> Vec<String> {
    let file_content = fs::read_to_string("input").expect("Cannot read the given file");
    let mut lines: Vec<String> = file_content.split("\n").map(str::to_string).collect();
    lines.pop();

    lines
}

fn part1() -> i32 {
    let rock_positions = read_file_lines();
    let rock_tiles = extract_rock_tiles(rock_positions);
    let cave = draw_cave(rock_tiles);
    let sand_counter = start_sand_dropping(cave);

    sand_counter
}

fn extract_rock_tiles(rock_positions: Vec<String>) -> Vec<Vec<Tile>> {
    let mut rock_tiles: Vec<Vec<Tile>> = Vec::new();

    for i in 0..rock_positions.len() {
        let tiles = rock_positions[i].split(" -> ").collect::<Vec<&str>>();
        rock_tiles.push(Vec::new());

        for tile in tiles {
            let pos = tile.split(",").collect::<Vec<&str>>();

            let rock_tile = Tile {
                x: pos[0].parse::<usize>().unwrap(),
                y: pos[1].parse::<usize>().unwrap(),
            };

            rock_tiles[i].push(rock_tile);
        }
    }

    rock_tiles
}

fn draw_cave(tiles: Vec<Vec<Tile>>) -> Cave {
    let (left_border, right_border, ground) = find_cave_borders(&tiles);

    let mut cave: Vec<Vec<String>> =
        vec![vec![".".to_string(); right_border - left_border + 1]; ground + 1];
    for tile_structure in tiles {
        for i in 0..tile_structure.len() - 1 {
            let x_delta = tile_structure[i].x.abs_diff(tile_structure[i + 1].x);
            let y_delta = tile_structure[i].y.abs_diff(tile_structure[i + 1].y);

            if x_delta == 0 {
                let smaller = if tile_structure[i].y < tile_structure[i + 1].y {
                    tile_structure[i].y
                } else {
                    tile_structure[i + 1].y
                };

                for j in 0..=y_delta {
                    cave[smaller + j][tile_structure[i].x - left_border] = "#".to_string();
                }
            } else if y_delta == 0 {
                let smaller = if tile_structure[i].x < tile_structure[i + 1].x {
                    tile_structure[i].x
                } else {
                    tile_structure[i + 1].x
                };

                for j in 0..=x_delta {
                    cave[tile_structure[i].y][smaller + j - left_border] = "#".to_string();
                }
            }
        }
    }

    Cave {
        left_border,
        right_border,
        bottom_border: ground,
        structure: cave,
    }
}

fn find_cave_borders(tiles: &Vec<Vec<Tile>>) -> (usize, usize, usize) {
    let mut left_border = 0;
    let mut right_border = 0;
    let mut ground = 0;

    for tile_structure in tiles {
        for tile in tile_structure {
            if tile.x > right_border {
                right_border = tile.x;
            } else if tile.x < left_border {
                left_border = tile.x;
            }

            if tile.y > ground {
                ground = tile.y;
            }
        }
    }

    (left_border, right_border, ground)
}

fn start_sand_dropping(mut cave: Cave) -> i32 {
    let mut is_flowing_out = false;
    let start = Tile {
        x: 500 - cave.left_border,
        y: 0,
    };
    let mut sand_counter = 0;

    while !is_flowing_out {
        let mut sand_x = start.x;
        let mut sand_y = start.y;

        loop {
            let state = check_sand_underground(&mut cave, &mut sand_x, &mut sand_y);

            match state {
                SandState::Falling => { continue; },
                SandState::Resting => {
                    sand_counter += 1;
                    break;
                },
                SandState::Flowing => {
                    is_flowing_out = true;
                    break;
                }
            }
        }
    }

    sand_counter
}

fn check_sand_underground(cave: &mut Cave, sand_x: &mut usize, sand_y: &mut usize) -> SandState {
    let y = *sand_y;
    let x = *sand_x;

    if y == cave.bottom_border || x == 0 {
        return SandState::Flowing;
    }

    if cave.structure[y + 1][x] == ".".to_string() {
        *sand_y += 1;
        return SandState::Falling;
    }

    if cave.structure[y + 1][x - 1] == ".".to_string() {
        *sand_y += 1;
        *sand_x -= 1;
        return SandState::Falling;
    }

    if x == cave.right_border - cave.left_border {
        return SandState::Flowing;
    }

    if cave.structure[y + 1][x + 1] == ".".to_string() {
        *sand_y += 1;
        *sand_x += 1;
        return SandState::Falling;
    } else {
        cave.structure[y][x] = "O".to_string();
        return SandState::Resting;
    }
}
