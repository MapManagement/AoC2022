use std::fs;

struct Result {
    sum_signal_strength: i32,
    display: String
}

fn main() {
    let result = iterate_cycles();
    let sum_signal_strength = &result.sum_signal_strength;
    let display = &result.display;

    println!("{}", sum_signal_strength);
    println!("{}", display);
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

fn iterate_cycles() -> Result {
    let lines = read_file_lines();
    let mut cycles = 0;
    let mut x = 1;
    let mut sum_signal_strength = 0;
    let mut display = "".to_string();
    
    for i in 0..lines.len() {
        let split_line: Vec<&str> = lines[i].split(" ").collect();

        if split_line.len() == 0 {
            continue;
        }

        if split_line.len() == 1 {
            cycles += 1;
            sum_signal_strength += get_signal_strength(&cycles, &x);
            display += &display_output(&cycles, &x);

            continue;
        }

        cycles += 1;
        sum_signal_strength += get_signal_strength(&cycles, &x);
        display += &display_output(&cycles, &x);

        cycles += 1; 
        sum_signal_strength += get_signal_strength(&cycles, &x);
        display += &display_output(&cycles, &x);

        x += split_line[1].parse::<i32>().unwrap();
    }

    Result { sum_signal_strength, display }
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

    cycle * x_value
}

fn display_output(cycle: &i32, x_value: &i32) -> String {
    let x = x_value;
    let corrected_cycle = cycle % 40;
    let range = corrected_cycle - 2 == *x || corrected_cycle - 1 == *x || corrected_cycle == *x;
    let mut char: String;

    if range {
        char = "#".to_string();
    }
    else {
        char = ".".to_string();
    }

    if cycle % 40 == 0 {
        char += "\n";
    }

    return char;
}
