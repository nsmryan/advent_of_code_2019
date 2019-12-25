use std::convert::TryFrom;
use std::collections::VecDeque;
use std::collections::HashMap;

//use rayon::prelude::*;

const NUM_COMPUTERS: usize = 50;

const DEBUG: bool = false;

const INPUT: &str =
"3,62,1001,62,11,10,109,2263,105,1,0,1037,2127,1368,1115,862,1331,600,1201,2022,1599,1793,2232,724,693,1300,1956,2199,1667,1857,1078,1434,1399,1754,631,1987,1630,1002,1725,831,765,1502,1236,1824,1152,660,571,893,1696,959,1469,796,2162,2086,928,1568,1535,1269,1923,2053,1888,0,0,0,0,0,0,0,0,0,0,0,0,3,64,1008,64,-1,62,1006,62,88,1006,61,170,1105,1,73,3,65,20102,1,64,1,20101,0,66,2,21102,1,105,0,1105,1,436,1201,1,-1,64,1007,64,0,62,1005,62,73,7,64,67,62,1006,62,73,1002,64,2,133,1,133,68,133,102,1,0,62,1001,133,1,140,8,0,65,63,2,63,62,62,1005,62,73,1002,64,2,161,1,161,68,161,1102,1,1,0,1001,161,1,169,1001,65,0,0,1102,1,1,61,1101,0,0,63,7,63,67,62,1006,62,203,1002,63,2,194,1,68,194,194,1006,0,73,1001,63,1,63,1105,1,178,21102,210,1,0,105,1,69,1202,1,1,70,1101,0,0,63,7,63,71,62,1006,62,250,1002,63,2,234,1,72,234,234,4,0,101,1,234,240,4,0,4,70,1001,63,1,63,1106,0,218,1105,1,73,109,4,21101,0,0,-3,21102,0,1,-2,20207,-2,67,-1,1206,-1,293,1202,-2,2,283,101,1,283,283,1,68,283,283,22001,0,-3,-3,21201,-2,1,-2,1105,1,263,22101,0,-3,-3,109,-4,2106,0,0,109,4,21101,0,1,-3,21101,0,0,-2,20207,-2,67,-1,1206,-1,342,1202,-2,2,332,101,1,332,332,1,68,332,332,22002,0,-3,-3,21201,-2,1,-2,1106,0,312,21201,-3,0,-3,109,-4,2106,0,0,109,1,101,1,68,359,20101,0,0,1,101,3,68,366,21001,0,0,2,21101,376,0,0,1106,0,436,22102,1,1,0,109,-1,2106,0,0,1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192,16384,32768,65536,131072,262144,524288,1048576,2097152,4194304,8388608,16777216,33554432,67108864,134217728,268435456,536870912,1073741824,2147483648,4294967296,8589934592,17179869184,34359738368,68719476736,137438953472,274877906944,549755813888,1099511627776,2199023255552,4398046511104,8796093022208,17592186044416,35184372088832,70368744177664,140737488355328,281474976710656,562949953421312,1125899906842624,109,8,21202,-6,10,-5,22207,-7,-5,-5,1205,-5,521,21102,0,1,-4,21101,0,0,-3,21102,1,51,-2,21201,-2,-1,-2,1201,-2,385,471,20101,0,0,-1,21202,-3,2,-3,22207,-7,-1,-5,1205,-5,496,21201,-3,1,-3,22102,-1,-1,-5,22201,-7,-5,-7,22207,-3,-6,-5,1205,-5,515,22102,-1,-6,-5,22201,-3,-5,-3,22201,-1,-4,-4,1205,-2,461,1106,0,547,21101,-1,0,-4,21202,-6,-1,-6,21207,-7,0,-5,1205,-5,547,22201,-7,-6,-7,21201,-4,1,-4,1105,1,529,22102,1,-4,-7,109,-8,2106,0,0,109,1,101,1,68,564,20102,1,0,0,109,-1,2105,1,0,1101,0,15271,66,1102,1,1,67,1102,1,598,68,1101,0,556,69,1101,0,0,71,1102,1,600,72,1106,0,73,1,1520,1101,16987,0,66,1101,1,0,67,1102,627,1,68,1102,1,556,69,1102,1,1,71,1101,629,0,72,1105,1,73,1,150,5,45389,1101,0,39397,66,1102,1,1,67,1102,658,1,68,1101,0,556,69,1102,0,1,71,1102,1,660,72,1105,1,73,1,1717,1101,35977,0,66,1102,2,1,67,1101,0,687,68,1101,0,302,69,1102,1,1,71,1102,691,1,72,1105,1,73,0,0,0,0,42,92154,1102,1,53197,66,1102,1,1,67,1102,1,720,68,1101,556,0,69,1101,0,1,71,1101,722,0,72,1105,1,73,1,1051,26,124071,1102,1,94379,66,1101,0,6,67,1101,751,0,68,1102,1,302,69,1101,1,0,71,1102,763,1,72,1106,0,73,0,0,0,0,0,0,0,0,0,0,0,0,32,205586,1101,25357,0,66,1101,0,1,67,1102,792,1,68,1101,556,0,69,1101,1,0,71,1102,794,1,72,1105,1,73,1,125,41,73354,1101,0,73751,66,1102,3,1,67,1102,1,823,68,1101,0,302,69,1101,1,0,71,1101,0,829,72,1106,0,73,0,0,0,0,0,0,22,172495,1102,1,47857,66,1102,1,1,67,1102,1,858,68,1101,0,556,69,1101,0,1,71,1101,0,860,72,1105,1,73,1,24877,30,14753,1101,72269,0,66,1102,1,1,67,1101,0,889,68,1101,556,0,69,1102,1,1,71,1102,1,891,72,1106,0,73,1,-150,19,54581,1102,1,92173,66,1102,1,3,67,1102,1,920,68,1102,1,302,69,1101,0,1,71,1102,926,1,72,1106,0,73,0,0,0,0,0,0,22,103497,1102,1,27779,66,1102,1,1,67,1101,955,0,68,1102,1,556,69,1102,1,1,71,1101,0,957,72,1106,0,73,1,3083,49,292749,1102,81283,1,66,1102,1,1,67,1101,0,986,68,1102,1,556,69,1102,1,7,71,1102,1,988,72,1106,0,73,1,5,5,90778,5,181556,42,61436,21,95267,41,36677,41,110031,12,188758,1101,0,41357,66,1102,1,3,67,1102,1,1029,68,1101,0,302,69,1102,1,1,71,1101,0,1035,72,1105,1,73,0,0,0,0,0,0,22,137996,1101,0,9257,66,1101,0,1,67,1102,1064,1,68,1102,1,556,69,1101,6,0,71,1102,1066,1,72,1105,1,73,1,17740,45,129566,7,7451,7,22353,1,50153,1,100306,1,150459,1102,54581,1,66,1102,4,1,67,1101,1105,0,68,1102,302,1,69,1101,0,1,71,1102,1,1113,72,1106,0,73,0,0,0,0,0,0,0,0,7,14902,1102,5273,1,66,1102,1,1,67,1101,1142,0,68,1102,556,1,69,1102,4,1,71,1102,1,1144,72,1106,0,73,1,2,42,15359,21,285801,12,94379,12,377516,1102,1,41959,66,1101,1,0,67,1101,0,1179,68,1102,1,556,69,1102,1,10,71,1102,1181,1,72,1106,0,73,1,1,30,29506,31,99754,47,20782,34,71954,49,97583,26,82714,20,74869,36,92173,40,147502,19,218324,1102,7451,1,66,1101,3,0,67,1102,1228,1,68,1102,1,302,69,1101,1,0,71,1102,1234,1,72,1106,0,73,0,0,0,0,0,0,25,7673,1102,1,49877,66,1101,2,0,67,1101,1263,0,68,1101,0,302,69,1102,1,1,71,1102,1267,1,72,1105,1,73,0,0,0,0,47,10391,1101,41081,0,66,1101,0,1,67,1102,1296,1,68,1101,0,556,69,1101,0,1,71,1101,1298,0,72,1106,0,73,1,337,36,276519,1102,76651,1,66,1102,1,1,67,1102,1,1327,68,1101,0,556,69,1101,0,1,71,1101,0,1329,72,1105,1,73,1,2659,40,73751,1101,45389,0,66,1101,0,4,67,1101,0,1358,68,1102,1,302,69,1102,1,1,71,1102,1,1366,72,1105,1,73,0,0,0,0,0,0,0,0,42,76795,1101,99859,0,66,1102,1,1,67,1101,1395,0,68,1102,1,556,69,1101,1,0,71,1102,1,1397,72,1105,1,73,1,28,49,195166,1101,0,95267,66,1102,1,3,67,1101,0,1426,68,1102,1,302,69,1102,1,1,71,1101,0,1432,72,1105,1,73,0,0,0,0,0,0,45,64783,1102,74869,1,66,1102,1,3,67,1101,1461,0,68,1102,302,1,69,1102,1,1,71,1102,1,1467,72,1106,0,73,0,0,0,0,0,0,22,34499,1101,31771,0,66,1101,0,1,67,1101,1496,0,68,1102,556,1,69,1102,2,1,71,1102,1498,1,72,1106,0,73,1,3,42,30718,19,109162,1101,0,14753,66,1101,0,2,67,1102,1529,1,68,1101,302,0,69,1102,1,1,71,1102,1533,1,72,1105,1,73,0,0,0,0,31,49877,1101,64783,0,66,1101,0,2,67,1102,1562,1,68,1102,1,302,69,1102,1,1,71,1102,1,1566,72,1106,0,73,0,0,0,0,25,15346,1101,0,27197,66,1101,1,0,67,1101,0,1595,68,1101,556,0,69,1102,1,1,71,1101,0,1597,72,1106,0,73,1,160,12,471895,1101,0,33857,66,1101,0,1,67,1101,0,1626,68,1101,0,556,69,1102,1,1,71,1101,0,1628,72,1105,1,73,1,17,20,224607,1101,7673,0,66,1101,4,0,67,1101,1657,0,68,1101,0,253,69,1101,1,0,71,1101,1665,0,72,1105,1,73,0,0,0,0,0,0,0,0,32,102793,1101,5987,0,66,1101,1,0,67,1101,0,1694,68,1101,556,0,69,1101,0,0,71,1102,1,1696,72,1106,0,73,1,1790,1102,1,50891,66,1102,1,1,67,1102,1723,1,68,1102,556,1,69,1102,0,1,71,1101,1725,0,72,1105,1,73,1,1961,1102,13159,1,66,1101,1,0,67,1102,1752,1,68,1102,1,556,69,1101,0,0,71,1102,1754,1,72,1106,0,73,1,1741,1101,0,34499,66,1102,5,1,67,1101,0,1781,68,1101,253,0,69,1102,1,1,71,1102,1,1791,72,1105,1,73,0,0,0,0,0,0,0,0,0,0,48,82997,1101,44687,0,66,1102,1,1,67,1102,1820,1,68,1102,556,1,69,1102,1,1,71,1102,1,1822,72,1106,0,73,1,53,40,221253,1102,102793,1,66,1102,2,1,67,1102,1851,1,68,1102,1,351,69,1101,1,0,71,1102,1855,1,72,1105,1,73,0,0,0,0,255,9257,1101,88873,0,66,1101,1,0,67,1102,1884,1,68,1101,0,556,69,1102,1,1,71,1102,1,1886,72,1105,1,73,1,3469,20,149738,1101,97583,0,66,1102,1,3,67,1101,1915,0,68,1101,0,302,69,1102,1,1,71,1101,0,1921,72,1105,1,73,0,0,0,0,0,0,22,68998,1102,10391,1,66,1102,1,2,67,1102,1950,1,68,1101,0,302,69,1102,1,1,71,1102,1954,1,72,1105,1,73,0,0,0,0,34,35977,1101,30089,0,66,1102,1,1,67,1102,1983,1,68,1102,556,1,69,1101,1,0,71,1102,1,1985,72,1106,0,73,1,250,48,165994,1102,1,7253,66,1101,0,1,67,1101,2014,0,68,1102,1,556,69,1101,0,3,71,1101,2016,0,72,1106,0,73,1,10,5,136167,41,146708,12,566274,1102,72467,1,66,1101,1,0,67,1101,0,2049,68,1101,556,0,69,1101,1,0,71,1102,1,2051,72,1106,0,73,1,54,26,41357,1102,82997,1,66,1101,2,0,67,1102,1,2080,68,1102,302,1,69,1102,1,1,71,1101,2084,0,72,1106,0,73,0,0,0,0,21,190534,1101,0,15359,66,1102,1,6,67,1101,0,2113,68,1101,302,0,69,1101,1,0,71,1101,0,2125,72,1105,1,73,0,0,0,0,0,0,0,0,0,0,0,0,25,23019,1101,50153,0,66,1101,0,3,67,1102,1,2154,68,1102,302,1,69,1101,1,0,71,1102,1,2160,72,1106,0,73,0,0,0,0,0,0,25,30692,1102,1,36677,66,1101,0,4,67,1102,2189,1,68,1101,302,0,69,1101,0,1,71,1101,0,2197,72,1105,1,73,0,0,0,0,0,0,0,0,12,283137,1101,293,0,66,1102,1,1,67,1101,2226,0,68,1102,556,1,69,1102,1,2,71,1102,1,2228,72,1106,0,73,1,71,42,46077,19,163743,1102,49297,1,66,1101,1,0,67,1101,2259,0,68,1101,0,556,69,1101,0,1,71,1102,1,2261,72,1105,1,73,1,217,36,184346";

