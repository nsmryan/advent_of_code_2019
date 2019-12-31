use std::collections::HashSet;
use std::collections::HashMap;

const INPUT: &str =
"....#
#..#.
#..##
..#..
#....";

//"....#
//#..#.
//##..#
//#.###
//.####";

pub type Loc = (usize, usize);

pub type Depth = isize;

#[derive(Clone, Debug)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Map {
        return Map { tiles };
    }

    pub fn print(&self) {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                print!("{}", tile.to_char());
            }
            println!("");
        }
    }

    pub fn serialize(&self) -> u32 {
        let mut serial = 0;

        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if self.tiles[y][x] == Tile::Bug {
                    serial |= 1 << ((y * 5) + x);
                }
            }
        }

        return serial;
    }

    pub fn num_bugs(&self) -> u32 {
        let mut count = 0;

        for row in self.tiles.iter() {
            for tile in row.iter() {
                if *tile == Tile::Bug {
                    count += 1;
                }
            }
        }

        return count;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    Bug,
}

impl Tile {
    pub fn from_char(ch: char) -> Tile {
        match ch {
            '#' => Tile::Bug,
            '.' => Tile::Empty,
            _ => panic!("Unexpected character!"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Bug => '#',
        }
    }
}

pub fn parse_input(input: &str) -> Map {
    let mut tiles = Vec::new();

    for line in input.split("\n") {
        let mut row = Vec::new();

        for ch in line.chars() {
            row.push(Tile::from_char(ch));
        }

        tiles.push(row);
    }

    return Map::new(tiles);
}

pub fn empty_maps() -> Vec<Map> {
    let mut empty_tiles = Vec::new();

    for _ in 0..5 {
        let empty_row = (0..5).map(|_| Tile::Empty).collect::<Vec<Tile>>();
        empty_tiles.push(empty_row);
    }
    let empty_map = Map { tiles: empty_tiles };

    let mut maps = Vec::new();
    for _ in 0..500 {
        maps.push(empty_map.clone());
    }

    return maps;
}

pub fn adjacent(x: usize, y: usize, width: usize, height: usize) -> Vec<((usize, usize), Depth)> {
    let mut adj = Vec::new();

    if x > 0 {
        adj.push(((x - 1, y), 0));
    }

    if x < width - 1 {
        adj.push(((x + 1, y), 0));
    }

    if y > 0 {
        adj.push(((x, y - 1), 0));
    }

    if y < height - 1 {
        adj.push(((x, y + 1), 0));
    }

    return adj;
}

pub fn step(maps: &Vec<Map>,
            adj_map: &HashMap<(usize, usize), Vec<((usize, usize), Depth)>>) -> Vec<Map> {
    let height = maps[0].tiles.len();
    let width = maps[0].tiles[0].len();


    let mut next_maps = maps.clone();

    for map_ix in 1..(maps.len() - 1) {
        for y in 0..height {
            for x in 0..width {
                if y == 2 && x == 2 {
                    continue;
                }

                let mut sum = 0;

                for (adj, depth) in adj_map.get(&(x, y)).expect("couldn't get position").iter() {
                    //println!("{:?}, {}", adj, depth);
                    let map_offset = map_ix as isize + depth;
                    if maps[map_offset as usize].tiles[adj.1][adj.0] == Tile::Bug {
                        sum += 1;
                    }
                }

                let sum = sum;

                match maps[map_ix].tiles[y][x] {
                    Tile::Empty => {
                        if sum == 1 || sum == 2 {
                            next_maps[map_ix].tiles[y][x] = Tile::Bug;
                        }
                    }

                    Tile::Bug => {
                        if sum != 1 {
                            next_maps[map_ix].tiles[y][x] = Tile::Empty;
                        }
                    }
                }
            }
        }
    }

    return next_maps;
}

fn main() {
    let mut map = parse_input(INPUT);

    map.print();
    println!("_____");
    println!("Starts with {:X}", map.serialize());

    let height = map.tiles.len();
    let width = map.tiles[0].len();

    let mut adj_map: HashMap<Loc, Vec<(Loc, Depth)>> = HashMap::new();
    // easy cases
    adj_map.insert((1, 1), adjacent(1, 1, width, height)); // 7
    adj_map.insert((3, 1), adjacent(3, 1, width, height)); // 9
    adj_map.insert((1, 3), adjacent(1, 3, width, height)); // 17
    adj_map.insert((3, 3), adjacent(3, 3, width, height)); // 19

    // map inner
    // left mid 12
    let mut adj =
        vec!(((0, 2), 0), ((1, 1), 0), ((1, 3), 0),
             ((0, 0), 1), ((0, 1), 1), ((0, 2), 1),
             ((0, 3), 1), ((0, 4), 1));
    adj_map.insert((1, 2), adj);

    // right mid 14
    let mut adj =
        vec!(((3, 1), 0), ((3, 3), 0), ((4, 2), 0),
             ((4, 0), 1), ((4, 1), 1), ((4, 2), 1),
             ((4, 3), 1), ((4, 4), 1));
    adj_map.insert((3, 2), adj);

    // up mid 8
    let mut adj =
        vec!(((1, 1), 0), ((2, 0), 0), ((3, 1), 0),
             ((0, 0), 1), ((1, 0), 1), ((2, 0), 1),
             ((3, 0), 1), ((4, 0), 1));
    adj_map.insert((2, 1), adj);

    // down mid 18
    let mut adj =
        vec!(((1, 3), 0), ((2, 4), 0), ((3, 3), 0),
             ((0, 4), 1), ((1, 4), 1), ((2, 4), 1),
             ((3, 4), 1), ((4, 4), 1));
    adj_map.insert((2, 3), adj);

    // map outer
    // left mid 11
    let mut adj =
        vec!(((0, 1), 0), ((1, 2), 0), ((0, 3), 0),
             ((1, 2), -1));
    adj_map.insert((0, 2), adj);

    // right mid 15
    let mut adj =
        vec!(((4, 1), 0), ((4, 3), 0), ((3, 2), 0),
             ((3, 2), -1));
    adj_map.insert((4, 2), adj);

    // top mid 3
    let mut adj =
        vec!(((2, 1), 0), ((1, 0), 0), ((3, 0), 0),
             ((2, 1), -1));
    adj_map.insert((2, 0), adj);

    // bottom mid 23
    let mut adj =
        vec!(((1, 4), 0), ((3, 4), 0), ((2, 3), 0),
             ((2, 3), -1));
    adj_map.insert((2, 4), adj);

    // map corners
    // top left corner 1
    let mut adj =
        vec!(((0, 1), 0), ((1, 0), 0),
             ((1, 2), -1), ((2, 1), -1));
    adj_map.insert((0, 0), adj);

    // bottom left corner 21
    let mut adj =
        vec!(((0, 3), 0), ((1, 3), 0),
             ((2, 3), -1), ((1, 2), -1));
    adj_map.insert((0, 4), adj);

    // top right corner 5
    let mut adj =
        vec!(((3, 0), 0), ((4, 1), 0),
             ((2, 1), -1), ((3, 2), -1));
    adj_map.insert((4, 0), adj);

    // bottom right corner 25
    let mut adj =
        vec!(((4, 3), 0), ((3, 4), 0),
             ((2, 3), -1), ((3, 2), -1));
    adj_map.insert((4, 4), adj);

    // map next to corners
    // top left top 2
    let mut adj =
        vec!(((0, 0), 0), ((2, 0), 0),  ((1, 1), 0),
             ((2, 1), -1));
    adj_map.insert((1, 0), adj);

    // top left side 6
    let mut adj =
        vec!(((0, 0), 0), ((0, 2), 0),  ((1, 1), 0),
             ((1, 2), -1));
    adj_map.insert((0, 1), adj);

    // top right side 10
    let mut adj =
        vec!(((4, 0), 0), ((3, 1), 0),  ((4, 2), 0),
             ((3, 2), -1));
    adj_map.insert((4, 1), adj);

    // top right top 4
    let mut adj =
        vec!(((2, 0), 0), ((3, 1), 0),  ((4, 0), 0),
             ((2, 1), -1));
    adj_map.insert((3, 0), adj);

    // bottom left side 16
    let mut adj =
        vec!(((0, 2), 0), ((1, 3), 0),  ((0, 4), 0),
             ((1, 2), -1));
    adj_map.insert((0, 3), adj);

    // bottom left bottom 22
    let mut adj =
        vec!(((0, 4), 0), ((2, 4), 0),  ((1, 3), 0),
             ((2, 3), -1));
    adj_map.insert((1, 4), adj);

    // bottom right side 20
    let mut adj =
        vec!(((3, 3), 0), ((4, 2), 0),  ((4, 4), 0),
             ((3, 2), -1));
    adj_map.insert((4, 3), adj);

    // bottom right bottom 24
    let mut adj =
        vec!(((2, 4), 0), ((4, 4), 0),  ((3, 3), 0),
             ((2, 3), -1));
    adj_map.insert((3, 4), adj);

    let mut maps = empty_maps();
    let start = 201;
    maps[start] = map;

    let total_bugs: u32 = maps.iter().map(|map| map.num_bugs()).sum();
    println!("Total bugs = {}", total_bugs);

    for ix in 0..1 {
    //for ix in 0..200 {
        println!("Running step {}", ix);
        maps = step(&maps, &adj_map);

        let total_bugs: u32 = maps.iter().map(|map| map.num_bugs()).sum();
        println!("Total bugs = {}", total_bugs);
    }

    maps[start - 5].print();
    println!("____");
    maps[start - 4].print();
    println!("____");
    maps[start - 3].print();
    println!("____");
    maps[start - 2].print();
    println!("____");
    maps[start - 1].print();
    println!("____");
    maps[start + 0].print();
    println!("____");
    maps[start + 1].print();
    println!("____");
    maps[start + 2].print();
    println!("____");
    maps[start + 3].print();
    println!("____");
    maps[start + 4].print();
    println!("____");
    maps[start + 5].print();
    println!("____");

    let total_bugs: u32 = maps.iter().map(|map| map.num_bugs()).sum();
    println!("Total bugs = {}", total_bugs);
}
