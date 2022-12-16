use std::rc::Rc;

pub enum Operator {
    Addition(u32),
    Subtraction(u32),
    Multiplication(u32),
    Division(u32),
    Exponent(u32)
}

pub struct Monkey {
    id: u32,
    items: Vec<u32>,
    divider: u32,
    operator: Operator,
    true_monkey: Option<Rc<Monkey>>,
    false_monkey: Option<Rc<Monkey>>
}

impl Monkey {
    pub fn new(id: u32, items: Vec<u32>, divider: u32, operator: Operator) -> Self {
       Self {
            id,
            items,
            divider,
            operator,
            true_monkey: Option::None,
            false_monkey: Option::None
        } 
    }

    pub fn set_true_monkey(&mut self, true_monkey: Rc<Monkey>) {
        self.true_monkey = Some(true_monkey);
    }

    pub fn set_false_monkey(&mut self, false_monkey: Rc<Monkey>) {
        self.false_monkey = Some(false_monkey);
    }

    pub fn execute_turn(&mut self) {
        for _ in 0..self.items.len() {
            self.execute_single_turn();
        }
    }

    fn execute_single_turn(&mut self) {
        if self.items.len() == 0 {
            return;
        }

        let worry_level = self.inspect_item();

        if worry_level % self.divider == 0 {
            let next_monkey = Rc::get_mut(&mut self.true_monkey.unwrap()).unwrap();
            next_monkey.items.push(worry_level);
        }
        else {
            let next_monkey = Rc::get_mut(&mut self.false_monkey.unwrap()).unwrap();
            next_monkey.items.push(worry_level);
        }

        let _ = &mut self.items.remove(0);
    }

    fn inspect_item(&self) -> u32 {
        let first_item = &self.items[0];
        let mut worry_level = self.extract_operator(*first_item);
        worry_level = worry_level / 3;
        
        worry_level
    }

    fn extract_operator(&self, value: u32) -> u32 {
        match &self.operator {
            Operator::Addition(x) => value + x,
            Operator::Subtraction(x) => value - x,
            Operator::Multiplication(x) => value * x,
            Operator::Division(x) => value / x,
            Operator::Exponent(x) => u32::pow(value, *x),
        }
    }
}

