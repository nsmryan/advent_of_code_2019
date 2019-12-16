use std::collections::VecDeque;
use std::collections::HashMap;


const DEBUG: bool = false;

const INPUT: &str =
// puzzle input
"1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,1,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,0,1020,1102,1,800,1023,1101,0,388,1025,1101,0,31,1012,1102,1,1,1021,1101,22,0,1014,1101,0,30,1002,1101,0,716,1027,1102,32,1,1009,1101,0,38,1017,1102,20,1,1015,1101,33,0,1016,1101,0,35,1007,1101,0,25,1005,1102,28,1,1011,1102,1,36,1008,1101,0,39,1001,1102,1,21,1006,1101,397,0,1024,1102,1,807,1022,1101,0,348,1029,1101,0,23,1003,1101,29,0,1004,1102,1,26,1013,1102,34,1,1018,1102,1,37,1010,1101,0,27,1019,1102,24,1,1000,1101,353,0,1028,1101,0,723,1026,109,14,2101,0,-9,63,1008,63,27,63,1005,63,205,1001,64,1,64,1106,0,207,4,187,1002,64,2,64,109,-17,2108,24,6,63,1005,63,223,1105,1,229,4,213,1001,64,1,64,1002,64,2,64,109,7,2101,0,2,63,1008,63,21,63,1005,63,255,4,235,1001,64,1,64,1106,0,255,1002,64,2,64,109,-7,2108,29,7,63,1005,63,273,4,261,1106,0,277,1001,64,1,64,1002,64,2,64,109,10,1208,-5,31,63,1005,63,293,1105,1,299,4,283,1001,64,1,64,1002,64,2,64,109,2,1207,-1,35,63,1005,63,315,1106,0,321,4,305,1001,64,1,64,1002,64,2,64,109,8,1205,3,333,1106,0,339,4,327,1001,64,1,64,1002,64,2,64,109,11,2106,0,0,4,345,1106,0,357,1001,64,1,64,1002,64,2,64,109,-15,21108,40,40,6,1005,1019,379,4,363,1001,64,1,64,1106,0,379,1002,64,2,64,109,16,2105,1,-5,4,385,1001,64,1,64,1105,1,397,1002,64,2,64,109,-25,2102,1,-1,63,1008,63,26,63,1005,63,421,1001,64,1,64,1106,0,423,4,403,1002,64,2,64,109,-8,1202,9,1,63,1008,63,25,63,1005,63,445,4,429,1105,1,449,1001,64,1,64,1002,64,2,64,109,5,1207,0,40,63,1005,63,467,4,455,1106,0,471,1001,64,1,64,1002,64,2,64,109,-6,2107,24,8,63,1005,63,487,1105,1,493,4,477,1001,64,1,64,1002,64,2,64,109,15,21107,41,40,1,1005,1011,509,1106,0,515,4,499,1001,64,1,64,1002,64,2,64,109,12,1205,-1,529,4,521,1105,1,533,1001,64,1,64,1002,64,2,64,109,-20,2102,1,2,63,1008,63,29,63,1005,63,555,4,539,1105,1,559,1001,64,1,64,1002,64,2,64,109,15,1201,-9,0,63,1008,63,38,63,1005,63,579,1105,1,585,4,565,1001,64,1,64,1002,64,2,64,109,-2,21102,42,1,-3,1008,1012,44,63,1005,63,609,1001,64,1,64,1106,0,611,4,591,1002,64,2,64,109,-21,2107,29,8,63,1005,63,629,4,617,1106,0,633,1001,64,1,64,1002,64,2,64,109,15,1202,0,1,63,1008,63,30,63,1005,63,657,1001,64,1,64,1106,0,659,4,639,1002,64,2,64,109,15,21102,43,1,-8,1008,1016,43,63,1005,63,681,4,665,1105,1,685,1001,64,1,64,1002,64,2,64,109,-10,21107,44,45,-4,1005,1010,707,4,691,1001,64,1,64,1106,0,707,1002,64,2,64,109,11,2106,0,2,1001,64,1,64,1106,0,725,4,713,1002,64,2,64,109,-16,21101,45,0,8,1008,1017,43,63,1005,63,749,1001,64,1,64,1105,1,751,4,731,1002,64,2,64,109,-3,1208,2,36,63,1005,63,773,4,757,1001,64,1,64,1106,0,773,1002,64,2,64,109,18,1206,-4,787,4,779,1105,1,791,1001,64,1,64,1002,64,2,64,109,-8,2105,1,7,1001,64,1,64,1106,0,809,4,797,1002,64,2,64,109,-2,21108,46,44,2,1005,1016,825,1105,1,831,4,815,1001,64,1,64,1002,64,2,64,109,7,21101,47,0,-8,1008,1013,47,63,1005,63,857,4,837,1001,64,1,64,1105,1,857,1002,64,2,64,109,-17,1201,-4,0,63,1008,63,24,63,1005,63,883,4,863,1001,64,1,64,1105,1,883,1002,64,2,64,109,10,1206,7,895,1106,0,901,4,889,1001,64,1,64,4,64,99,21102,1,27,1,21102,1,915,0,1105,1,922,21201,1,24405,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21101,942,0,0,1106,0,922,22102,1,1,-1,21201,-2,-3,1,21101,0,957,0,1106,0,922,22201,1,-1,-2,1106,0,968,21201,-2,0,-2,109,-3,2106,0,0";
// itself
//"109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
// 16 digit number 
//"1102,34915192,34915192,7,4,7,99,0";
// 1125899906842624
//"104,1125899906842624,99";

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

fn main() {
    let mut int_code = IntCodeState::from_string(INPUT);

    int_code.input.push_front(2);

    int_code.run();

    if int_code.output.len() > 1 {
        for code in int_code.output.iter() {
            println!("Code {} Output", code);
        }
    } else {
        println!("Result = {}", int_code.output.pop_front().unwrap());
    }
}
