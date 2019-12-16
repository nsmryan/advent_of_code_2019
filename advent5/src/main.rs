use std::collections::VecDeque;


const INPUT: &str = 
"3,225,1,225,6,6,1100,1,238,225,104,0,1102,35,92,225,1101,25,55,225,1102,47,36,225,1102,17,35,225,1,165,18,224,1001,224,-106,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1101,68,23,224,101,-91,224,224,4,224,102,8,223,223,101,1,224,224,1,223,224,223,2,217,13,224,1001,224,-1890,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1102,69,77,224,1001,224,-5313,224,4,224,1002,223,8,223,101,2,224,224,1,224,223,223,102,50,22,224,101,-1800,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,1102,89,32,225,1001,26,60,224,1001,224,-95,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,51,79,225,1102,65,30,225,1002,170,86,224,101,-2580,224,224,4,224,102,8,223,223,1001,224,6,224,1,223,224,223,101,39,139,224,1001,224,-128,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1102,54,93,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,677,224,1002,223,2,223,1005,224,329,101,1,223,223,7,677,677,224,102,2,223,223,1006,224,344,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,359,1001,223,1,223,7,677,226,224,1002,223,2,223,1005,224,374,1001,223,1,223,1107,677,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,107,226,677,224,102,2,223,223,1005,224,404,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,419,101,1,223,223,107,226,226,224,102,2,223,223,1005,224,434,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,449,101,1,223,223,108,226,226,224,102,2,223,223,1006,224,464,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,479,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,494,101,1,223,223,1007,226,677,224,102,2,223,223,1006,224,509,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,524,101,1,223,223,107,677,677,224,102,2,223,223,1005,224,539,101,1,223,223,1008,677,226,224,1002,223,2,223,1005,224,554,1001,223,1,223,1008,226,226,224,1002,223,2,223,1006,224,569,1001,223,1,223,1108,226,226,224,102,2,223,223,1005,224,584,101,1,223,223,1107,226,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,1108,677,226,224,102,2,223,223,1005,224,629,1001,223,1,223,8,226,226,224,1002,223,2,223,1005,224,644,1001,223,1,223,1107,677,677,224,1002,223,2,223,1005,224,659,1001,223,1,223,1007,677,677,224,1002,223,2,223,1005,224,674,101,1,223,223,4,223,99,226";


type Value = i32;

type Dst = Value;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Mode {
    Imm,
    Pos,
}

impl Mode {
    pub fn from_digit(digit: u8) -> Mode {
        match digit {
            0 => Mode::Pos,
            1 => Mode::Imm,
            _ => panic!(format!("Unexpected mode value {}", digit)),
        }
    }

