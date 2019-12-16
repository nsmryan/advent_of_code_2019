use std::collections::VecDeque;

const DEBUG: bool = false;

const INPUT: &str =
"3,8,1001,8,10,8,105,1,0,0,21,46,59,84,93,102,183,264,345,426,99999,3,9,1002,9,4,9,1001,9,3,9,102,2,9,9,1001,9,5,9,102,3,9,9,4,9,99,3,9,1002,9,3,9,101,4,9,9,4,9,99,3,9,1002,9,4,9,1001,9,4,9,102,2,9,9,1001,9,2,9,1002,9,3,9,4,9,99,3,9,1001,9,5,9,4,9,99,3,9,1002,9,4,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,99";
//"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

const PERMUTATIONS: [[Value; 5]; 120] = 
[[0,1,2,3,4] ,[1,0,2,3,4] ,[0,2,1,3,4] ,[2,0,1,3,4] ,[1,2,0,3,4] ,[2,1,0,3,4] ,[0,1,3,2,4] ,[1,0,3,2,4] ,[0,3,1,2,4] ,[3,0,1,2,4] ,[1,3,0,2,4] ,[3,1,0,2,4] ,[0,2,3,1,4] ,[2,0,3,1,4] ,[0,3,2,1,4] ,[3,0,2,1,4] ,[2,3,0,1,4] ,[3,2,0,1,4] ,[1,2,3,0,4] ,[2,1,3,0,4] ,[1,3,2,0,4] ,[3,1,2,0,4] ,[2,3,1,0,4] ,[3,2,1,0,4] ,[0,1,2,4,3] ,[1,0,2,4,3] ,[0,2,1,4,3] ,[2,0,1,4,3] ,[1,2,0,4,3] ,[2,1,0,4,3] ,[0,1,4,2,3] ,[1,0,4,2,3] ,[0,4,1,2,3] ,[4,0,1,2,3] ,[1,4,0,2,3] ,[4,1,0,2,3] ,[0,2,4,1,3] ,[2,0,4,1,3] ,[0,4,2,1,3] ,[4,0,2,1,3] ,[2,4,0,1,3] ,[4,2,0,1,3] ,[1,2,4,0,3] ,[2,1,4,0,3] ,[1,4,2,0,3] ,[4,1,2,0,3] ,[2,4,1,0,3] ,[4,2,1,0,3] ,[0,1,3,4,2] ,[1,0,3,4,2] ,[0,3,1,4,2] ,[3,0,1,4,2] ,[1,3,0,4,2] ,[3,1,0,4,2] ,[0,1,4,3,2] ,[1,0,4,3,2] ,[0,4,1,3,2] ,[4,0,1,3,2] ,[1,4,0,3,2] ,[4,1,0,3,2] ,[0,3,4,1,2] ,[3,0,4,1,2] ,[0,4,3,1,2] ,[4,0,3,1,2] ,[3,4,0,1,2] ,[4,3,0,1,2] ,[1,3,4,0,2] ,[3,1,4,0,2] ,[1,4,3,0,2] ,[4,1,3,0,2] ,[3,4,1,0,2] ,[4,3,1,0,2] ,[0,2,3,4,1] ,[2,0,3,4,1] ,[0,3,2,4,1] ,[3,0,2,4,1] ,[2,3,0,4,1] ,[3,2,0,4,1] ,[0,2,4,3,1] ,[2,0,4,3,1] ,[0,4,2,3,1] ,[4,0,2,3,1] ,[2,4,0,3,1] ,[4,2,0,3,1] ,[0,3,4,2,1] ,[3,0,4,2,1] ,[0,4,3,2,1] ,[4,0,3,2,1] ,[3,4,0,2,1] ,[4,3,0,2,1] ,[2,3,4,0,1] ,[3,2,4,0,1] ,[2,4,3,0,1] ,[4,2,3,0,1] ,[3,4,2,0,1] ,[4,3,2,0,1] ,[1,2,3,4,0] ,[2,1,3,4,0] ,[1,3,2,4,0] ,[3,1,2,4,0] ,[2,3,1,4,0] ,[3,2,1,4,0] ,[1,2,4,3,0] ,[2,1,4,3,0] ,[1,4,2,3,0] ,[4,1,2,3,0] ,[2,4,1,3,0] ,[4,2,1,3,0] ,[1,3,4,2,0] ,[3,1,4,2,0] ,[1,4,3,2,0] ,[4,1,3,2,0] ,[3,4,1,2,0] ,[4,3,1,2,0] ,[2,3,4,1,0] ,[3,2,4,1,0] ,[2,4,3,1,0] ,[4,2,3,1,0] ,[3,4,2,1,0] ,[4,3,2,1,0]];

type Value = i128;

type Dst = Value;

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Exit => "Exit",
        }
    }
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
                self.memory[dst as usize] = result;
                return Status::Step;
            }

            Instr::Mult(src1, src2, dst) => {
                let val1 = self.fetch(src1);
                let val2 = self.fetch(src2);
                let result = val1 * val2;
                if DEBUG { println!("{} * {} = {}", val1, val2, result)}
                self.memory[dst as usize] = result;
                return Status::Step;
            }

            Instr::Input(dst) => {
                if self.input.len() > 0 {
                    let input = self.read_input();
                    if DEBUG { println!("<- {}", input) }
                    self.memory[dst as usize] = input;
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
                self.memory[dst as usize] = result;
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
                self.memory[dst as usize] = result;
                if DEBUG { println!("{} < {} -> {}", first, second, result) }

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

pub fn phase_list() -> Vec<Vec<Value>> {
    let mut lists = vec!();

    for list in PERMUTATIONS.iter() {
        let mut perm  = vec!();
        for item in list {
            perm.push(item + 5);
        }
        lists.push(perm.to_vec());
    }

    return lists;
}

fn main() {
  let int_code = IntCodeState::from_string(INPUT);

  let mut max_thrust = -10000000;

  for phases in phase_list() {
      let mut amp_circuit = vec![int_code.clone(); 5];

      println!("Starting new phase set");
      for (amp, phase) in amp_circuit.iter_mut().zip(phases.iter()) {
          amp.input.push_front(*phase);
      }

      let mut exited_nodes = 0;
      let mut input = 0;
      while exited_nodes != 5 {
          let mut ix = 0;
          for amp in amp_circuit.iter_mut() {
              amp.input.push_back(input);
              let status = amp.run();

              if status == Status::Blocked ||
                 status == Status::Exit {
                  input = amp.output.pop_back().unwrap();
                  //println!("{} ({:?}) -> {}", ix, status, input)
              }

              if status == Status::Exit {
                  exited_nodes += 1;
              }

              ix += 1;

          }
      }

      max_thrust = std::cmp::max(max_thrust, input);

      println!("______");
  }

  println!("Result = {}", max_thrust);
}

