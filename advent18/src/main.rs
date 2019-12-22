use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::collections::binary_heap::BinaryHeap;
use std::hash::{Hash, Hasher};

use pathfinding::directed::astar::astar;
use rayon::prelude::*;


const PRINT: bool = false;

const IX: usize = 3;

const INPUT: [&str; 6] = [
"#########
#b.A.@.a#
#########",
"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
"#################################################################################
#...#...............#...........#.......#.#...........#..z......#...K........r..#
###.#.#############.#.#.#######.#.###.#.#.#.#####.###.###.#######.#.#############
#...#.#.....#...#...#.#.#.......#...#.#.#.#.....#.#.....C.#.......#.#.....#.....#
#.###.#.###.#.#.#.###.#.###########.#.###.#####.#.#####.###.#######.#.###.#.###.#
#...#.#.#.#b#.#...#...#...........#.#...#.....#.#.....#.#...#.....#.....#...#...#
###.#.#.#.#.#.#####.###########.###.###.#T#####.#####.###.###P###.#########.#.#R#
#...#.#...#.#f....#.....#...#...#...#.#.#.#...#.Y.#...#.....#.#.#..p#a....#.#.#.#
#.#.#.#####.#####.#####.#.#.#.###.###.#.#.#.#.###.#.###.#####.#.###.#.###.#.#.#.#
#.#.#.....#.#.....#...#...#.#.....#...#.#.#.#.#...#.....#.....#...#.#.#.#.#.#.#.#
#.#######.#.#.#######.#####.#######.###.#.#.#.#.#########.#######.#.#.#G#.###.#.#
#..q#.......#.......#.....#.#.......#.U.#.#.#...#....n..#v..#.L...#.#.#.....#.#.#
#.#.#.#######.#####.#.###.#.#.#####.#.#.#.#.###########.###.###.#.#.#.#####.#.#.#
#.#...#.....#...#...#.#...#.#.#.....#.#.#...#...........#.#...#.#.#.#.....#...#.#
#.#####.###.#####.#####.#.#.#.#.#####.#.#.#####.#.#####.#.###.#.###.#.###.#####.#
#.....#.#...#.....#.....#.#...#...#...#.#.#...#.#...#.#.#.#...#.#...#.#.#...#..g#
###X###.#.###.#####.#############.#.###.#.#.#.#####.#.#.#.#.###.#.###.#.###.#####
#...#...#.....#...#...........D...#.#.#.#...#.#.....#...#.#.#.....#...A...#.....#
#.###.#########.#.#.###########.###.#.#.#####.#.#####.###.#.###.#########.#####.#
#...#.#.....#...#.....#...#...#.#...#...#.....#.#...#.#...#...#l#...#.#j..#...#.#
#.###.#.#####.#########.#.#.#.#.#.#####.#.#####.###.#.###.###V#.#.#.#.#.###.#J#.#
#.#...#.....#.#.....#...#...#.#.#...S.#.#...#.....#.#.....#.#.#...#.#.....#.#...#
#.#.###.###.#.#.###.#########.#.#####.#.###.#####.#.#####.#.#######H#######.#####
#.#...#...#.#.#...#...#...#...#.....#.#.#...#.....#.....#.#...#...#........h#...#
#.###I###.#.#.#.###.#.#.#.#.#########.###.###.#######.#.#.#.#.#.#.#############B#
#...#...#.#.#.#.#...#.#.#.#.#.......#...#.....#.......#.#.#.#...#...#...#.......#
###.###.#.#M#.#.#.###.###.#.#.#####.###.#######.#######.#.#.#######.#.#.###.###.#
#.#.#.#.#.#...#.#...#.....#...#.#...#...#...#.....#...#.#...#.....#...#.....#.#.#
#.#.#.#.#.#####.###.###########.#.###.###.#.#.###.###.#.#####.#.#############.#.#
#.#...#.#.....#.#.#...#...........#.#...#.#.#...#.#...#.#...#.#...............#.#
#.###.#.#####.#.#.###.#.###########.#.#.#.#.###.#.#.###.#.###.#########.#####.#.#
#...#.#.#.#...#.#...#.#.#.....#...#...#.#.#...#.#.#.....#.#...#.....#.#...#...#.#
###.#.#.#.#.###.#.###.#.#.###.###.#.###.#####.#.#.#.#####.#.#####.#.#.###.#####.#
#...#.#.#...#...#...#...#.#.#...#.....#.#.....#.#.#.......#.......#.#...#.......#
#.#O#.#.#####.#####.#####.#.###.#.#####.#.###.#.#.#################.###.#########
#.#.#.#...#...#...........#.....#.#...#.#.#...#.#...#.......#.......#.....#.....#
#.#.#.###.#.###.###########.#######.#.#.#.###.#.###.#.###.###.#######.###.###.#.#
#.#.#.#.#...#...#.#.......#...#.....#.#.#...#.#...#.#.#.#.#...#.........#.#...#.#
#.###.#.#######.#.#.#.#######.#.#####.#.#.#.#####.#.#.#.#.#.###########.#.#.###.#
#.........E.....#...#...........#.........#.......#....e#...............#...#...#
#######################################.@.#######################################
#.....#...#.........#...............#.........#...#.............#...#...#...#...#
#.###.#.#.#######.#.###.###########.###.#.#####.#.#.#.#########.#.#.#.#.#.#.###.#
#.#.#...#.....#...#...#...#.....#.#.#...#.......#...#.....#.#...#.#...#.#.#.....#
#.#.#########.#.#####.#.#.#.###.#.#.#.###.###############.#.#.###.#####.#.#####.#
#.#.#...#...#...#.#...#.#.#...#...#.#...#.#.......#...#...#.#.....#.#...#.#x#...#
#.#.#.#.#.#.#####.#.#####.#.#.###.#.###.#.#####.#.#.#.#.###.#######.#.###.#.#.###
#.#...#...#.#...#...#.....#.#...#.#.....#...#...#.#.#.#...#.........#....o#...#.#
#.#####.###.#.#.#.###.#####.###.#######.###.#.###.#.#####.###.#############.###.#
#.#...#.#.#.#.#...#.#.....#.#...#.....#s#...#.#.#.#.....#...#...........#.#.....#
#.#.#.#.#.#.#.#####.#.###.#.#.#.#.###.#.#.###.#.#.###.#.###.#########.#.#.#####.#
#...#.#.#.#...#...#...#...#.#.#.#...#.#.#.....#.......#.#.#.#.........#.......#.#
#.###.#.#.#####.#.#####.#####.#.###.#.#.###.###########.#.#.#.#########.#######.#
#.#.#.#.#.......#.......#.....#.#...#...#...#...#.....#...#.#.#..m......#.....#.#
#.#.#.#.###.#############.#######.#######.###.#.#.###.#.###.#.#.#####.###.###.#.#
#...#.#...#.....#.....#.....#...#.#.....#...#.#.#...#...#...#.#.#...#...#...#...#
###.#.###.#.###.#####.#.###.#.#.#.#.###.#.###.#.#########.###.###.#.#######.#####
#...#.#.#.#...#.....#.#.#.#...#...#...#.#.#...#.......#...#.......#.......#.Q...#
#.###.#.#.#.#######.#.#.#.#############.#.#.#########.#.#########.#######.#####.#
#...#.#...#.#.......#.......#...........#.#.#.......#...#.......#.......#...#...#
###.#.#.###.#.#####.#######.#.#.#######.#.#.#.#####.#####.#.###.#######.###.#.###
#...#.#.#.#.#.#.....#.#...#.#.#.......#.#.#.#.....#.#...#.#.#.#.#.....#...#...#.#
#####.#.#.#.#.###.###.#.#.#.#.#######.###.#.#.#####.#.#.#.###.#.#.#.#.###.#####.#
#...W.#.#...#...#.#...#.#.#.#.......#...#u#...#.....#.#.#.....#.#.#.#.#.#.#.....#
#.#####.###.###.###.#####.#.#######.###.#######.###.#.#.#######.###.#.#.#.#.#####
#.#...#...#.#.#.#...#.....#.#.........#.#...#...#...#.#...#...#.#...#...#.#w....#
#.#.#.###.#.#.#.#.###.#.###.#.#########.#.#.#.#.#####.###.#.#.#.#.#######.###.#.#
#.#.#.#.#.#...#.#.#...#.#...#...#.#.....#.#...#.#...#.#.#...#...#...#...#...#.#.#
#.#.#.#.#.#####.#.#.#####.#####.#.#.###.#.#######.#.#.#.#########.#.#.#.###.###.#
#.#.#.#.........#.#.....#.#...#...#.#...#.#.....#.#.#.....#.....#.#...#...#...#.#
#.#.#.###.#######.#####.#.#.#.###.#.###.#.#.###.#.#.#####.#####.#.#######.###F#.#
#t..#...#.#.......#.....#...#.#...#...#.#.#.#.#.#.#.....#.#.....#...#...#.#...#.#
#.#####.###.#######.#########.#######.#.#.#.#.#.#.###.#.#.#.#######.#N#.#.#.###.#
#.....#...#.#...............#.#.......#.#.#.#.#...#.#.#.#.#.......#...#.#...#...#
#########.#.#####.#####.###.#.#.#######.#.#.#.#####.#.###.#.#####.#####.#####.#.#
#........y#.....#.#...#.#...#.#.#.......#...#.......#.....#.#.....#...#....i..#.#
#.#############.#.#.#.#.###.#.#.#.###########.###.#########.#.#####.#.#########.#
#...#.......#...#.#.#.#...#.#.#.#.#.....#..k#...#.....#c..#.#.....#.#...#.....#.#
#.#.#.#####.#.###.#.#.###.###.#.#.###.#.#.#####.#####.#.#.#.#####.#.###.#.###.#.#
#.#.......#...#.....#...#.......#..d..#.#.......Z...#...#.......#.....#.....#...#
#################################################################################"
];


