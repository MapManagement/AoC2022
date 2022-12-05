use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let number = iterate_file_lines();
    println!("{}", number);
}

struct AssignmentPair {
    start_first_elf: i32,
    end_first_elf: i32,
    start_second_elf: i32,
    end_second_elf: i32
}

fn read_file_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let input_file = File::open("input")?;

    Ok(io::BufReader::new(input_file).lines())
}

fn iterate_file_lines() -> i32 {
    if let Ok(lines) = read_file_lines() {

        let mut counter = 0;

        for line in lines {
            if let Ok(pair) = &line {
                let assignment_pair = get_assigment_pair(pair);
                if check_assignment_ranges(&assignment_pair) {
                    counter += 1;
                }
            }
        }

        return counter;
    }

    return 0;
}

fn get_assigment_pair(assignment_line: &String) -> AssignmentPair {
    let assignments: Vec<&str> = assignment_line.split(",").collect();

    let assignment_one = assignments[0];
    let assignment_two = assignments[1];
    let split_assignment_one: Vec<&str> = assignment_one.split("-").collect();
    let split_assignment_two: Vec<&str> = assignment_two.split("-").collect();

    let assignment_pair = AssignmentPair {
        start_first_elf: split_assignment_one[0].parse::<i32>().unwrap(),
        end_first_elf: split_assignment_one[1].parse::<i32>().unwrap(),
        start_second_elf: split_assignment_two[0].parse::<i32>().unwrap(),
        end_second_elf: split_assignment_two[1].parse::<i32>().unwrap()
    };

    assignment_pair
}

fn check_assignment_ranges(assignment_pair: &AssignmentPair) -> bool {
    let first_start_smaller = assignment_pair.start_first_elf < assignment_pair.start_second_elf;
    let first_end_greater = assignment_pair.end_first_elf > assignment_pair.end_second_elf;
    let start_equals = assignment_pair.start_first_elf == assignment_pair.start_second_elf;
    let end_equals = assignment_pair.end_first_elf == assignment_pair.end_second_elf;

    if (first_start_smaller || start_equals)  && (first_end_greater || end_equals) {
        return true;
    }

    if (!first_start_smaller || start_equals) && (!first_end_greater || end_equals) {
        return true;
    }

    return false;
}


