use std::fs;

fn main() {
    let part1 = part1();

    print!("Part 1: {}", part1);
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

fn part1() -> usize {
    let lines = read_file_lines();
    let mut tail = Point { x: 0, y: 0 };
    let mut head = Point { x: 0, y: 0 };
    let mut all_tail_pos: Vec<Point> = Vec::new();

    for line in lines {
        let motion = parse_line(line);

        for _ in 0..motion.steps {
            match motion.direction {
                Direction::Left =>  head.x -= 1,
                Direction::Right => head.x += 1,
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::None => { } 
            };

            if head.x == tail.x && head.y == tail.y {
                continue;
            }

            if head.x - tail.x > 1 {
                if head.y - tail.y == 1 {
                    tail.x += 1;
                    tail.y += 1;
                    if !all_tail_pos.contains(&tail) {
                        all_tail_pos.push(tail.clone());
                    }

                    continue;
                }
                else if head.y - tail.y == -1  {
                    tail.x += 1;
                    tail.y -= 1;
                    if !all_tail_pos.contains(&tail) {
                        all_tail_pos.push(tail.clone());
                    }

                    continue;
                }

                tail.x += 1;
            }
            else if head.x - tail.x < -1 {
                if head.y - tail.y == 1 {
                    tail.x -= 1;
                    tail.y += 1;
                    if !all_tail_pos.contains(&tail) {
                        all_tail_pos.push(tail.clone());
                    }

                    continue;
                }
                else if head.y - tail.y == -1  {
                    tail.x -= 1;
                    tail.y -= 1;
                    if !all_tail_pos.contains(&tail) {
                        all_tail_pos.push(tail.clone());
                    }

                    continue;
                }

                tail.x -= 1;
            }

            if head.y - tail.y > 1 {
                if head.x - tail.x == 1 {
                    tail.x += 1;
                    tail.y += 1;
                    if !all_tail_pos.contains(&tail) {
                        all_tail_pos.push(tail.clone());
                    }

                    continue;
                }
                else if head.x - tail.x == -1  {
                    tail.x -= 1;
                    tail.y += 1;
                    if !all_tail_pos.contains(&tail) {
                        all_tail_pos.push(tail.clone());
                    }

                    continue;
                }

                tail.y += 1;
            }
            else if head.y - tail.y < -1 {
                if head.x - tail.x == 1 {
                    tail.x += 1;
                    tail.y -= 1;
                    if !all_tail_pos.contains(&tail) {
                        all_tail_pos.push(tail.clone());
                    }

                    continue;
                }
                else if head.x - tail.x == -1  {
                    tail.x -= 1;
                    tail.y -= 1;
                    if !all_tail_pos.contains(&tail) {
                        all_tail_pos.push(tail.clone());
                    }

                    continue;
                }

                tail.y -= 1;
            }

            if !all_tail_pos.contains(&tail) {
                all_tail_pos.push(tail.clone());
            }
        }
    }

    return all_tail_pos.len();
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