pub type Loc = (usize, usize);

pub type Cost = usize;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    pub fn move_dir(&self, loc: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::North => (loc.0, loc.1 - 1),
            Dir::East => (loc.0 + 1, loc.1),
            Dir::South => (loc.0, loc.1 + 1),
            Dir::West => (loc.0 - 1, loc.1),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Tile {
    Empty,
    Wall,
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Solution {
    steps: usize,
    loc: Loc,
    collected: Vec<char>,
    keys: Vec<(Loc, char)>,
    doors: Vec<(Loc, char)>,
    goal: Option<Loc>,
}

impl Hash for Solution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
        self.collected.hash(state);
        self.keys.hash(state);
        self.doors.hash(state);
        self.goal.hash(state);
    }
}

impl Solution {
    pub fn hashed(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return hasher.finish();
    }

    pub fn key_at_loc(&self, loc: Loc) -> Option<char> {
        if let Some(key_ix) = self.keys.iter().position(|(key_loc, ch)| *key_loc == loc) {
            return Some(self.keys[key_ix].1);
        }

        return None;
    }

    pub fn pick_up_key(&mut self, key_ch: char) {
        if let Some(key_ix) = self.keys.iter().position(|(key_loc, ch)| *ch == key_ch) {
            self.keys.swap_remove(key_ix);
            self.collected.push(key_ch);
        } else {
            panic!("Attempted to pick up key that doesn't exist!");
        }
    }

