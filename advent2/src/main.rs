
const ADD_CODE: Value = 1;
const MULT_CODE: Value = 2;
const EXIT_CODE: Value = 99;
const TARGET_VALUE: Value = 19690720;

const PROGRAM_STRING: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,2,9,19,23,2,13,23,27,1,6,27,31,2,6,31,35,2,13,35,39,1,39,10,43,2,43,13,47,1,9,47,51,1,51,13,55,1,55,13,59,2,59,13,63,1,63,6,67,2,6,67,71,1,5,71,75,2,6,75,79,1,5,79,83,2,83,6,87,1,5,87,91,1,6,91,95,2,95,6,99,1,5,99,103,1,6,103,107,1,107,2,111,1,111,5,0,99,2,14,0,0";


type Value = u32;

#[derive(Clone)]
struct IntCodeState {
    pub memory: Vec<Value>,
    pub cursor: usize,
}

enum Opcode {
    Add,
    Mult,
    Exit
}

impl Opcode {
}

impl IntCodeState {
    pub fn from_string(prog_string: &str) -> IntCodeState {
        let memory = 
            prog_string.split(",")
                       .map(|code| code.parse::<Value>().unwrap())
                       .collect::<Vec<Value>>();

        let result = IntCodeState {
            memory,
            cursor: 0,
        };
        
        return result;
    }

    pub fn is_exit(&self) -> bool {
    }

    pub fn current_value(&self) -> Value {
        return self.memory[self.cursor];
    }

    pub fn pop(&mut self) -> Value {
        assert!(self.cursor < self.memory.len());

        //dbg!(self.cursor);
        let result = self.memory[self.cursor];
        //dbg!(result);
        self.cursor += 1;

        return result;
    }

    pub fn ensure(&mut self, ix: Value) {
        if self.memory.len() < ix as usize {
            //dbg!("Ensuring size");
            for _ in 0..(ix as usize - self.memory.len() + 1) {
                self.memory.push(0);
            }
        }
    }

    pub fn add(&mut self, src1: Value, src2: Value, dst: Value) {
        self.ensure(src1);
        self.ensure(src2);
        self.ensure(dst);

        //dbg!("Add", self.memory[src1 as usize], self.memory[src2 as usize], self.memory[dst as usize]);
        self.memory[dst as usize] = self.memory[src1 as usize] + self.memory[src2 as usize];
    }

    pub fn mult(&mut self, src1: Value, src2: Value, dst: Value) {
        self.ensure(src1);
        self.ensure(src2);
        self.ensure(dst);

        //dbg!("Mult", self.memory[src1 as usize], self.memory[src2 as usize], self.memory[dst as usize]);
        self.memory[dst as usize] = self.memory[src1 as usize] * self.memory[src2 as usize];
    }

    pub fn run(&mut self) {
        while self.current_value() != EXIT_CODE {
            let value = self.pop();
            //dbg!(value);

            if value == ADD_CODE {
                //dbg!();
                let src1 = self.pop();
                let src2 = self.pop();
                let dst = self.pop();

                self.add(src1, src2, dst);
            } else if value == MULT_CODE {
                //dbg!();
                let src1 = self.pop();
                let src2 = self.pop();
                let dst = self.pop();

                self.mult(src1, src2, dst);
            } else {
                panic!(format!("Unexpected code {} at cursor = {}", value, self.cursor));
            }
        }
    }

    pub fn result(&self) -> Value {
        return self.memory[0];
    }
}

#[test]
pub fn test_add() {
    let mut int_program = IntCodeState::from_string("1,0,1,2,99");

    int_program.memory[0] = 1;
    int_program.memory[1] = 2;
    int_program.run();

    assert_eq!(int_program.memory[2], 3);
}

#[test]
pub fn test_mult() {
    let mut int_program = IntCodeState::from_string("2,0,1,2,99");

    int_program.memory[0] = 1;
    int_program.memory[1] = 2;
    int_program.run();

    assert_eq!(int_program.memory[2], 2);
}

#[test]
pub fn test_both() {
    let mut int_program = IntCodeState::from_string("2,0,1,2,1,2,0,5,99");

    int_program.memory[0] = 1;
    int_program.memory[1] = 2;
    int_program.run();

    assert_eq!(int_program.memory[5], 3);
}

fn part_one() {
    let mut int_program = IntCodeState::from_string(PROGRAM_STRING);

    int_program.memory[1] = 12;
    int_program.memory[2] = 2;

    int_program.run();

    println!("Result = {}", int_program.result());
}

fn part_two() {
    let int_program = IntCodeState::from_string(PROGRAM_STRING);

    let mut noun = 0;
    let mut verb = 0;

    loop {
        let mut program_attempt = int_program.clone();

        program_attempt.memory[1] = noun;
        program_attempt.memory[2] = verb;

        program_attempt.run();

        if program_attempt.memory[0] == TARGET_VALUE {
            println!("Result = {}", (100 * noun) + verb);
            break;
        }

        if noun == verb {
            noun += 1;
            verb = 0;
        } else {
            verb += 1;
        }
    }
}

fn main() {
    part_two();
}
