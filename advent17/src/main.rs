use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;


// this controls whether to output the screen (PLAY = false)
// or to play the game (PLAY = true)
const PLAY: bool = false;

const DEBUG: bool = false;

const INPUT: &str =
"1,330,331,332,109,3468,1102,1182,1,16,1101,0,1479,24,101,0,0,570,1006,570,36,1002,571,1,0,1001,570,-1,570,1001,24,1,24,1106,0,18,1008,571,0,571,1001,16,1,16,1008,16,1479,570,1006,570,14,21102,58,1,0,1105,1,786,1006,332,62,99,21101,0,333,1,21101,73,0,0,1105,1,579,1101,0,0,572,1102,1,0,573,3,574,101,1,573,573,1007,574,65,570,1005,570,151,107,67,574,570,1005,570,151,1001,574,-64,574,1002,574,-1,574,1001,572,1,572,1007,572,11,570,1006,570,165,101,1182,572,127,101,0,574,0,3,574,101,1,573,573,1008,574,10,570,1005,570,189,1008,574,44,570,1006,570,158,1105,1,81,21101,0,340,1,1106,0,177,21102,477,1,1,1106,0,177,21101,0,514,1,21102,176,1,0,1105,1,579,99,21101,0,184,0,1106,0,579,4,574,104,10,99,1007,573,22,570,1006,570,165,1001,572,0,1182,21102,375,1,1,21102,211,1,0,1105,1,579,21101,1182,11,1,21102,222,1,0,1106,0,979,21101,0,388,1,21101,233,0,0,1106,0,579,21101,1182,22,1,21102,244,1,0,1106,0,979,21101,0,401,1,21101,0,255,0,1105,1,579,21101,1182,33,1,21102,266,1,0,1105,1,979,21102,414,1,1,21102,1,277,0,1106,0,579,3,575,1008,575,89,570,1008,575,121,575,1,575,570,575,3,574,1008,574,10,570,1006,570,291,104,10,21101,1182,0,1,21102,313,1,0,1106,0,622,1005,575,327,1101,1,0,575,21101,0,327,0,1105,1,786,4,438,99,0,1,1,6,77,97,105,110,58,10,33,10,69,120,112,101,99,116,101,100,32,102,117,110,99,116,105,111,110,32,110,97,109,101,32,98,117,116,32,103,111,116,58,32,0,12,70,117,110,99,116,105,111,110,32,65,58,10,12,70,117,110,99,116,105,111,110,32,66,58,10,12,70,117,110,99,116,105,111,110,32,67,58,10,23,67,111,110,116,105,110,117,111,117,115,32,118,105,100,101,111,32,102,101,101,100,63,10,0,37,10,69,120,112,101,99,116,101,100,32,82,44,32,76,44,32,111,114,32,100,105,115,116,97,110,99,101,32,98,117,116,32,103,111,116,58,32,36,10,69,120,112,101,99,116,101,100,32,99,111,109,109,97,32,111,114,32,110,101,119,108,105,110,101,32,98,117,116,32,103,111,116,58,32,43,10,68,101,102,105,110,105,116,105,111,110,115,32,109,97,121,32,98,101,32,97,116,32,109,111,115,116,32,50,48,32,99,104,97,114,97,99,116,101,114,115,33,10,94,62,118,60,0,1,0,-1,-1,0,1,0,0,0,0,0,0,1,24,22,0,109,4,2102,1,-3,586,21002,0,1,-1,22101,1,-3,-3,21101,0,0,-2,2208,-2,-1,570,1005,570,617,2201,-3,-2,609,4,0,21201,-2,1,-2,1105,1,597,109,-4,2106,0,0,109,5,1202,-4,1,630,20101,0,0,-2,22101,1,-4,-4,21102,0,1,-3,2208,-3,-2,570,1005,570,781,2201,-4,-3,652,21002,0,1,-1,1208,-1,-4,570,1005,570,709,1208,-1,-5,570,1005,570,734,1207,-1,0,570,1005,570,759,1206,-1,774,1001,578,562,684,1,0,576,576,1001,578,566,692,1,0,577,577,21101,702,0,0,1105,1,786,21201,-1,-1,-1,1105,1,676,1001,578,1,578,1008,578,4,570,1006,570,724,1001,578,-4,578,21101,0,731,0,1105,1,786,1105,1,774,1001,578,-1,578,1008,578,-1,570,1006,570,749,1001,578,4,578,21101,756,0,0,1106,0,786,1105,1,774,21202,-1,-11,1,22101,1182,1,1,21102,1,774,0,1105,1,622,21201,-3,1,-3,1105,1,640,109,-5,2106,0,0,109,7,1005,575,802,20101,0,576,-6,20101,0,577,-5,1106,0,814,21102,0,1,-1,21101,0,0,-5,21102,0,1,-6,20208,-6,576,-2,208,-5,577,570,22002,570,-2,-2,21202,-5,51,-3,22201,-6,-3,-3,22101,1479,-3,-3,1202,-3,1,843,1005,0,863,21202,-2,42,-4,22101,46,-4,-4,1206,-2,924,21101,1,0,-1,1106,0,924,1205,-2,873,21102,1,35,-4,1106,0,924,1201,-3,0,878,1008,0,1,570,1006,570,916,1001,374,1,374,1201,-3,0,895,1101,0,2,0,1201,-3,0,902,1001,438,0,438,2202,-6,-5,570,1,570,374,570,1,570,438,438,1001,578,558,922,20101,0,0,-4,1006,575,959,204,-4,22101,1,-6,-6,1208,-6,51,570,1006,570,814,104,10,22101,1,-5,-5,1208,-5,39,570,1006,570,810,104,10,1206,-1,974,99,1206,-1,974,1102,1,1,575,21101,973,0,0,1105,1,786,99,109,-7,2106,0,0,109,6,21102,0,1,-4,21102,1,0,-3,203,-2,22101,1,-3,-3,21208,-2,82,-1,1205,-1,1030,21208,-2,76,-1,1205,-1,1037,21207,-2,48,-1,1205,-1,1124,22107,57,-2,-1,1205,-1,1124,21201,-2,-48,-2,1106,0,1041,21102,1,-4,-2,1106,0,1041,21102,-5,1,-2,21201,-4,1,-4,21207,-4,11,-1,1206,-1,1138,2201,-5,-4,1059,1201,-2,0,0,203,-2,22101,1,-3,-3,21207,-2,48,-1,1205,-1,1107,22107,57,-2,-1,1205,-1,1107,21201,-2,-48,-2,2201,-5,-4,1090,20102,10,0,-1,22201,-2,-1,-2,2201,-5,-4,1103,2102,1,-2,0,1105,1,1060,21208,-2,10,-1,1205,-1,1162,21208,-2,44,-1,1206,-1,1131,1105,1,989,21102,439,1,1,1105,1,1150,21102,477,1,1,1105,1,1150,21102,1,514,1,21101,1149,0,0,1105,1,579,99,21101,0,1157,0,1106,0,579,204,-2,104,10,99,21207,-3,22,-1,1206,-1,1138,1201,-5,0,1176,1202,-4,1,0,109,-6,2105,1,0,22,7,44,1,5,1,40,7,3,1,40,1,3,1,1,1,3,1,22,9,9,1,3,1,1,1,3,1,22,1,7,1,9,1,3,1,1,1,3,1,22,1,7,1,1,13,1,1,3,1,22,1,7,1,1,1,7,1,5,1,3,1,22,1,7,1,1,1,7,1,3,7,22,1,7,1,1,1,7,1,5,1,26,7,1,9,1,9,30,1,3,1,5,1,7,1,1,1,30,1,3,1,5,1,7,1,1,1,30,1,3,1,5,1,7,1,1,1,30,1,3,1,1,13,1,1,9,7,14,1,3,1,1,1,3,1,9,1,9,1,5,1,14,1,3,1,1,1,3,1,9,9,1,1,5,1,14,1,3,1,1,1,3,1,17,1,1,1,5,1,14,1,3,7,17,1,1,1,5,1,14,1,5,1,21,1,1,1,5,1,14,1,5,1,21,1,1,1,5,1,14,1,5,1,21,1,1,1,5,1,14,7,11,13,1,9,38,1,3,1,3,1,3,1,38,1,3,1,3,1,3,1,38,1,3,1,3,1,3,1,38,1,3,1,3,9,34,1,3,1,7,1,3,1,34,13,3,1,38,1,11,1,32,7,11,1,32,1,17,1,32,1,5,13,32,1,5,1,44,1,5,1,44,1,5,1,44,1,5,1,44,1,5,1,44,7,12";