    pub fn door_at_loc(&self, loc: Loc) -> Option<char> {
        if let Some(door_ix) = self.doors.iter().position(|(door_loc, ch)| *door_loc == loc) {
            return Some(self.doors[door_ix].1);
        }

        return None;
    }

    pub fn open_door(&mut self, door_ch: char) {
        if let Some(door_ix) = self.doors.iter().position(|(door_loc, ch)| *ch == door_ch) {
            self.doors.swap_remove(door_ix);

            if let Some(door_pos) = self.collected.iter().position(|door_ch| *door_ch == door_ch.to_lowercase().next().unwrap()) {
                self.collected.swap_remove(door_pos);
            } else {
            panic!("Attempted to open door without the door!");
            }
        } else {
            panic!("Attempted to open door that doesn't exist!");
        }
    }

    pub fn path_no_doors(&self, map: &Map, start: Loc, end: Loc) -> Option<(Vec<Loc>, Cost)> {
        let path =
            astar(&start, 
                  |p| {
                      let mut sucs = Vec::new();
                      if map.tiles[p.1 - 1][p.0] == Tile::Empty &&
                         self.door_at_loc((p.0, p.1 - 1)).is_none() {
                          sucs.push(((p.0, p.1 - 1), 1));
                      }
                      if map.tiles[p.1 + 1][p.0] == Tile::Empty &&
                         self.door_at_loc((p.0, p.1 + 1)).is_none() {
                          sucs.push(((p.0, p.1 + 1), 1));
                      }
                      if map.tiles[p.1][p.0 + 1] == Tile::Empty &&
                         self.door_at_loc((p.0 + 1, p.1)).is_none() {
                          sucs.push(((p.0 + 1, p.1), 1));
                      }
                      if map.tiles[p.1][p.0 - 1] == Tile::Empty &&
                         self.door_at_loc((p.0 - 1, p.1)).is_none() {
                          sucs.push(((p.0 - 1, p.1), 1));
                      }

                      return sucs;
                  },
                  |p| {
                      ((p.0 as i32 - end.0 as i32).pow(2) + 
                      (p.1 as i32 - end.1 as i32).pow(2)) as usize

                  },
                  |p| {
                      *p == end
                  });

        return path;
    }