    pub fn pull_mode(digits: &mut VecDeque<u8>) -> Mode {
        if let Some(mode_int) = digits.pop_front() {
            return Mode::from_digit(mode_int);
        } else {
            return Mode::Pos;
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Src {
    src: Value,
    mode: Mode,
}

impl Src {
    pub fn new(src: Value, mode: Mode) -> Src {
        return Src {
            src,
            mode,
        };
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Instr {
    Add(Src, Src, Dst),
    Mult(Src, Src, Dst),
    Input(Dst),
    Output(Src),
    JmpIfTrue(Src, Src),
    JmpIfFalse(Src, Src),
    LT(Src, Src, Dst),
    EQ(Src, Src, Dst),
    Exit
}

#[derive(Clone)]
struct IntCodeState {
    pub memory: Vec<Value>,
    pub input: VecDeque<Value>,
    pub output: VecDeque<Value>,
    pub cursor: usize,
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
            input: VecDeque::new(),
            output: VecDeque::new(),
        };
        
        return result;
    }

    pub fn parse_instr(&mut self) -> Instr {
        let mut digits = to_digits(self.pop());

        dbg!(self.cursor);
        dbg!(&digits);

        let mut opcode_value = digits.pop_front().unwrap();
        if digits.len() > 0 {
            if let Some(next_digit) = digits.pop_front() {
                opcode_value += 10 * next_digit;
            }
        }

        match opcode_value {
            1 => {
                let src1 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let dst = self.pop();
                Instr::Add(src1, src2, dst)
            }

            2 => {
                let src1 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let dst = self.pop();
                Instr::Mult(src1, src2, dst)
            }

            3 => {
                Instr::Input(self.pop())
            }

            4 => {
                let src = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::Output(src)
            }

            5 => {
                let src1 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::JmpIfTrue(src1, src2)
            }

            6 => {
                let src1 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::JmpIfFalse(src1, src2)
            }

            7 => {
                let src1 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let dst = self.pop();
                Instr::LT(src1, src2, dst)
            }

            8 => {
                let src1 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Src::new(self.pop(), Mode::pull_mode(&mut digits));
                let dst = self.pop();
                Instr::EQ(src1, src2, dst)
            }

            99 =>{
                Instr::Exit
            }

            _ => {
                panic!(format!("Unexpected input {}", opcode_value))
            }
        }
    }

    pub fn pop(&mut self) -> Value {
        assert!(self.cursor < self.memory.len());

        let result = self.memory[self.cursor];
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

    pub fn fetch(&self, src: Src) -> Value {
        match src.mode {
            Mode::Imm => {
                return src.src;
            }

            Mode::Pos => {
                return self.memory[src.src as usize];
            }
        }
    }

    pub fn add(&mut self, src1: Src, src2: Src, dst: Dst) {
        self.memory[dst as usize] = self.fetch(src1) + self.fetch(src2);
    }

    pub fn mult(&mut self, src1: Src, src2: Src, dst: Dst) {
        self.memory[dst as usize] = self.fetch(src1) * self.fetch(src2);
    }

    pub fn run(&mut self) {
        while self.cursor < self.memory.len() {
            match self.parse_instr() {
                Instr::Add(src1, src2, dst) => {
                    self.add(src1, src2, dst);
                }

                Instr::Mult(src1, src2, dst) => {
                    self.mult(src1, src2, dst);
                }

                Instr::Input(dst) => {
                    let input = self.read_input();
                    self.memory[dst as usize] = input;
                }

                Instr::Output(src) => {
                    let value = self.fetch(src);
                    self.write_output(value);
                }

                Instr::JmpIfTrue(src1, src2) => {
                    let cond_value = self.fetch(src1);
                    if cond_value != 0 {
                        let new_ip = self.fetch(src2);
                        self.cursor = new_ip as usize;
                    }
                }

                Instr::JmpIfFalse(src1, src2) => {
                    let cond_value = self.fetch(src1);
                    if cond_value == 0 {
                        let new_ip = self.fetch(src2);
                        self.cursor = new_ip as usize;
                    }
                }

                Instr::LT(src1, src2, dst) => {
                    let first = self.fetch(src1);
                    let second = self.fetch(src2);
                    if first < second {
                        self.memory[dst as usize] = 1;
                    } else {
                        self.memory[dst as usize] = 0;
                    }
                }

                Instr::EQ(src1, src2, dst) => {
                    let first = self.fetch(src1);
                    let second = self.fetch(src2);
                    if first == second {
                        self.memory[dst as usize] = 1;
                    } else {
                        self.memory[dst as usize] = 0;
                    }
                }

                Instr::Exit => {
                    break;
                }
            }
        }
    }

    pub fn write_output(&mut self, value: Value) {
        self.output.push_back(value);
    }

    pub fn read_input(&mut self) -> Value {
        return self.input.pop_front().unwrap();
    }

    pub fn result(&self) -> Value {
        return self.memory[0];
    }
}

fn to_digits(value: Value) -> VecDeque<u8> {
    let mut current = value;
    let mut digits = VecDeque::new();

    if value == 0 {
        digits.push_back(0);
    } else {
        while current > 0 {
            digits.push_back((current % 10) as u8);
            current /= 10;
        }
    }

    return digits;
}

fn main() {
  let mut int_code = IntCodeState::from_string(INPUT);

  int_code.input.push_front(5);

  int_code.run();

  let last_output = int_code.output.pop_back().unwrap();

  println!("Result = {}", last_output);
}