pub type Value = i128;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Scaffold,
}

impl Tile {
    pub fn from_code(value: Value) -> Tile {
        let left = '<' as Value;
        let right = '>' as Value;
        let up = '^' as Value;
        let down = 'v' as Value;
        match value {
            35 => Tile::Scaffold,
            46 => Tile::Empty,
            left => Tile::Scaffold,
            right => Tile::Scaffold,
            down => Tile::Scaffold,
            up => Tile::Scaffold,
            _ => panic!(format!("Unexpected output ({})!", value)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    pub fn to_input(&self) -> Value {
        match self {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West  => 3,
            Dir::East  => 4,
        }
    }

    pub fn to_output(&self) -> &str {
        match self {
            Dir::North => "^",
            Dir::South => "v",
            Dir::West  => ">",
            Dir::East  => "<",
        }
    }

    pub fn rotate_left(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::South => Dir::East,
            Dir::West  => Dir::South,
            Dir::East  => Dir::North,
        }
    }

    pub fn rotate_right(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::West  => Dir::North,
            Dir::East  => Dir::South,
        }
    }

    pub fn move_dir(&self, loc: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::North => (loc.0, loc.1 - 1),
            Dir::East => (loc.0 + 1, loc.1),
            Dir::South => (loc.0, loc.1 + 1),
            Dir::West => (loc.0 - 1, loc.1),
        }
    }
}