    pub fn path_to(&self, map: &Map, start: Loc, end: Loc) -> Option<(Vec<Loc>, Cost)> {
        let path =
            astar(&start, 
                  |p| {
                      let mut sucs = Vec::new();
                      if map.tiles[p.1 - 1][p.0] == Tile::Empty {
                          sucs.push(((p.0, p.1 - 1), 1));
                      }
                      if map.tiles[p.1 + 1][p.0] == Tile::Empty {
                          sucs.push(((p.0, p.1 + 1), 1));
                      }
                      if map.tiles[p.1][p.0 + 1] == Tile::Empty {
                          sucs.push(((p.0 + 1, p.1), 1));
                      }
                      if map.tiles[p.1][p.0 - 1] == Tile::Empty {
                          sucs.push(((p.0 - 1, p.1), 1));
                      }

                      return sucs;
                  },
                  |p| {
                      ((p.0 as i32 - end.0 as i32).pow(2) + 
                      (p.1 as i32 - end.1 as i32).pow(2)) as usize

                  },
                  |p| {
                      *p == end
                  });

        return path;
    }

    pub fn reach_goal(&mut self, map: &Map) -> Cost {
        if let Some(goal) = self.goal {

            let path = self.path_to(map, self.loc, goal);

            if let Some((points, cost)) = path {
                //println!("loc = {:?}, goal = {:?}", self.loc, goal);
                //println!("{:?}", &points);
                //println!("cost = {}", cost);
                //println!("old steps = {}", self.steps);
                self.steps += cost;
                //println!("new steps = {}", self.steps);

                self.loc = goal;
                
                if let Some(key_ix) = self.keys.iter().position(|(loc,ch)| *loc == self.loc) {
                    // remove the key from the solutions key list
                    let (_, ch) = self.keys.swap_remove(key_ix);

                    // add the key to the collected key list
                    self.collected.push(ch);
                } /* else if let Some(door_ix) = self.doors.iter().position(|(loc,ch)| *loc == self.loc) {
                    // remove the door- it is now unlocked by the key
                    let (door_loc, door_ch) = self.doors.swap_remove(door_ix);

                    let collected_ix = self.collected.iter().position(|ch| {
                        return door_ch.to_lowercase().next().unwrap() == *ch;
                    });

                    if let Some(collected_ix) = collected_ix {
                        // remove collected key, using it on the door
                        self.collected.swap_remove(collected_ix);
                    } else {
                        // goal was a door we didn't have the key too?
                        panic!(format!("Attempted to open door ({}) without key!", door_ch));
                    }
                }
                  */
                return cost;
            } else {
                panic!("no path to goal!");
            }
        } else {
            panic!("Asked to reach goal when no goal is present!");
        }
    }

