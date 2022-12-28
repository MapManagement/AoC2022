use std::fs;

fn main() {
    let part1 = get_tail_positions(2);
    let part2 = get_tail_positions(10);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
    None
}

struct Motion {
    steps: i32,
    direction: Direction
}

#[derive(Clone)]
struct Point {
    y: i32,
    x: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
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

fn get_tail_positions(rope_length: usize) -> usize {
    let lines = read_file_lines();
    let mut rope =  vec![Point { x: 0, y: 0 }; rope_length];
    let mut all_tail_pos: Vec<Point> = Vec::new();

    for line in lines {
        let motion = parse_line(line);

        for _ in 0..motion.steps {
            match motion.direction {
                Direction::Left =>  rope[0].x -= 1,
                Direction::Right => rope[0].x += 1,
                Direction::Up => rope[0].y += 1,
                Direction::Down => rope[0].y -= 1,
                Direction::None => { } 
            };

            for i in 1..rope.len() {
                let prev = i - 1;

                if (rope[prev].x - rope[i].x).abs() > 1 {
                    rope[i].x += (rope[prev].x - rope[i].x).signum();

                    if (rope[prev].y - rope[i].y).abs() >= 1 {
                        rope[i].y += (rope[prev].y - rope[i].y).signum();
                    }
                }
                
                if (rope[prev].y - rope[i].y).abs() > 1 {
                    rope[i].y += (rope[prev].y - rope[i].y).signum();

                    if (rope[prev].x - rope[i].x).abs() >= 1 {
                        rope[i].x += (rope[prev].x - rope[i].x).signum();
                    }
                }
                                
                if i == rope.len() - 1 {
                    expand_tail_pos(&mut all_tail_pos, &rope[i]);
                }
            }
        }
    }

    return all_tail_pos.len();
}

fn expand_tail_pos(tail_pos_vec: &mut Vec<Point>, tail: &Point) {
    if !tail_pos_vec.contains(tail) {
        tail_pos_vec.push(tail.clone());
    }
}

fn parse_line(line: String) -> Motion {
    let line_parts = line.split(" ").collect::<Vec<&str>>();
    let direction = match  line_parts[0] {
        "L" => Direction::Left,
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        _ => Direction::None
    };

    let steps = line_parts[1].parse::<i32>().expect("Couldn't parse to i32.");

    Motion { steps, direction }
}