pub type Map = Vec<Vec<Tile>>;


pub fn print_map(map: &Map, loc: (usize, usize), dir: Dir) {
    println!("");
    for x in 0..map[0].len() {
        print!("_");
    }
    println!("");
    for y in 0..map.len() {
        print!("|");
        for x in 0..map[0].len() {
            match map[y][x] {
                Tile::Empty => {
                    print!(" ");
                }

                Tile::Scaffold => {
                    if (x, y) == loc {
                        print!("{}", dir.to_output());
                    } else {
                        print!("#");
                    }
                }
            }
        }
        print!("|");
        println!("");
    }
    for x in 0..map[0].len() {
        print!("_");
    }
    println!("");
}

pub fn clear_console() {
    print!("{}[2J", 27 as char);
}

pub fn wait_for_input() {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string);
}

fn main() {
    /* Set Up Map */
    let mut int_code = IntCodeState::from_string(INPUT);

    let size = 70;
    let mut map = vec![vec![Tile::Empty; size]; size];
    let mut loc = (size / 2, size / 2);
    let start_loc: (usize, usize)= (size / 2, size / 2);
    let mut oxygen_loc = (0, 0);

    let mut status = Status::Step;

    let mut block_count = 0;
    let mut dir = Dir::West;

    let mut iterations = 0;
    while status != Status::Exit {
        int_code.input.push_front(dir.to_input());
        status = int_code.run();

        println!("output length = {}", int_code.output.len());

        //if status == Status::Blocked {
            let mut x = 1;
            let mut y = 1;
            while int_code.output.len() > 0 {
                let code = int_code.output.pop_front().unwrap();

                if code == 10 {
                    y += 1;
                    x = 1;
                } else {
                    map[y][x] = Tile::from_code(code);

                    if code == '>' as Value {
                        loc = (x, y);
                        dir = Dir::East;
                        println!("east ({}, {})", loc.0, loc.1);
                    } else if code == '<' as Value {
                        loc = (x, y);
                        dir = Dir::West;
                        println!("west ({}, {})", loc.0, loc.1);
                    } else if code == 'v' as Value {
                        loc = (x, y);
                        dir = Dir::South;
                        println!("south ({}, {})", loc.0, loc.1);
                    } else if code == '^' as Value {
                        loc = (x, y);
                        dir = Dir::North;
                        println!("north ({}, {})", loc.0, loc.1);
                    }

                    x += 1;
                }
            //}
        }

        clear_console();
        print_map(&map, loc, dir);

        if iterations > 100000 {
            break;
        }
        iterations += 1;

        let mut answer = 0;
        for y in 1..(map.len() - 1) {
            for x in 1..(map[0].len() - 1) {
                if map[y-1][x] == Tile::Scaffold &&
                   map[y+1][x] == Tile::Scaffold &&
                   map[y][x-1] == Tile::Scaffold &&
                   map[y][x+1] == Tile::Scaffold {
                       answer += x*y;
                }
            }
        }
        //println!("answer = {}", answer);

        break;
    }


    let mut cur_loc = loc;
    let mut cur_dir = dir;
    let mut action_list: Vec<String> = Vec::new();
    let mut move_amount = 0;
    loop {
        let next_loc = dir.move_dir(cur_loc);
        if map[next_loc.1][next_loc.0] != Tile::Scaffold {
            if move_amount > 0 {
                action_list.push(move_amount.to_string());
                move_amount = 0;
            }

            let right = dir.rotate_right();
            let left = dir.rotate_left();
            let right_dir = right.move_dir(cur_loc);
            let left_dir = left.move_dir(cur_loc);

            if map[right_dir.1][right_dir.0] == Tile::Scaffold {
                action_list.push("R".to_string());
                dir = right;
            } else if map[left_dir.1][left_dir.0] == Tile::Scaffold {
                action_list.push("L".to_string());
                dir = left;
            } else {
                //apparently we are at the end now.
                break;
            }
        } else {
            move_amount += 1;
            cur_loc = next_loc;
        }
        //dbg!(&action_list);
    }
    println!("{}", action_list.join(","));

    let mut int_code = IntCodeState::from_string(INPUT);
    *int_code.memory.get_mut(&0).unwrap() = 2;

    let main_routine = "A,A,B,C,A,C,A,B,C,B\n";
    let a_routine = "R,12,L,8,R,6\n";
    let b_routine = "R,12,L,6,R,6,R,8,R,6\n";
    let c_routine = "L,8,R,8,R,6,R,12\n";
    let prompt = "n\n";

    int_code.input = 
        main_routine.chars()
        .chain(a_routine.chars())
        .chain(b_routine.chars())
        .chain(c_routine.chars())
        .chain(prompt.chars())
        .map(|ch| ch as Value)
        .collect::<VecDeque<Value>>();

    status = Status::Step;
    while status != Status::Exit {
        status = int_code.run();
        dbg!(&int_code.output);
    }
}