    pub fn generate_goals(&self, paths: &HashMap<(Loc, Loc), Vec<Loc>>, map: &Map) -> Vec<Loc> {
        let mut new_goals = Vec::new();
        
        for (key_loc, key_ch) in self.keys.iter() {
            let mut hit_door = false;
            let mut hit_key = false;
            if let Some(path) = paths.get(&(self.loc, *key_loc)) {
                let up_to = path.len();
                for path_loc in path.iter().take(up_to) {
                    if let Some(door_ch) = self.door_at_loc(*path_loc) {
                        if !self.collected.iter().any(|col_key| 
                            *col_key == door_ch.to_lowercase().next().unwrap()) {
                            //let col_ch = self.collected[col_ix];
                            //println!("Adding door {}, with key {}", door_ch, col_ch);;
                            //new_goals.push(*path_loc);
                            hit_door = true;
                            break;
                        }
                    } else if let Some(other_key_ch) = self.key_at_loc(*path_loc) {
                        if *key_ch != other_key_ch {
                            hit_key = true;
                            break;
                        }
                    }
                }
            } else {
                panic!(format!("No path? {:?} {:?}", self.loc, key_loc));
            }

            if !hit_key && (!hit_door || self.path_no_doors(map, self.loc, *key_loc).is_some()) {
                new_goals.push(*key_loc);
            }
        }

        return new_goals;
    }

    pub fn print(&self, map: &Map) {
        println!("SOLUTION:");
        for y in 0..map.tiles.len() {
            for x in 0..map.tiles[0].len() {
                if let Some(key_ch) = self.key_at_loc((x, y)) {
                    print!("{}", key_ch);
                } else if let Some(door_ch) = self.door_at_loc((x, y)) {
                    print!("{}", door_ch);
                } else if self.loc == (x, y) {
                    print!("@");
                } else {
                    match map.tiles[y][x] {
                        Tile::Empty => {
                            print!(" ");
                        },

                        Tile::Wall => {
                            print!("#");
                        }
                    }
                }
            }

            println!("");
        }

        println!("Loc {:?}", self.loc);
        println!("Collected keys {:?}", self.collected);
        println!("Remaining keys {:?}", self.keys);
        println!("Remaining doors {:?}", self.doors);
        println!("Current steps {}", self.steps);
        println!("Current goal {:?}", self.goal);
        println!("END SOLUTION:");
    }
}

pub fn distance(loc0: Loc, loc1: Loc) -> usize {
    ((loc0.0 as i32 - loc0.0 as i32).abs() +
    (loc1.1 as i32 - loc1.1 as i32).abs()) as usize
    //self.paths.get(&(*loc, *other_loc)).unwrap().len();
}

pub struct Solver {
    //seen: HashMap<u64, usize>,
    //solutions: HashMap<Cost, Vec<Solution>>,
    initial: Solution,
    paths: HashMap<(Loc, Loc), Vec<Loc>>,
}

impl Solver {
    pub fn new(initial_solution: Solution, map: &Map) -> Solver {
        //let mut seen = HashMap::new();
        //let mut solutions = HashMap::new();
        let mut paths = HashMap::new();

        for (key_loc, _) in initial_solution.keys.iter() {
            for (other_key_loc, _) in initial_solution.keys.iter() {
                if key_loc != other_key_loc {
                    if let Some((path, _)) = initial_solution.path_to(map, *key_loc, *other_key_loc) {
                        paths.insert((*key_loc, *other_key_loc), path);
                    }
                }
            }
            if let Some((path, _)) = initial_solution.path_to(map, initial_solution.loc, *key_loc) {
                paths.insert((initial_solution.loc, *key_loc), path);
            }
            if let Some((path, _)) = initial_solution.path_to(map, *key_loc, initial_solution.loc) {
                paths.insert((*key_loc, initial_solution.loc), path);
            }
        }

        //seen.insert(initial_solution.hashed(), 0);
        //solutions.insert(0, vec!(initial_solution));

        return Solver {
            //seen,
            //solutions,
            initial: initial_solution,
            paths,
        };
    }

