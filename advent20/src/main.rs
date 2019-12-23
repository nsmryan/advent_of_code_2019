use std::collections::HashMap;

use pathfinding::directed::astar::astar;


const IX: usize = 2;

const INPUT: [&str;3] = [
"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ",
"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ",


"                                     S         N       I         Y       A     F                                       
                                     D         C       J         J       C     W                                       
  ###################################.#########.#######.#########.#######.#####.#####################################  
  #.......#.#...#.....#...#.....#.........#...#.....#.#.......#...#...........#.....#.................#.#...#.#.#.#.#  
  ###.###.#.###.#####.#.#.#####.###.#####.#.#.#.#.###.#.#########.###.###.#######.###.#####.###.###.###.###.#.#.#.#.#  
  #...#.#...#.#...#.#...#.#.........#.....#.#.#.#...#.....#.#.....#.#...#.#.#.#.#.......#.#.#.#.#.#.#.#.#...#.#.....#  
  #.###.#.###.###.#.###.###.###.#####.#####.#.###.#####.#.#.#####.#.###.###.#.#.#.#.#####.###.###.###.#.###.#.#.#####  
  #.#...#.....................#.#.#.......#.#.........#.#.#.......#.....#.......#.#...#.#...#.#.#.#.......#.....#.#.#  
  #####.###.#.###.###.#.#.#.#.###.#######.#.#############.#.###.###.#.#.#####.#.#.#####.#.###.#.#.#.###.#.#####.#.#.#  
  #.........#.#...#...#.#.#.#.#.#.#.......#.#.......#.....#.#.#.#...#.#.#.#...#...#.#.....#...#.....#.#.#.#.....#...#  
  #.#.#######.#############.###.#.###.#.#.#.#.###.#.#.#.#.#.#.#####.#####.###.#####.#.#####.#####.###.#########.#.#.#  
  #.#.#...#...#.#.#...........#.......#.#.#.#.#.#.#.#.#.#.#.#...#.....#.....#.....#.....#...#.....#...#...........#.#  
  ###.###.###.#.#.#######.#####.###.#######.#.#.#.#####.###.#.###.###.###.#######.#.#.#####.###.#.#.#.#######.#######  
  #.#...#.#...#.#...#.#.......#.#...#.#.#.....#.#.....#...#.....#.#...#.#.....#.#...#...........#.#.#.#...#.......#.#  
  #.#####.#####.#.#.#.###.#########.#.#.#.###.###.#######.#.#.#######.#.###.###.#.###.#.#.###.###.#.###.#####.#.###.#  
  #...#...........#...#.#.#.#.#.#.....#.#...#.#.....#.#.#.#.#.#.........#.........#...#.#...#...#.#...#.#.....#.#...#  
  #.#######.#####.###.#.###.#.#.#####.#.#.#######.#.#.#.#.###.###.#.###.#.#.#.#.#.###.#######.#####.###.#######.#.#.#  
  #.....#...#.#...#.............#.#.....#.#...#...#.#.....#.#...#.#.#.#.#.#.#.#.#...#...#.............#.#.#...#...#.#  
  #.#########.#####.#######.###.#.#.#########.###.###.###.#.#.#.###.#.#####.#######.###.###.#.#.#.#####.#.#.###.#####  
  #.#.#.#.#...#.#.#.#.......#.#.....#.....#.........#.#...#...#...#...#.#.......#.#.#.#.#...#.#.#.#.#.#.#...#.......#  
  #.#.#.#.###.#.#.#####.#.#.#.#####.#.#.#######.#.###.###.#.#.#.###.###.###.#####.###.#.###########.#.#.#.#.###.###.#  
  #.#...#.....#...#.#...#.#...#.#...#.#.....#...#...#.#...#.#.#.#.#...#.#.#.#.#...#...#.#.....#.#.......#.#...#.#...#  
  #.###.#.#######.#.#######.###.###.#.###.#######.#####.###.#.#.#.#.###.#.#.#.#.###.#######.###.#.#####.###.###.#####  
  #...........#.......#.#.#.#.........#.#...#.#.....#.....#.#.#.#...#.#.#...#...#...#...#.#...#.#.#...#...#...#.....#  
  ###.###.###########.#.#.###.###.#.###.#.###.###.#######.#.#######.#.#.###.#.#.#.#.#.###.#.###.#####.#.###.###.#.###  
  #.....#...#.#.#.......#.#.#.#...#...#...#...#.#.#.......#.#.......#.#.#.....#.#.#.#...#.....#...#...#...#...#.#.#.#  
  ###.#######.#.###.#####.#.#####.#.###.#####.#.#.#.#.#####.###.#.#.#.#.###.###.#.###.###.#.#.#.#####.###.#.###.###.#  
  #.#.....#.#.#...#...#...#.......#.#.#.....#.#...#.#...#.....#.#.#.......#...#.....#.....#.#...#.#...#.#.#.#.#.#.#.#  
  #.#.###.#.#.###.#.###.#########.###.###.###.#.#.#.#.#.###.#.###.#######.#######.###.#.#.#######.#.###.#.#.#.#.#.#.#  
  #.#...#.#...#.#...#.#...#.#...#...#.......#...#.#.#.#.#...#...#.#.......#...........#.#.#...#...#.#.#.....#.......#  
  #.#.#######.#.#.#.#.#.###.#.#######.###########.#.#######.#######.###########.#############.###.#.#.###.###.###.###  
  #...#...#.....#.#.#...#.#.#.#      S           N P       R       Y           M        #.#.#.#...#...#.#...#.#.....#  
  ###.#.#####.#.###.###.#.#.#.#      J           D S       A       J           F        #.#.#.#.###.###.#.#######.###  
  #.#.#.#.#.#.#...........#.#.#                                                         #.......#.........#.........#  
  #.#.#.#.#.#####.###.#####.#.#                                                         #.#####.#.###.###.#.#####.###  
  #...#...#...#.#.#.#.#.#.#...#                                                         #.....#.....#.#.#.....#......KQ
  ###.###.#.###.#.#.#.#.#.#.###                                                         #####.###.#####.#.###########  
  #.#.....#.....#.#.....#.....#                                                         #.......#.#.#.#.#...#.......#  
  #.###.#######.#####.#.#.#.###                                                         #.#####.###.#.#.#.#.#######.#  
  #.#...#.#...#.....#.#.#.#...#                                                       SC..#...#.#.#.#.#.#.#.#.#.#.#.#  
  #.###.#.#.#####.###.#.#.#.###                                                         ###.#####.#.#.#.#####.#.#.#.#  
ZB....#.#...#.........#...#....UX                                                       #.....#...........#.......#..SJ
  #.###.###.#####.###.#.#######                                                         #.#.#.#.#.#####.###.###.#.#.#  
  #.................#.#.#.#...#                                                       UI..#.#...#.#...#.#.#.#.#.#...#  
  #.#.###.#####.###.#.###.#.###                                                         ###.#.#######.#.#.#.#.#####.#  
  #.#.#.#.#.#.#...#.#.#.#.....#                                                         #.#.#...#.........#.#...#...#  
  #.#.#.###.#.#########.#.###.#                                                         #.#.#######.#####.#.###.#####  
  #.#.#.#.#.#...#.......#.#...#                                                         #.#.#.....#.#.#.......#...#.#  
  #####.#.#.#.#.#######.#.#.###                                                         #.#####.#####.#########.###.#  
  #.#.#...#...#...#.#.....#...#                                                         #...#.....#...............#..RA
  #.#.#.###.#####.#.###.#.#.#.#                                                         #.#.#.#.###.#############.#.#  
JA..............#.......#.#.#..SD                                                     NC..#...#.#.....#.....#...#.#.#  
  #####.###.#.###.#############                                                         #.###.#.###.#####.#####.#.#.#  
UJ....#.#...#.#...#...#.#......JL                                                       #...#.#.#.............#.....#  
  #.#####.#######.#.#.#.#.###.#                                                         #######.#.#.###.#############  
  #.....#.#.....#.#.#...#...#..FW                                                       #.........#.#.#.#.#.#...#...#  
  #.#########.#####.###.#.#####                                                         #.###########.###.#.###.###.#  
YM....#...#.#.........#.......#                                                         #.#...........#.....#........ND
  #.#.#.#.#.#.#####.###.#######                                                         ###.#########.#.###.#.#.#####  
  #.#...#.......#.....#.....#.#                                                         #.#.........#...#.#...#.#.#.#  
  #######.###.###.###########.#                                                         #.#########.#####.#.#.###.#.#  
  #.....#...#.#.#.#.....#.#...#                                                       YT..........#.#...#...#...#...#  
  #.###.#######.###.#####.###.#                                                         ###.###.###.#.###.#########.#  
  #...#...#...#.#.....#.#......XQ                                                       #.#.#.........#...#...#...#.#  
  #.###.###.#.#.#.#.#.#.#.#.#.#                                                         #.#.#.#######.#####.###.###.#  
  #.#...#...#...#.#.#.#.#.#.#.#                                                       OY..#.#.#...#.#.#.#.....#.#...#  
  #.#.###.#.###.###.#.#.#####.#                                                         #.#####.###.###.###.#.#.#.#.#  
MF..#.....#.#.......#.........#                                                         #...#...............#.....#..YT
  #.###.#.#####.#.###########.#                                                         #.#####.#####.#######.###.#.#  
  #...#.#.#.#.#.#.#.#.......#.#                                                         #...#...#.#...#.#.......#.#.#  
  #.#######.#.#####.###.###.###                                                         #.#.#.###.###.#.#.###.#.#####  
  #.#.....#.....#...#...#.....#                                                         #.#.......#.....#.#...#.#...#  
  #####.###.#####.###.#.#.###.#                                                         ###.###.#####.#############.#  
LE......#.#.#.#.#...#.#.#.#...#                                                         #.#...#.#.#.#...#...#.....#..XQ
  #.#.#.#.#.#.#.###.#.#######.#                                                         #.#######.#.###.###.###.#.#.#  
  #.#.#...............#.#...#..YM                                                     ZB....#...#.#...#.#.....#.#...#  
  #####################.#.#####                                                         #.#####.#.###.#####.###.###.#  
  #.....#...............#.....#                                                         #.#...#...#.......#...#...#.#  
  #.###.#.#########.###.#.###.#                                                         #.#.#####.###.#####.#.###.###  
  #.#.#.#...#.......#...#...#..LE                                                       #...................#.....#.#  
  #.#.#.###.###.###.#.#####.###                                                         ###########################.#  
  #...#...#...#.#...#.#...#...#                                                         #.#.........................#  
  ###.###.#.###.###.#.#.#.#.###                                                         #.#.#.###.#.#.###.###.###.#.#  
YL......#...#.#.#...#...#.....#                                                         #.#.#.#.#.#.#.#.....#...#.#.#  
  ###.#######.###.#####.###.#.#                                                         #.###.#.#.###.###.#########.#  
  #.......#.#.....#.#...#...#.#                                                       JQ..#.#...#.#...#.....#.......#  
  ###.###.#.###.###.###.#.#.###                                                         #.#.###.###.###.###.###.#.###  
  #.....#...#...#.......#.#.#.#                                                         #.........#.#...#.....#.#....JL
  ###.#########.#####.#######.#    Y         A   I           M       U   K     J        #.#.###.#####.###.#########.#  
  #.........#.....#.....#.....#    L         C   J           V       J   Q     A        #.#.#...#.....#...#.#.......#  
  #.###.#.#####.#############.#####.#########.###.###########.#######.###.#####.#########.#####.#####.###.#.#####.###  
  #...#.#...#.....#.......#.#.........#.#.#.#...#.....#.......#.........#.....#.........#...#.#...#...#.....#...#...#  
  #####.#.#.#######.#####.#.#.###.#.###.#.#.#.#####.#######.#.#.#.#####.###.#.#######.#######.#.#.#.###.###.###.#.###  
  #.....#.#...#...#...#.#.#...#.#.#.#.........#.....#.......#.#.#...#...#...#.#.#.....#.#...#...#.#...#.#...#.......#  
  ###.#.#####.###.#.###.#.###.#.#.###.###.#######.#######.###.#.#######.###.#.#.#####.#.###.#.#.#######.###.###.###.#  
  #...#.....#...#...#.......#.#.....#.#...#.....#.#...#.....#.#.#.....#...#.#.....#.........#.#.......#.#.....#.#.#.#  
  #.#.#.#####.#######.#.#.#####.#.#######.#.#.###.#.#.#.#.#####.#.#.#######.#.###.###.###.###########.#.#.###.###.#.#  
  #.#.#...#...#.....#.#.#.......#...#.......#.#.#...#.#.#...#.#...#.....#...#...#.#.#.#.#.#...#...#...#.#.#.#.#.....#  
  ###.#.#.#########.#######.#####.#.###.#####.#.#####.#.#####.#####.#######.#.#####.#.#.###.#.#.#####.#####.#######.#  
  #...#.#.#.................#...#.#.#...#.#.#...#...#.#.#.....#.#.......#...#.#.....#.......#.#.....#.#...#.........#  
  #.###.#####.###.###.#.#.#.###.#.#####.#.#.#.###.#.#.#.#.#.###.#.#.#.#######.#####.#.###.#.#####.#####.###.###.#.#.#  
  #.#.....#...#.....#.#.#.#.#.......#...#.....#...#...#...#.#.....#.#.#.....#.#...#.....#.#...............#...#.#.#.#  
  ###.#.#######.#####.#####.###.#.#########.###.#.###.#.#####.#.#######.#.###.#.#####.###.#.#.#.#.###.#.###########.#  
  #.#.#...#.....#.......#...#.#.#.#.#.....#...#.#...#.#.#...#.#.#.#.#.#.#.........#.#...#.#.#.#.#...#.#.......#.....#  
  #.#.#.#.#####.#.#####.#.###.###.###.#.###.#.#.#######.#.#.###.#.#.#.###.#.#.#.###.#.###.###.###############.###.#.#  
  #.#.#.#.#.....#.#...#.#.#.....#.....#...#.#.#.#.#.....#.#.....#...#.....#.#.#...#.....#.#.#...........#.#.....#.#.#  
  #.###.#####.###.###.#.#.#.###.#.#.###.#.###.#.#.###.#.#####.#.#.###.#.###.###.###.#.#####.#####.#.#.###.#########.#  
  #.......#.#.#.#.#.....#.#.#.#.#.#.#...#.#...#.....#.#.#...#.#.#...#.#.#.....#...#.#.#.......#...#.#...........#...#  
  ###.#.#.#.###.#.###.#######.#.#####.#######.###.#####.#.#####.#.#####.#.#.#.#.#.###.#.#.#.###.###.###.###.#####.#.#  
  #...#.#...#.....#...#.#.#.#.#...#...#.#.#...#...#.#.....#...#...#.#.#.#.#.#.#.#.#.....#.#...#.#...#...#...#.#...#.#  
  ###.###.###.#.#.#####.#.#.#.###.#.###.#.#.#.#.#.#.###.###.###.#.#.#.#####.#########.###############.#.#####.###.#.#  
  #...#.#.#...#.#.#.#.#.#.#.#...........#.#.#.#.#...#.....#.....#.#.#.....#.#...#.....#.#.#...#.#.#.#.#...#.......#.#  
  ###.#.#####.#####.#.#.#.#.#.#######.###.###.#.#######.###.#.#.###.#.#####.#.#######.#.#.#.###.#.#.#######.#.###.#.#  
  #.#.#...#.#.#.#.............#.#.#...#.......#.......#.#.#.#.#.#.#...#.......#.#.#.#...................#.#.#...#.#.#  
  #.###.###.###.#######.###.###.#.#.#########.#.#######.#.#####.#.#.#####.#####.#.#.#.###.###############.#####.#####  
  #.........#.........#.#.....#.....#.....#.#.#...#.#.#...#.....#...#.#...#...#.#...#.#.#.......#.#...#...#.....#...#  
  #####.###.###.#####.#####.#####.#######.#.#.###.#.#.###.#####.#.#.#.#.###.#.#.#.#.#.#.#.#######.#.###.#.#####.#.###  
  #.....#.......#.............#...........#...#.......#...#.......#.#.......#...#.#.....#...............#.#.........#  
  #################################.###.###.#.#######.###.#######.###########.#####.#################################  
                                   A   U   Z M       S   P       U           J     O                                   
                                   A   X   Z V       C   S       I           Q     Y                                   "
];

