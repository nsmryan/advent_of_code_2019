use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

const DEBUG: bool = false;

const INPUT: &str =
"3,8,1005,8,321,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1002,8,1,29,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1002,8,1,50,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,73,1,1105,16,10,2,1004,8,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1002,8,1,103,1006,0,18,1,105,14,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,131,1006,0,85,1,1008,0,10,1006,0,55,2,104,4,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,168,2,1101,1,10,1006,0,14,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,102,1,8,196,1006,0,87,1006,0,9,1,102,20,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,1001,8,0,228,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1002,8,1,250,2,5,0,10,2,1009,9,10,2,107,17,10,1006,0,42,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,1001,8,0,287,2,102,8,10,1006,0,73,1006,0,88,1006,0,21,101,1,9,9,1007,9,925,10,1005,10,15,99,109,643,104,0,104,1,21102,1,387353256856,1,21101,0,338,0,1105,1,442,21101,936332866452,0,1,21101,349,0,0,1105,1,442,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,179357024347,1,21101,0,396,0,1105,1,442,21102,1,29166144659,1,21102,407,1,0,1105,1,442,3,10,104,0,104,0,3,10,104,0,104,0,21102,1,718170641252,1,21102,430,1,0,1106,0,442,21101,825012151040,0,1,21102,441,1,0,1106,0,442,99,109,2,21202,-1,1,1,21102,1,40,2,21102,1,473,3,21102,463,1,0,1105,1,506,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,468,469,484,4,0,1001,468,1,468,108,4,468,10,1006,10,500,1102,1,0,468,109,-2,2105,1,0,0,109,4,1202,-1,1,505,1207,-3,0,10,1006,10,523,21101,0,0,-3,22101,0,-3,1,21202,-2,1,2,21102,1,1,3,21102,1,542,0,1105,1,547,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,570,2207,-4,-2,10,1006,10,570,22102,1,-4,-4,1105,1,638,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21101,0,589,0,1106,0,547,22102,1,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,608,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,630,21202,-1,1,1,21102,630,1,0,105,1,505,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0";

type Value = i128;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
    Step,
    Blocked,
    Exit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Mode {
    Imm,
    Pos,
    Rel,
}