pub type Value = i64;

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


pub type Dest = Value;

pub struct Data(Value, Value);

pub struct Packet(Dest, Data);


fn main() {
    /* Set Up Map */
    let mut int_code = IntCodeState::from_string(INPUT);

    let mut status = Status::Step;

    let mut computers = Vec::new();
    for ix in 0..NUM_COMPUTERS {
        let mut cur_computer = int_code.clone();

        cur_computer.input.push_front(ix as i64);

        computers.push(cur_computer);
    }

    let mut nat_ys = Vec::new();
    let mut nat_xy = (0, 0);
    let mut last_sent_y = 0;

    'outer: loop {
        let mut any_packets = false;
       
        for ix in 0..NUM_COMPUTERS {
            if computers[ix].input.len() == 0 {
                computers[ix].input.push_front(-1);
            } else {
                any_packets = true;
            }

            computers[ix].run();

            while computers[ix].output.len() > 0 {

                let address = computers[ix].output.pop_front().unwrap();
                let x = computers[ix].output.pop_front().unwrap();
                let y = computers[ix].output.pop_front().unwrap();

                if address == 255 {
                    println!("nat_ys.len() = {}", nat_ys.len());

                    nat_xy = (x, y);
                } else {
                    computers[address as usize].input.push_back(x);
                    computers[address as usize].input.push_back(y);
                }
            }
        }

        if !any_packets {
            if nat_ys.contains(&nat_xy.1) {
                break 'outer;
            }

            computers[0].input.push_back(nat_xy.0);
            computers[0].input.push_back(nat_xy.1);
            nat_ys.push(nat_xy.1);
            last_sent_y = nat_xy.1;
        }
    }

    println!("y = {}", last_sent_y);
}
