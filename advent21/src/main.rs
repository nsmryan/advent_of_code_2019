use std::convert::TryFrom;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

use rayon::prelude::*;


const DEBUG: bool = false;

const INPUT: &str =
"109,2050,21101,0,966,1,21101,0,13,0,1105,1,1378,21102,1,20,0,1106,0,1337,21102,1,27,0,1105,1,1279,1208,1,65,748,1005,748,73,1208,1,79,748,1005,748,110,1208,1,78,748,1005,748,132,1208,1,87,748,1005,748,169,1208,1,82,748,1005,748,239,21101,0,1041,1,21101,0,73,0,1106,0,1421,21102,78,1,1,21102,1,1041,2,21102,88,1,0,1105,1,1301,21101,68,0,1,21101,1041,0,2,21101,0,103,0,1105,1,1301,1101,0,1,750,1106,0,298,21102,1,82,1,21101,1041,0,2,21101,125,0,0,1105,1,1301,1101,0,2,750,1105,1,298,21102,79,1,1,21102,1,1041,2,21102,1,147,0,1106,0,1301,21101,0,84,1,21102,1041,1,2,21101,162,0,0,1105,1,1301,1102,3,1,750,1105,1,298,21101,65,0,1,21101,0,1041,2,21101,184,0,0,1105,1,1301,21101,0,76,1,21102,1041,1,2,21102,199,1,0,1105,1,1301,21102,75,1,1,21101,0,1041,2,21102,214,1,0,1106,0,1301,21101,0,221,0,1106,0,1337,21102,1,10,1,21101,1041,0,2,21102,1,236,0,1106,0,1301,1105,1,553,21101,85,0,1,21101,0,1041,2,21101,254,0,0,1105,1,1301,21101,78,0,1,21102,1,1041,2,21101,269,0,0,1106,0,1301,21101,276,0,0,1106,0,1337,21101,10,0,1,21101,0,1041,2,21102,1,291,0,1105,1,1301,1101,0,1,755,1106,0,553,21102,1,32,1,21102,1041,1,2,21101,313,0,0,1106,0,1301,21102,320,1,0,1106,0,1337,21102,327,1,0,1105,1,1279,2101,0,1,749,21102,1,65,2,21102,73,1,3,21101,346,0,0,1106,0,1889,1206,1,367,1007,749,69,748,1005,748,360,1102,1,1,756,1001,749,-64,751,1105,1,406,1008,749,74,748,1006,748,381,1101,0,-1,751,1106,0,406,1008,749,84,748,1006,748,395,1101,0,-2,751,1106,0,406,21102,1,1100,1,21101,406,0,0,1106,0,1421,21101,0,32,1,21102,1100,1,2,21102,421,1,0,1106,0,1301,21101,428,0,0,1106,0,1337,21101,0,435,0,1105,1,1279,1202,1,1,749,1008,749,74,748,1006,748,453,1102,-1,1,752,1105,1,478,1008,749,84,748,1006,748,467,1102,-2,1,752,1106,0,478,21101,0,1168,1,21101,0,478,0,1106,0,1421,21102,485,1,0,1105,1,1337,21102,1,10,1,21102,1168,1,2,21102,1,500,0,1106,0,1301,1007,920,15,748,1005,748,518,21102,1209,1,1,21101,0,518,0,1106,0,1421,1002,920,3,529,1001,529,921,529,101,0,750,0,1001,529,1,537,1002,751,1,0,1001,537,1,545,1002,752,1,0,1001,920,1,920,1105,1,13,1005,755,577,1006,756,570,21102,1,1100,1,21102,1,570,0,1106,0,1421,21102,1,987,1,1106,0,581,21101,1001,0,1,21102,1,588,0,1105,1,1378,1101,758,0,593,1001,0,0,753,1006,753,654,21001,753,0,1,21101,0,610,0,1106,0,667,21101,0,0,1,21101,0,621,0,1106,0,1463,1205,1,647,21102,1,1015,1,21102,1,635,0,1106,0,1378,21101,1,0,1,21102,1,646,0,1105,1,1463,99,1001,593,1,593,1106,0,592,1006,755,664,1101,0,0,755,1105,1,647,4,754,99,109,2,1101,0,726,757,21201,-1,0,1,21102,9,1,2,21102,1,697,3,21102,692,1,0,1106,0,1913,109,-2,2106,0,0,109,2,101,0,757,706,2101,0,-1,0,1001,757,1,757,109,-2,2106,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,255,63,191,95,127,223,159,0,77,78,179,124,86,202,126,207,249,163,93,53,87,138,173,103,184,232,118,51,227,35,204,250,107,234,108,114,219,123,109,166,181,98,215,185,190,137,213,242,122,169,119,241,92,120,226,238,79,167,158,111,56,230,229,178,84,115,156,125,171,100,85,141,157,34,71,188,186,239,187,153,121,177,246,237,39,245,253,116,214,175,216,203,233,139,236,198,155,205,140,196,152,43,221,62,252,170,94,212,70,38,76,222,218,99,231,117,50,60,247,59,54,58,182,228,200,243,168,172,143,251,102,174,248,206,110,197,201,220,244,42,113,106,69,101,57,46,68,254,199,217,183,162,55,49,154,61,47,142,136,235,189,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,20,73,110,112,117,116,32,105,110,115,116,114,117,99,116,105,111,110,115,58,10,13,10,87,97,108,107,105,110,103,46,46,46,10,10,13,10,82,117,110,110,105,110,103,46,46,46,10,10,25,10,68,105,100,110,39,116,32,109,97,107,101,32,105,116,32,97,99,114,111,115,115,58,10,10,58,73,110,118,97,108,105,100,32,111,112,101,114,97,116,105,111,110,59,32,101,120,112,101,99,116,101,100,32,115,111,109,101,116,104,105,110,103,32,108,105,107,101,32,65,78,68,44,32,79,82,44,32,111,114,32,78,79,84,67,73,110,118,97,108,105,100,32,102,105,114,115,116,32,97,114,103,117,109,101,110,116,59,32,101,120,112,101,99,116,101,100,32,115,111,109,101,116,104,105,110,103,32,108,105,107,101,32,65,44,32,66,44,32,67,44,32,68,44,32,74,44,32,111,114,32,84,40,73,110,118,97,108,105,100,32,115,101,99,111,110,100,32,97,114,103,117,109,101,110,116,59,32,101,120,112,101,99,116,101,100,32,74,32,111,114,32,84,52,79,117,116,32,111,102,32,109,101,109,111,114,121,59,32,97,116,32,109,111,115,116,32,49,53,32,105,110,115,116,114,117,99,116,105,111,110,115,32,99,97,110,32,98,101,32,115,116,111,114,101,100,0,109,1,1005,1262,1270,3,1262,20101,0,1262,0,109,-1,2105,1,0,109,1,21102,1,1288,0,1105,1,1263,20102,1,1262,0,1102,0,1,1262,109,-1,2105,1,0,109,5,21102,1310,1,0,1105,1,1279,21202,1,1,-2,22208,-2,-4,-1,1205,-1,1332,22101,0,-3,1,21101,0,1332,0,1106,0,1421,109,-5,2105,1,0,109,2,21102,1346,1,0,1105,1,1263,21208,1,32,-1,1205,-1,1363,21208,1,9,-1,1205,-1,1363,1106,0,1373,21102,1,1370,0,1105,1,1279,1105,1,1339,109,-2,2105,1,0,109,5,2102,1,-4,1385,21002,0,1,-2,22101,1,-4,-4,21102,0,1,-3,22208,-3,-2,-1,1205,-1,1416,2201,-4,-3,1408,4,0,21201,-3,1,-3,1105,1,1396,109,-5,2106,0,0,109,2,104,10,21201,-1,0,1,21102,1,1436,0,1105,1,1378,104,10,99,109,-2,2105,1,0,109,3,20002,593,753,-1,22202,-1,-2,-1,201,-1,754,754,109,-3,2105,1,0,109,10,21102,1,5,-5,21102,1,1,-4,21102,1,0,-3,1206,-9,1555,21101,0,3,-6,21101,5,0,-7,22208,-7,-5,-8,1206,-8,1507,22208,-6,-4,-8,1206,-8,1507,104,64,1106,0,1529,1205,-6,1527,1201,-7,716,1515,21002,0,-11,-8,21201,-8,46,-8,204,-8,1105,1,1529,104,46,21201,-7,1,-7,21207,-7,22,-8,1205,-8,1488,104,10,21201,-6,-1,-6,21207,-6,0,-8,1206,-8,1484,104,10,21207,-4,1,-8,1206,-8,1569,21102,1,0,-9,1106,0,1689,21208,-5,21,-8,1206,-8,1583,21101,1,0,-9,1105,1,1689,1201,-5,716,1589,20101,0,0,-2,21208,-4,1,-1,22202,-2,-1,-1,1205,-2,1613,21201,-5,0,1,21102,1613,1,0,1106,0,1444,1206,-1,1634,21201,-5,0,1,21102,1,1627,0,1105,1,1694,1206,1,1634,21102,1,2,-3,22107,1,-4,-8,22201,-1,-8,-8,1206,-8,1649,21201,-5,1,-5,1206,-3,1663,21201,-3,-1,-3,21201,-4,1,-4,1106,0,1667,21201,-4,-1,-4,21208,-4,0,-1,1201,-5,716,1676,22002,0,-1,-1,1206,-1,1686,21101,0,1,-4,1106,0,1477,109,-10,2105,1,0,109,11,21102,0,1,-6,21102,0,1,-8,21101,0,0,-7,20208,-6,920,-9,1205,-9,1880,21202,-6,3,-9,1201,-9,921,1724,21001,0,0,-5,1001,1724,1,1733,20102,1,0,-4,22101,0,-4,1,21102,1,1,2,21102,9,1,3,21102,1754,1,0,1106,0,1889,1206,1,1772,2201,-10,-4,1766,1001,1766,716,1766,21001,0,0,-3,1106,0,1790,21208,-4,-1,-9,1206,-9,1786,21201,-8,0,-3,1106,0,1790,22101,0,-7,-3,1001,1733,1,1795,21002,0,1,-2,21208,-2,-1,-9,1206,-9,1812,22102,1,-8,-1,1106,0,1816,22101,0,-7,-1,21208,-5,1,-9,1205,-9,1837,21208,-5,2,-9,1205,-9,1844,21208,-3,0,-1,1105,1,1855,22202,-3,-1,-1,1105,1,1855,22201,-3,-1,-1,22107,0,-1,-1,1106,0,1855,21208,-2,-1,-9,1206,-9,1869,22101,0,-1,-8,1106,0,1873,22102,1,-1,-7,21201,-6,1,-6,1106,0,1708,22102,1,-8,-10,109,-11,2105,1,0,109,7,22207,-6,-5,-3,22207,-4,-6,-2,22201,-3,-2,-1,21208,-1,0,-6,109,-7,2106,0,0,0,109,5,2101,0,-2,1912,21207,-4,0,-1,1206,-1,1930,21101,0,0,-4,21201,-4,0,1,22102,1,-3,2,21101,0,1,3,21102,1949,1,0,1105,1,1954,109,-5,2105,1,0,109,6,21207,-4,1,-1,1206,-1,1977,22207,-5,-3,-1,1206,-1,1977,21202,-5,1,-5,1106,0,2045,22101,0,-5,1,21201,-4,-1,2,21202,-3,2,3,21102,1,1996,0,1105,1,1954,21202,1,1,-5,21101,1,0,-2,22207,-5,-3,-1,1206,-1,2015,21101,0,0,-2,22202,-3,-2,-3,22107,0,-4,-1,1206,-1,2037,21201,-2,0,1,21102,2037,1,0,105,1,1912,21202,-3,-1,-3,22201,-5,-3,-5,109,-6,2105,1,0";

