pub mod structs;

use std::borrow::BorrowMut;
use std::fs;
use std::rc::Rc;
use structs::{Monkey, Operator};


fn main() {
    
}

fn read_monkey_strings() -> Vec<String> {
    let file_content = fs::read_to_string("input")
        .expect("Cannot read the given file");
    let mut lines: Vec<String> = file_content.split("\n\n")
        .map(str::to_string)
        .collect();
    lines.pop();

    lines
}

fn extract_monkeys() -> Vec<Rc<Monkey>> {
    let monkey_strings = read_monkey_strings();
    let mut monkeys = Vec::new();

    for monkey_string in &monkey_strings {
        let lines: Vec<&str> = monkey_string.split("\n").collect();
        monkeys.push(Rc::new(create_monkey(lines)));
    }

    for i in 0..monkeys.len() {
        let lines: Vec<&str> = monkey_strings[i].split("\n").collect();
        let true_monkey = &monkeys[get_monkey_followup(lines[4].to_string())];
        let false_monkey = &monkeys[get_monkey_followup(lines[5].to_string())];

        //monkeys[i].set_true_monkey(true_monkey);
        //monkeys[i].set_false_monkey(false_monkey);
    }

    monkeys
}

fn create_monkey(lines: Vec<&str>) -> Monkey {
    let id = get_monkey_id(lines[0].to_string());
    let items = get_monkey_items(lines[1].to_string());
    let operation = get_monkey_operation(lines[2].to_string());
    let divider = get_monkey_divider(lines[3].to_string());

    Monkey::new(id, items, divider, operation)
}

fn get_monkey_id(id_line: String) -> u32 {
    let words: Vec<&str> = id_line.split(" ").collect();
    let id_string = &words[1];
    let id_with_suffix = id_string.strip_suffix(":");
    let id = id_with_suffix.unwrap().parse::<u32>().unwrap();

    id
}

fn get_monkey_items(item_line: String) -> Vec<u32> {
    let words: Vec<&str> = item_line.split(" ").collect();
    let item_strings = &words[2..words.len()];
    let mut items: Vec<u32> = Vec::new();

    for item in item_strings {
        items.push(item.parse::<u32>().unwrap());
    }

    items
}

fn get_monkey_operation(operation_line: String) -> Operator {
    let words: Vec<&str> = operation_line.split(" ").collect();
    let operation_strings = &words[3..words.len()];
    
    if operation_strings[0] == operation_strings[2] {
        return Operator::Exponent(2);
    }

    let operator = operation_strings[1];
    let number = operation_strings[2];

    match operator {
        "+" => Operator::Addition(number.parse::<u32>().unwrap()),
        "-" => Operator::Subtraction(number.parse::<u32>().unwrap()),
        "*" => Operator::Multiplication(number.parse::<u32>().unwrap()),
        "/" => Operator::Division(number.parse::<u32>().unwrap()),
        _ => Operator::Addition(0)
    }
}

fn get_monkey_divider(divider_line: String) -> u32 {
    let words: Vec<&str> = divider_line.split(" ").collect();
    let divider_string = words[words.len() - 1];

    divider_string.parse::<u32>().unwrap()
}

fn get_monkey_followup(followup_line: String) -> usize {
    let words: Vec<&str> = followup_line.split(" ").collect();

    words[words.len()].parse::<usize>().unwrap()
}
