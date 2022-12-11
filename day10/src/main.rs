use std::fs;

fn main() {
    let sum_signal_strength = iterate_cycles();
    println!("{}", sum_signal_strength);
}

fn read_file_lines() -> Vec<String> {
    let file_content = fs::read_to_string("input")
        .expect("Cannot read the given file");

    let lines: Vec<String> = file_content.split("\n")
        .map(str::to_string)
        .collect();

    lines
}

fn iterate_cycles() -> i32 {
    let lines = read_file_lines();
    let mut cycles = 0;
    let mut x = 1;
    let mut sum_signal_strength = 0;
    
    for i in 0..lines.len() {
        let split_line: Vec<&str> = lines[i].split(" ").collect();

        if split_line.len() == 0 {
            continue;
        }

        if split_line.len() == 1 {
            cycles += 1;
            sum_signal_strength += get_signal_strength(&cycles, &x);
            continue;
        }

        cycles += 1;
        sum_signal_strength += get_signal_strength(&cycles, &x);

        cycles += 1; 
        sum_signal_strength += get_signal_strength(&cycles, &x);

        x += split_line[1].parse::<i32>().unwrap();
    }

    sum_signal_strength
}

fn check_cycle(cycle: &i32) -> bool {
    match &cycle {
        20 | 60 | 100 | 140 | 180 | 220 => true,
        _ => false
    }
}

fn get_signal_strength(cycle: &i32, x_value: &i32) -> i32 {
    if !check_cycle(cycle) {
        return 0;
    }
    println!("C{}, X{}", cycle, x_value);

    cycle * x_value
}
