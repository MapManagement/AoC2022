use std::fs;

fn main() {
    let part1 = part1();

    println!("Part1: {}", part1);
}

struct Tile {
    x: usize,
    y: usize
}

struct Cave {
    left_border: usize,
    right_border: usize,
    bottom_border: usize,
    structure: Vec<Vec<String>>
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

fn part1() -> i32 {
    let rock_positions = read_file_lines();
    let rock_tiles = extract_rock_tiles(rock_positions);
    let cave = draw_cave(rock_tiles);
    let sand_counter = start_sand_dropping(cave);

    sand_counter
}

fn extract_rock_tiles(rock_positions: Vec<String>) -> Vec<Tile> {
    let mut rock_tiles: Vec<Tile> = Vec::new();

    for rock_position in rock_positions {
        let tiles = rock_position.split(" -> ").collect::<Vec<&str>>();

        for tile in tiles {
            let pos = tile.split(",").collect::<Vec<&str>>();

            let rock_tile = Tile {
                x: pos[0].parse::<usize>().unwrap(),
                y: pos[1].parse::<usize>().unwrap()
            };

            rock_tiles.push(rock_tile);
        }
    }

    rock_tiles
}

fn draw_cave(tiles: Vec<Tile>) -> Cave {
    let lowest_rock = tiles.iter().max_by_key(|tile| tile.y).unwrap();
    let left_rock = tiles.iter().min_by_key(|tile| tile.x).unwrap();
    let right_rock = tiles.iter().max_by_key(|tile| tile.x).unwrap();

    let mut cave: Vec<Vec<String>> = vec![vec![".".to_string(); right_rock.x - left_rock.x + 1]; lowest_rock.y + 1];

    for i in 0..tiles.len() - 1 {
        let x_delta = tiles[i].x.abs_diff(tiles[i + 1].x);
        let y_delta = tiles[i].y.abs_diff(tiles[i + 1].y);

        if x_delta == 0 {
            let smaller = if tiles[i].y < tiles[i + 1].y {
                tiles[i].y
            } else {
                tiles[i + 1].y
            };

            for j in 0..=y_delta {
                cave[smaller + j][tiles[i].x - left_rock.x] = "#".to_string();
            }
        }
        else if y_delta == 0 {
            let smaller = if tiles[i].x < tiles[i + 1].x {
                tiles[i].x
            } else {
                tiles[i + 1].x
            };

            for j in 0..=x_delta {
                cave[tiles[i].y][smaller + j - left_rock.x] = "#".to_string();
            }
        }
    }
    
    Cave {
        left_border: left_rock.x,
        right_border: right_rock.x,
        bottom_border: lowest_rock.y,
        structure: cave
    }
}

fn start_sand_dropping(mut cave: Cave) -> i32 {
    let mut is_flowing_out = false;
    let start = Tile { x: 500 - cave.left_border, y: 0 };
    let mut sand_counter = 0;

    while !is_flowing_out {
        let mut sand_x = start.x;
        let mut sand_y = start.y;

        loop {
            if sand_y == cave.bottom_border || sand_x == 0 || sand_x == cave.right_border - cave.left_border {
                is_flowing_out = true;
                break;
            }

            if cave.structure[sand_y + 1][sand_x] == ".".to_string() {
                sand_y += 1;
            }
            else {
                if cave.structure[sand_y + 1][sand_x - 1] == ".".to_string() {
                    sand_y += 1;
                    sand_x -= 1;
                }
                else {
                    if cave.structure[sand_y + 1][sand_x + 1] == ".".to_string() {
                        sand_y += 1;
                        sand_x += 1;
                    }
                    else {
                        cave.structure[sand_y][sand_x] = "O".to_string();
                        sand_counter += 1;
                        break;
                    }
                }
            }
        }
    }

    sand_counter
}