impl Mode {
    pub fn from_digit(digit: u8) -> Mode {
        match digit {
            0 => Mode::Pos,
            1 => Mode::Imm,
            2 => Mode::Rel,
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
struct Loc {
    loc: Value,
    mode: Mode,
}

impl Loc {
    pub fn new(loc: Value, mode: Mode) -> Loc {
        return Loc {
            loc,
            mode,
        };
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instr {
    Add(Loc, Loc, Loc),
    Mult(Loc, Loc, Loc),
    Input(Loc),
    Output(Loc),
    JmpIfTrue(Loc, Loc),
    JmpIfFalse(Loc, Loc),
    LT(Loc, Loc, Loc),
    EQ(Loc, Loc, Loc),
    AdjustRelBase(Loc),
    Exit
}

impl Instr {
    pub fn name(&self) -> &str {
        use Instr::*;
        match self {
            Add(_, _, _) => "Add",
            Mult(_, _, _) => "Mult",
            Input(_) => "Input",
            Output(_) => "Output",
            JmpIfTrue(_, _) => "JmpIfTrue",
            JmpIfFalse(_, _) => "JmpIfFalse",
            LT(_, _, _) => "LT",
            EQ(_, _, _) => "EQ",
            AdjustRelBase(_) => "AdjustRelBase",
            Exit => "Exit",
        }
    }
}

#[derive(Clone)]
struct IntCodeState {
    pub memory: HashMap<usize, Value>,
    pub input: VecDeque<Value>,
    pub output: VecDeque<Value>,
    pub cursor: usize,
    pub relative_base: Value,
}

impl IntCodeState {
    pub fn from_string(prog_string: &str) -> IntCodeState {
        let memory_vec = 
            prog_string.split(",")
                       .map(|code| code.parse::<Value>().unwrap())
                       .collect::<Vec<Value>>();
        let mut memory = HashMap::new();
        for (ix, value) in memory_vec.iter().enumerate() {
            memory.insert(ix, *value);
        }

        let result = IntCodeState {
            memory,
            cursor: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            relative_base: 0,
        };
        
        return result;
    }

    pub fn parse_instr(&mut self) -> Instr {
        let mut digits = to_digits(self.pop());

        let mut opcode_value = digits.pop_front().unwrap();
        if digits.len() > 0 {
            if let Some(next_digit) = digits.pop_front() {
                opcode_value += 10 * next_digit;
            }
        }

        match opcode_value {
            1 => {
                let src1 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let dst  = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::Add(src1, src2, dst)
            }

            2 => {
                let src1 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let dst  = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::Mult(src1, src2, dst)
            }

            3 => {
                let dst  = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::Input(dst)
            }

            4 => {
                let src = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::Output(src)
            }

            5 => {
                let src1 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::JmpIfTrue(src1, src2)
            }

            6 => {
                let src1 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::JmpIfFalse(src1, src2)
            }

            7 => {
                let src1 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let dst  = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::LT(src1, src2, dst)
            }

            8 => {
                let src1 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let src2 = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                let dst  = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::EQ(src1, src2, dst)
            }

            9 => {
                let src = Loc::new(self.pop(), Mode::pull_mode(&mut digits));
                Instr::AdjustRelBase(src)
            }

            99 =>{
                Instr::Exit
            }

            _ => {
                panic!(format!("Unexpected input {}", opcode_value))
            }
        }
    }

    pub fn write_memory(&mut self, location: Value, value: Value) {
        self.memory.insert(location as usize, value);
    }

    pub fn read_memory(&self, location: Value) -> Value {
        if self.memory.contains_key(&(location as usize)) {
            return *self.memory.get(&(location as usize)).unwrap();
        } else {
            return 0;
        }
    }

    pub fn pop(&mut self) -> Value {
        assert!(self.cursor < self.memory.len());

        let result = self.read_memory(self.cursor as Value);
        self.cursor += 1;

        return result;
    }

    // this is the opposite function from fetch
    pub fn provide(&mut self, dst: Loc, value: Value) {
        match dst.mode {
            Mode::Imm => {
                panic!("Attempted to write to immediate position!");
            }

            Mode::Pos => {
                return self.write_memory(dst.loc, value);
            }

            Mode::Rel => {
                return self.write_memory(self.relative_base + dst.loc, value);
            }
        }
    }

    // this is the opposite function from provide
    pub fn fetch(&self, src: Loc) -> Value {
        match src.mode {
            Mode::Imm => {
                return src.loc;
            }

            Mode::Pos => {
                return self.read_memory(src.loc);
            }

            Mode::Rel => {
                return self.read_memory(self.relative_base + src.loc);
            }
        }
    }

    pub fn step(&mut self) -> Status {
        let instr = self.parse_instr();

        let status = self.execute_instr(instr);

        return status;
    }

    pub fn execute_instr(&mut self, instr: Instr) -> Status {
        //println!("{}", instr.name());
        match instr {
            Instr::Add(src1, src2, dst) => {
                let val1 = self.fetch(src1);
                let val2 = self.fetch(src2);
                let result = val1 + val2;
                if DEBUG { println!("{} + {} = {}", val1, val2, result)}
                self.provide(dst, result);
                return Status::Step;
            }

            Instr::Mult(src1, src2, dst) => {
                let val1 = self.fetch(src1);
                let val2 = self.fetch(src2);
                let result = val1 * val2;
                if DEBUG { println!("{} * {} = {}", val1, val2, result)}
                self.provide(dst, result);
                return Status::Step;
            }

            Instr::Input(dst) => {
                if self.input.len() > 0 {
                    let input = self.read_input();
                    if DEBUG { println!("<- {}", input) }
                    self.provide(dst, input);
                    return Status::Step;
                } else {
                    // back up to input instr
                    self.cursor -= 2;
                    return Status::Blocked;
                }
            }

            Instr::Output(src) => {
                let value = self.fetch(src);
                self.write_output(value);
                if DEBUG { println!("{} ->", value) }
                return Status::Step;
            }

            Instr::JmpIfTrue(src1, src2) => {
                let cond_value = self.fetch(src1);
                if cond_value != 0 {
                    let new_ip = self.fetch(src2);
                    if DEBUG { println!("JMP {}", new_ip) }
                    self.cursor = new_ip as usize;
                }

                return Status::Step;
            }

            Instr::JmpIfFalse(src1, src2) => {
                let cond_value = self.fetch(src1);
                if cond_value == 0 {
                    let new_ip = self.fetch(src2);
                    if DEBUG { println!("JMP {}", new_ip) }
                    self.cursor = new_ip as usize;
                }

                return Status::Step;
            }

            Instr::LT(src1, src2, dst) => {
                let first = self.fetch(src1);
                let second = self.fetch(src2);
                let result;
                if first < second {
                    result = 1;
                } else {
                    result = 0;
                }
                self.provide(dst, result);
                if DEBUG { println!("{} < {} -> {}", first, second, result) }

                return Status::Step;
            }

            Instr::EQ(src1, src2, dst) => {
                let first = self.fetch(src1);
                let second = self.fetch(src2);
                let result;
                if first == second {
                    result = 1;
                } else {
                    result = 0;
                }
                self.provide(dst, result);
                if DEBUG { println!("{} < {} -> {}", first, second, result) }

                return Status::Step;
            }

            Instr::AdjustRelBase(src) => {
                let value = self.fetch(src);

                self.relative_base += value;

                return Status::Step;
            }

            Instr::Exit => {
                if DEBUG { println!("Exit") }
                return Status::Exit;
            }
        }
    }

    pub fn run(&mut self) -> Status {
        let mut status = Status::Step;

        while status != Status::Blocked &&
              status != Status::Exit {
            assert!(self.cursor < self.memory.len());
            status = self.step();
        }

        return status;
    }

    pub fn write_output(&mut self, value: Value) {
        self.output.push_back(value);
    }

    pub fn read_input(&mut self) -> Value {
        return self.input.pop_front().unwrap();
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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Color {
    Black,
    White,
}

type Map = Vec<Vec<Color>>;


fn main() {
    let mut int_code = IntCodeState::from_string(INPUT);

    let mut map = vec![vec![0; 100]; 100];
    map[50][50] = 1;

    let mut status = Status::Step;
    let mut tiles = HashSet::new();
    let mut loc = (50, 50);
    let mut facing = 0;

    while status != Status::Exit {
      int_code.input.push_front(map[loc.1][loc.0]);
      status = int_code.run();
      if status == Status::Blocked {
          let new_color = int_code.output.pop_front().unwrap();
          map[loc.1][loc.0] = new_color;
          tiles.insert(loc);

          let angle = int_code.output.pop_front().unwrap();
          if angle == 0 {
              if facing == 0 {
                  facing = 3;
              } else {
                  facing -= 1;
              }
          } else {
              facing = (facing + 1) % 4;
          }

          if facing == 0 {
              loc.1 -= 1;
          } else if facing == 1 {
              loc.0 += 1;
          } else if facing == 2 {
              loc.1 += 1;
          } else if facing == 3 {
              loc.0 -= 1;
          }
      }
    }

    println!("Result = {}", tiles.len());

    for y in 0..100 {
        for x in 0..100 {
            if map[y][x] == 0 {
                print!(" ");
            } else {
                print!("+");
            }
        }
        println!("");
    }
}