    pub fn solve(&mut self, map: &Map) -> Cost {

        let path =
            astar(&self.initial, 
                  |solution| {
                      //let mut sucs = Vec::new();

                      let new_goals = solution.generate_goals(&self.paths, map);

                      let sucs = new_goals.into_par_iter().map(|new_goal| {
                      //let sucs = new_goals.into_iter().map(|new_goal| {
                          let mut new_solution = solution.clone();
                          new_solution.goal = Some(new_goal);

                          let cost = new_solution.reach_goal(map);

                          return (new_solution, cost);
                      })
                      .collect::<Vec<(Solution, Cost)>>();

                      return sucs;
                  },
                  |solution| {
                      let mut heuristics = Vec::new();

                      /* loc tree heuristic */
                      let mut loc_tree_cost = 0;
                      let mut loc_paths: HashSet<(Loc, Loc)> = HashSet::new();
                      let mut locs = solution.keys.iter().map(|key| key.0).collect::<Vec<Loc>>();
                      //locs.push(solution.loc);
                      for loc in locs.iter() {
                          let mut loc_path_cost = None;
                          let mut shortest_pair = None;

                          for other_loc in locs.iter() {
                              if *loc != *other_loc {
                                 //if loc_paths.contains(&(*loc, *other_loc)) ||
                                 //   loc_paths.contains(&(*other_loc, *loc)) {
                                 //     loc_path_cost = None;
                                 //     shortest_pair = None;
                                 //     break;
                                 //}

                                  //println!("{:?} -> {:?}", loc, other_loc);
                                 let path_cost = 
                                     //distance(*loc, *other_loc);
                                     //solution.path_to(map, *loc, *other_loc).unwrap().1;
                                     self.paths.get(&(*loc, *other_loc)).unwrap().len();
                                 if let Some(cost) = loc_path_cost {
                                     if path_cost < cost {
                                         loc_path_cost = Some(cost);
                                         shortest_pair = Some((*loc, *other_loc))
                                     }
                                 } else {
                                     loc_path_cost = Some(path_cost);
                                     shortest_pair = Some((*loc, *other_loc));
                                 }
                              }
                          }

                          if let Some(pair) = shortest_pair {
                             if !loc_paths.contains(&pair) && !loc_paths.contains(&(pair.1, pair.0)) {
                                 loc_paths.insert(pair);
                             }
                          }

                          if let Some(cost) = loc_path_cost {
                              loc_tree_cost += cost;
                          }
                      }
                      println!("loc_paths.len() = {:3} of {:3}", loc_paths.len(), solution.keys.len());
                      let mut loc_tree_cost =
                          loc_paths.iter().map(|(loc0, loc1)| {
                              //distance(*loc0, *loc1)
                              self.paths.get(&(*loc0, *loc1)).unwrap().len()
                              //solution.path_to(map, *loc0, *loc1).unwrap().1
                          }).sum();

                      let mut min_dist = 100000000;
                      for (key_loc, _) in solution.keys.iter() {
                          min_dist =
                              std::cmp::min(min_dist,
                                            self.paths.get(&(solution.loc, *key_loc)).unwrap().len());

                      }
                      loc_tree_cost += min_dist;
                      //println!("loc tree cost {:4}", loc_tree_cost);
                      heuristics.push(loc_tree_cost);

                      let heuristic = *heuristics.iter().max().unwrap();

                      return heuristic;
                  },
                  |solution| {
                      solution.keys.len() == 0
                  });

        let path_clone = path.clone().unwrap();
        let num_sols = path_clone.0.len();
        dbg!(&path_clone.0[num_sols - 1].collected);

        return path.unwrap().1;
    }
}

pub fn parse_input(input: &str) -> (Map, Vec<(Loc, char)>, Vec<(Loc, char)>, Loc) {
    let mut tiles = Vec::new();
    let mut keys = Vec::new();
    let mut doors = Vec::new();

    let mut x = 0;
    let mut y = 0;

    let mut loc = (0, 0);

    for line in input.split("\n") {
        let mut row = Vec::new();
        x = 0;
        for ch in line.chars() {
            match ch {
                '#' => {
                    row.push(Tile::Wall);
                }

                '.' => {
                    row.push(Tile::Empty);
                }

                '@' => {
                    row.push(Tile::Empty);
                    loc = (x, y);
                }

                'a'..='z' => {
                    keys.push(((x, y), ch));
                    row.push(Tile::Empty);
                },

                'A'..='Z' => {
                    row.push(Tile::Empty);
                    doors.push(((x, y), ch));
                }
                
                _ => {
                    panic!(format!("Unexpected tile {}", ch));
                }
            }

            x += 1;
        }

        tiles.push(row);

        y += 1;
    }

    return (Map { tiles, }, keys, doors, loc);
}

fn main() {
    let (map, keys, doors, loc) = parse_input(INPUT[IX]);

    let initial_solution = 
        Solution {
            steps: 0,
            loc,
            collected: Vec::new(),
            keys,
            doors,
            goal: None,
    };

    let mut solver = Solver::new(initial_solution, &map);

    println!("Min steps: {}", solver.solve(&map));
}