pub type Name = (char, char);

pub type Loc = (usize, usize);

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Tile {
    Empty,
    Floor,
    Wall,
    Portal(Name),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PortalType {
    Inner,
    Outer,
}

pub type Map = Vec<Vec<Tile>>;


pub fn adjacent(loc: Loc, width: usize, height: usize) -> Vec<Loc> {
    let mut adj = Vec::new();

    if loc.0 + 1 != width {
        adj.push((loc.0 + 1, loc.1));
    }

    if loc.0 > 0 {
        adj.push((loc.0 - 1, loc.1));
    }

    if loc.1 + 1 != height {
        adj.push((loc.0,     loc.1 + 1));
    }

    if loc.1 > 0 {
        adj.push((loc.0,     loc.1 - 1));
    }

    return adj;
}

pub fn distance(loc0: Loc, loc1: Loc) -> usize {
    return ((loc0.0 as isize - loc1.0 as isize).abs() +
            (loc0.1 as isize - loc1.1 as isize).abs()) as usize;
}

pub fn get_portal_type(loc: Loc, width: usize, height: usize) -> PortalType {
    if loc.0 < 3 {
        return PortalType::Outer;
    }

    if width - loc.0 < 3 {
        return PortalType::Outer;
    }

    if loc.1 < 3 {
        return PortalType::Outer;
    }

    if height - loc.1 < 3 {
        return PortalType::Outer;
    }

    return PortalType::Inner;
}

pub fn parse_input(input: &str) -> (Map, Loc, Loc, HashMap<Loc, (Loc, PortalType)>) {
    let mut map = Vec::new();

    let mut portal_xy = Vec::new();

    for (y, line) in input.split("\n").enumerate() {
        let mut row = Vec::new();

        for (x, ch) in line.chars().enumerate() {
            match ch {
                ' ' => {
                    row.push(Tile::Empty);
                }

                '.' => {
                    row.push(Tile::Floor);
                }

                '#' => {
                    row.push(Tile::Wall);
                }

                _ => {
                    row.push(Tile::Empty);
                    portal_xy.push(((x, y), ch));
                }
            }
        }

        map.push(row);
    }

    let height = map.len();
    let width = map[0].len();

    let mut start_loc = None;
    let mut end_loc = None;

    let mut portal_list = Vec::new();
    for (loc, ch) in portal_xy.iter() {
        let adj = adjacent(*loc, width, height);
        if let Some(pos_ix) = adj.iter().position(|loc| { map[loc.1][loc.0] == Tile::Floor }) {
            let pos = adj[pos_ix];

            let other_ix =
                portal_xy.iter().position(|(this_loc, this_ch)| distance(*loc, *this_loc) == 1).unwrap();
            let other_ch = portal_xy[other_ix].1;

            if *ch == 'A' && other_ch == 'A' {
                start_loc = Some(*loc);
            } else if *ch == 'Z' && other_ch == 'Z' {
                end_loc = Some(*loc);
            } else {
                let x_diff = pos.0 as isize - loc.0 as isize;
                let y_diff = pos.1 as isize - loc.1 as isize;
                let name;
                if x_diff == 1 {
                    name = (other_ch, *ch);
                } else if x_diff == -1 {
                    name = (*ch, other_ch);
                } else if y_diff == 1 {
                    name = (other_ch, *ch);
                } else {
                    name = (*ch, other_ch);
                }
                map[loc.1][loc.0] = Tile::Portal(name);
                portal_list.push((*loc, name, pos));
                println!("Added {:?} {:?}", loc, name);
            }
        }
    }
    println!("Made portal list");

    for portal in portal_list.iter() {
        println!("{:?} ", portal);
    }

    let mut portals = HashMap::new();
    for portal in portal_list.iter() {
        let portal_type = get_portal_type(portal.0, width, height);
        println!("portal {:?}, {:?}", portal, portal_type);
        let other_opening =
            portal_list.iter().position(|loc| loc.1 == portal.1 && loc.0 != portal.0).unwrap();
        portals.insert(portal.0, (portal_list[other_opening].2, portal_type));
    }
    println!("Found portals");


    return (map, start_loc.unwrap(), end_loc.unwrap(), portals);
}

fn main() {
    let (map, start_loc, end_loc, portals) = parse_input(INPUT[IX]);

    for row in map.iter() {
        for tile in row.iter() {
            match tile {
                Tile::Empty => print!(" "),
                Tile::Floor => print!("."),
                Tile::Wall => print!("#"),
                Tile::Portal(Name) => print!("O"),
            }
        }
        println!("");
    }

    let height = map.len();
    let width = map[0].len();

    let (path, cost) =
        astar(&(start_loc, 0),
              |(p, depth)| {
                  let mut sucs = Vec::new();

                  for loc in adjacent(*p, width, height) {
                      match map[loc.1][loc.0] {
                          Tile::Floor => {
                            sucs.push(((loc, *depth), 1));
                          }

                          Tile::Portal(name) => {
                              let (new_loc, portal_type) = *portals.get(&loc).unwrap();
                              match portal_type {
                                  PortalType::Inner => {
                                      sucs.push(((new_loc, depth+1), 1));
                                  }

                                  PortalType::Outer => {
                                      if *depth > 0 {
                                          sucs.push(((new_loc, depth-1), 1));
                                      }
                                  }

                              }
                          }

                          _ => {}
                      }
                  }

                  return sucs;
              },
              |(p, _)| {
                  0//distance(*p, end_loc)
              },
              |(p, depth)| {
                  distance(*p, end_loc) == 1 && *depth == 0
              }).expect("Couldn't find path!");

    //for entry in path {
    //    println!("{:?}", entry);
    //}

    println!("Cost = {}", cost - 1);
}
