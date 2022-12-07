use std::convert::TryInto;
use std::fs;

fn main() {
    let sequence_length_first = 4;
    let position_first = find_marker_position(sequence_length_first);
    println!("First puzzle: {}", position_first);

    let sequence_length_second = 14;
    let position_second = find_marker_position(sequence_length_second);
    println!("Second puzzle: {}", position_second);
}


fn read_file() -> String {
    let file_content = fs::read_to_string("input")
        .expect("Cannot read the given file");

    file_content
}

fn find_marker_position(sequence_length: i32) -> i32 {
    let sequence = read_file();
    let mut marker: Vec<char> = Vec::new();
    let sequence_chars = sequence.chars();
    let mut position = 0;
    

    for char in sequence_chars {
        if marker.len() < sequence_length.try_into().unwrap() {
            marker.push(char);
        }
        else {
            if position > sequence_length * 2 - 2 {
                if check_marker(&marker, sequence_length) {
                    return position
                }
            }

            marker.insert(0, char);
            marker.pop();
        }
        position += 1;
    }

    return 0;
}

fn check_marker(marker: &Vec<char>, sequence_length: i32) -> bool {
    let mut count = 0;

    for c in marker {
        for x in marker {
            if c == x {
                count += 1;
            }
        }
    }

    if count == sequence_length {
        return true;
    }

    return false;
}