pub type Value = i32;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Step,
    Blocked,
    Exit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
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
pub struct Loc {
    pub loc: Value,
    pub mode: Mode,
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
pub enum Instr {
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
pub struct IntCodeState {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JumpReg {
    T,
    J,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

impl JumpReg {
    pub fn compile(self) -> String {
        match self {
            JumpReg::T => "T".to_string(),
            JumpReg::J => "J".to_string(),
            JumpReg::A => "A".to_string(),
            JumpReg::B => "B".to_string(),
            JumpReg::C => "C".to_string(),
            JumpReg::D => "D".to_string(),
            JumpReg::E => "E".to_string(),
            JumpReg::F => "F".to_string(),
            JumpReg::G => "G".to_string(),
            JumpReg::H => "H".to_string(),
            JumpReg::I => "I".to_string(),
        }
    }

    pub fn all() -> Vec<JumpReg> {
        use JumpReg::*;
        return vec!(T, J, A, B, C, D, E, F, G, H, I);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JumpWriteReg {
    T,
    J,
}

impl JumpWriteReg {
    pub fn compile(self) -> String {
        match self {
            JumpWriteReg::T => "T".to_string(),
            JumpWriteReg::J => "J".to_string(),
        }
    }

    pub fn all() -> Vec<JumpWriteReg> {
        use JumpWriteReg::*;
        return vec!(T, J);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JumpCode {
    And(JumpReg, JumpWriteReg),
    Or(JumpReg, JumpWriteReg),
    Not(JumpReg, JumpWriteReg),
}

impl JumpCode {
    pub fn compile(self) -> String{
        match self {
            JumpCode::And(reg, write_reg) => {
                format!("NOT {} {}", reg.compile(), write_reg.compile())
            },

            JumpCode::Or(reg, write_reg) => {
                format!("NOT {} {}", reg.compile(), write_reg.compile())
            },

            JumpCode::Not(reg, write_reg) => {
                format!("NOT {} {}", reg.compile(), write_reg.compile())
            },

        }
    }
}

pub fn all_jump_code_helper(current: Vec<JumpCode>, progs: &mut Vec<Vec<JumpCode>>) {
    if progs.len() % 100000 == 0 {
        println!("len = {}", progs.len());
    }
    if current.len() == 15 {
        return;
    }

    for reg in JumpReg::all() {
        for write_reg in JumpWriteReg::all() {
            let mut and_prog = current.clone();
            and_prog.push(JumpCode::And(reg, write_reg));
            progs.push(and_prog.clone());
            all_jump_code_helper(and_prog.clone(), progs);

            let mut or_prog = current.clone();
            or_prog.push(JumpCode::Or(reg, write_reg));
            progs.push(or_prog.clone());
            all_jump_code_helper(or_prog.clone(), progs);

            let mut not_prog = current.clone();
            not_prog.push(JumpCode::Not(reg, write_reg));
            progs.push(not_prog.clone());
            all_jump_code_helper(not_prog.clone(), progs);
        }
    }
}
 
pub fn all_jump_code_programs() -> Vec<Vec<JumpCode>> {
    let mut progs = Vec::new();

    all_jump_code_helper(Vec::new(), &mut progs);

    return progs;
}

pub fn compile_jump_code(jump_code: &[JumpCode]) -> String {
    let mut result = "".to_string();

    for code in jump_code {
        result = format!("{}{}\n", result, code.compile()).to_string();
    }

    return result;
}

fn main() {
    /* Set Up Map */
    let mut int_code = IntCodeState::from_string(INPUT);

    let mut status = Status::Step;

    let jump_code =
        [
        // test end of vision
          "OR D T\n"
        , "AND I T\n"
        , "AND E T\n"
        , "OR H T\n"
          
        // clear J
        , "AND D T\n"
        , "NOT T J\n"
        , "NOT J J\n"

        , "AND T J\n"
        , "OR D T\n"
        , "AND A T\n"
        , "AND B T\n"
        , "AND C T\n"
        , "NOT T T\n"
        , "AND T J\n"
        ].join("");;

    /*
    all_jump_code_programs().into_par_iter().map(|prog| {
        let mut cur_int_code = int_code.clone();

        cur_int_code.run();
        for ch in cur_int_code.output.iter() {
            print!("{}", u8::try_from(*ch).unwrap() as char);
        }
        cur_int_code.output.clear();

        cur_int_code.input = 
            jump_code.chars()
            .chain("RUN\n".chars())
            .map(|ch| ch as Value)
            .collect::<VecDeque<Value>>();
        cur_int_code.run();

        if cur_int_code.output[0] > 128 {
            println!("Answer = {}", cur_int_code.output[0]);
        } else {
            for value in cur_int_code.output.iter() {
                if let Ok(ch) = u8::try_from(*value) {
                    print!("{}", ch as char);
                } else {
                    println!("Hull damage {}", value);
                    return;
                }
            }
        }
    });
    */

    int_code.run();
    for ch in int_code.output.iter() {
        print!("{}", u8::try_from(*ch).unwrap() as char);
    }
    int_code.output.clear();

    int_code.input = 
        jump_code.chars()
        .chain("RUN\n".chars())
        .map(|ch| ch as Value)
        .collect::<VecDeque<Value>>();
    int_code.run();

    if int_code.output[0] > 128 {
        println!("Answer = {}", int_code.output[0]);
    } else {
        for value in int_code.output.iter() {
            if let Ok(ch) = u8::try_from(*value) {
                print!("{}", ch as char);
            } else {
                println!("Hull damage {}", value);
                return;
            }
        }
    }
}
