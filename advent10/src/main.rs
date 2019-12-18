use std::collections::HashMap;


const INPUT: &str =
".###.###.###.#####.#
#####.##.###..###..#
.#...####.###.######
######.###.####.####
#####..###..########
#.##.###########.#.#
##.###.######..#.#.#
.#.##.###.#.####.###
##..#.#.##.#########
###.#######.###..##.
###.###.##.##..####.
.##.####.##########.
#######.##.###.#####
#####.##..####.#####
##.#.#####.##.#.#..#
###########.#######.
#.##..#####.#####..#
#####..#####.###.###
####.#.############.
####.#.#.##########.";
// example 
//".#....#####...#..
//##...##.#####..##
//##...#...#.#####.
//..#.....#...###..
//..#.#.....#....##";
// (5, 8) => 33
//"......#.#.
//#..#.#....
//..#######.
//.#.#.###..
//.#..#.....
//..#....#.#
//#..#....#.
//.##.#..###
//##...#..#.
//.#....####";
// (1, 2) => 35
//"#.#...#.#.
//.###....#.
//.#....#...
//##.#.#.#.#
//....#.#.#.
//.##..###.#
//..#...##..
//..##....##
//......#...
//.####.###.";
// (6, 3) => 41
//".#..#..###
//####.###.#
//....###.#.
//..###.##.#
//##.##.#.#.
//....###..#
//..#.#..#.#
//#..#.#.###
//.##...##.#
//.....#.#..";

type Field = Vec<Vec<Loc>>;

type FieldMap = HashMap<isize, Vec<((usize, usize), f32)>>;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Loc {
    Empty,
    Astroid,
}

impl Loc {
    pub fn from_char(loc: char) -> Loc {
        match loc {
            '#' => Loc::Astroid,
            '.' => Loc::Empty,
            _   => panic!("Unexpected input in astroid field!"),
        }
    }
}


pub fn parse_string(field_string: &str) -> Field {
    let mut field = Vec::new();

    for row in field_string.split_whitespace() {
        let mut field_row = Vec::new();
        for loc in row.chars() {
            field_row.push(Loc::from_char(loc));
        }

        field.push(field_row);
    }

    return field;
}

pub fn field_width(field: &Field) -> usize {
    return field[0].len();
}

pub fn field_height(field: &Field) -> usize {
    return field.len();
}

pub fn can_see_from(station: (isize, isize), field: &Field, loc: (isize, isize)) -> bool {
    let mut diff_pos = (loc.0 - station.0, loc.1 - station.1);

    let mut blocked = false;

    while (diff_pos.0 != 0 || diff_pos.1 != 0) &&
          (diff_pos.0.abs() % 2 == 0 && diff_pos.1.abs() % 2 == 0) {
        let mut changed = false;
        if diff_pos.0 != 0 && diff_pos.0.abs() % 2 == 0 {
            diff_pos.0 /= 2;
            changed = true;
        }

        if diff_pos.1 != 0 && diff_pos.1.abs() % 2 == 0 {
            diff_pos.1 /= 2;
            changed = true;
        }

        if !changed {
            break;
        }

        let x_pos = (station.0 + diff_pos.0) as usize;
        let y_pos = (station.1 + diff_pos.1) as usize;
        if field[y_pos][x_pos] == Loc::Astroid {
            blocked = true;
            break;
        }
    }

    let x_sign = if diff_pos.0 < 0 {
        -1
    } else if diff_pos.0 > 0 {
        1
    } else {
        0
    };

    let y_sign = if diff_pos.1 < 0 {
        -1
    } else if diff_pos.1 > 0 {
        1
    } else {
        0
    };

    // diagonals
    if diff_pos.0.abs() == diff_pos.1.abs() {
        for offset in 1..diff_pos.0.abs() {
            let x_pos = (station.0 + offset * x_sign) as usize;
            let y_pos = (station.1 + offset * y_sign) as usize;
            if field[y_pos][x_pos] == Loc::Astroid {
                blocked = true;
                break;
            }
        }
    }

    // horizontal
    if diff_pos.0.abs() > 1  && diff_pos.1 == 0 {
        for x in 1..diff_pos.0.abs() {
            let x_pos = (station.0 + x * x_sign) as usize;
            let y_pos = station.1 as usize;
            if field[y_pos][x_pos] == Loc::Astroid {
                blocked = true;
                break;
            }
        }
    }

    // vertical
    if diff_pos.0 == 0 && diff_pos.1.abs() > 1 {
        for y in 1..diff_pos.1.abs() {
            let x_pos = station.0 as usize;
            let y_pos = (station.1 + y * y_sign) as usize;
            if field[y_pos][x_pos] == Loc::Astroid {
                blocked = true;
                break;
            }
        }
    }

    return !blocked;
}

fn print_field(field: &Field, hits: Vec<Vec<bool>>, loc: (usize, usize)) {
    for y in 0..field_height(&field) {
        for x in 0..field_width(&field) {
            if loc == (x, y) {
                print!("S");
            } else if hits[y][x] {
                if field[y][x] == Loc::Astroid {
                    print!("#");
                } else {
                    panic!("weird!");
                }
            } else if field[y][x] == Loc::Astroid {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("_______________________");
}

fn main_attempt1() {
    let field = parse_string(INPUT);

    let mut best_loc = (0, 0);
    let mut num_best_asteroids = 0;
    let mut best_hits = vec!();
    let height = field_height(&field);
    let width = field_width(&field);
    let mut hits = vec![vec![false; width]; height];

    for station_y in 0..height {
        for station_x in 0..width {
            hits = vec![vec![false; width]; height];
            let mut current_num_asteroids = 0;
            for y in 0..height {
                for x in 0..width {
                    if x == station_x && y == station_y {
                        continue;
                    }

                    let pos = (station_x as isize, station_y as isize);
                    let can_see = can_see_from(pos, &field, (x as isize, y as isize));
                    let is_asteroid = field[y][x] == Loc::Astroid;
                    let hit = can_see && is_asteroid;

                    hits[y][x] = hit;

                    if hit {
                        current_num_asteroids += 1;
                    }
                }
            }

            if current_num_asteroids > num_best_asteroids {
                num_best_asteroids = current_num_asteroids;
                best_loc = (station_x, station_y);
                best_hits = hits;
            }
        }
    }

    print_field(&field, best_hits, best_loc);

    println!("Best position: {:?}", best_loc);
    println!("Result = {}", num_best_asteroids);
}

fn get_angle(station: (usize, usize), asteroid: (usize, usize)) -> isize {
    let x_diff = station.0 as isize - asteroid.0 as isize;
    let y_diff = station.1 as isize - asteroid.1 as isize;
    let mut angle = (y_diff as f32).atan2(x_diff as f32);
    if angle < 0.0 {
        angle += 2.0 * std::f32::consts::PI;
    }

    angle += std::f32::consts::PI * (3.0 / 2.0);

    if angle > 2.0 * std::f32::consts::PI {
        angle -= 2.0 * std::f32::consts::PI;
    }

    let mut int_angle = (angle * 10000.0) as isize;

    if int_angle == 62831 {
        int_angle = 0;
    }

    return int_angle;
}

fn distance(station: (usize, usize), asteroid: (usize, usize)) -> f32 {
    let x = (station.0 as isize - asteroid.0 as isize).pow(2);
    let y = (station.1 as isize - asteroid.1 as isize).pow(2);
    return ((x + y) as f32).sqrt();
}

fn can_see_angle(field_map: &FieldMap, station: (usize, usize), asteroid: (usize, usize)) -> bool {
    let angle = get_angle(station, asteroid);
    let station_dist = distance(station, asteroid);

    let mut num_hits = 0;
    let dists: &Vec<((usize, usize), f32)> = &field_map.get(&angle).unwrap();
    //dbg!(angle, dists);
    for dist in dists.iter() {
        if dist.1 < station_dist {
            num_hits += 1;
        }
    }

    return num_hits == 0;
}

fn make_field_map(field: &Field, loc: (usize, usize)) -> FieldMap {
    let mut field_map: FieldMap = HashMap::new();

    let height = field_height(&field);
    let width = field_width(&field);
    for y in 0..height {
        for x in 0..width {
            if loc == (x, y) || field[y][x] != Loc::Astroid {
                continue;
            }

            let angle = get_angle(loc, (x, y));
            if !field_map.contains_key(&angle) {
                field_map.insert(angle, Vec::new());
            }
            let value = field_map.get_mut(&angle).unwrap();
            value.push(((x, y), distance(loc, (x, y))));
        }
    }

    return field_map;
}

fn location_hits(field: &Field, station: (usize, usize)) -> usize {
    let field_map = make_field_map(&field, station);
    let mut hits = 0;
    let height = field_height(&field);
    let width = field_width(&field);

    for y in 0..height {
        for x in 0..width {
            if (x, y) == station {
                continue;
            }

            if field[y][x] == Loc::Astroid {
                if can_see_angle(&field_map, station, (x, y)) {
                    hits += 1;
                }
            }
        }
    }

    return hits;
}

fn try_loc(field: &Field, loc: (usize, usize)) {
    println!("________");
    println!("Hits at {:?}: {}", loc, location_hits(&field, loc));
    let width = field_width(&field);
    let field_map = make_field_map(&field,loc);
    //dbg!(&field_map);
    let height = field_height(&field);
    let mut hits = vec![vec![false; width]; height];
    for y in 0..height {
        for x in 0..width {
            if field[y][x] == Loc::Astroid {
                hits[y][x] = can_see_angle(&field_map, loc, (x, y));
                
            }
        }
    }
    print_field(&field, hits, loc);
}

fn main() {
    let field = parse_string(INPUT);

    let mut best_loc = (0, 0);
    let mut num_best_asteroids = 0;
    let mut best_hits = vec!();
    let height = field_height(&field);
    let width = field_width(&field);
    let mut hits = vec![vec![false; width]; height];
    let mut best_field_map = HashMap::new();


    for station_y in 0..height {
        for station_x in 0..width {
            if field[station_y][station_x] != Loc::Astroid {
                continue;
            }

            hits = vec![vec![false; width]; height];
            let mut current_num_asteroids = 0;
            let field_map = make_field_map(&field, (station_x, station_y));
            for y in 0..height {
                for x in 0..width {
                    if x == station_x && y == station_y {
                        continue;
                    }

                    let pos = (station_x as isize, station_y as isize);
                    if field[y][x] == Loc::Astroid {
                        let hit = can_see_angle(&field_map, (station_x, station_y), (x, y));

                        hits[y][x] = hit;

                        if hit {
                            current_num_asteroids += 1;
                        }
                    }
                }
            }

            if current_num_asteroids > num_best_asteroids {
                num_best_asteroids = current_num_asteroids;
                best_loc = (station_x, station_y);
                best_hits = hits;
                best_field_map = field_map;
            }
        }
    }

    //dbg!(best_field_map);
    //dbg!(best_loc);
    //dbg!(&field);

    print_field(&field, best_hits, best_loc);

    println!("Best position: {:?}", best_loc);
    println!("Result = {}", num_best_asteroids);

    let answer_loc = (5, 8);
    try_loc(&field, answer_loc);

    let test_case = 
        "..........
        .#...#....
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........";
    let test_field = parse_string(&test_case);
    //try_loc(&test_field, (1, 1));

    let result_loc = (8, 16);

    println!("Angle = {}", get_angle(result_loc, (4, 16)));
    println!("Angle = {}", get_angle(result_loc, (10, 16)));
    println!("Angle = {}", get_angle(result_loc, (8, 8)));
    println!("Angle = {}", get_angle(result_loc, (8, 19)));

    let best_loc = (8, 16);
    let best_field_map = make_field_map(&parse_string(INPUT), best_loc);

    let angles =
        best_field_map.keys()
                      .into_iter()
                      .map(|angle| *angle)
                      .collect::<Vec<isize>>();
    let mut locs = 
        best_field_map.values()
                      .into_iter()
                      .map(|locs| locs.iter().map(|loc| *loc).collect::<Vec<((usize, usize), f32)>>())
                      .collect::<Vec<Vec<((usize, usize), f32)>>>();

    let mut blast_data =
        best_field_map.iter()
              .map(|(angle, vec)| (*angle, vec.iter().map(|val| *val).collect::<Vec<((usize, usize), f32)>>()))
              .collect::<Vec<(isize, Vec<((usize, usize), f32)>)>>();
    blast_data.sort_by_key(|dat| dat.0);

    for mut value in blast_data.iter_mut() {
        value.1.sort_by_key(|value| (value.1 * 10000.0) as usize);
    }

    let mut blasted = 0;
    let mut last_loc = (0, 0);
    let mut ix = 0;
    let mut any_blasted = false;
    while blasted < 200 {
        if blast_data[ix].1.len() > 0 {
          let loc = blast_data[ix].1.remove(0).0;
          println!("{:?}", loc);
          blasted += 1;
          last_loc = loc;
          any_blasted = true;
        }
        ix = (ix + 1) % locs.len();
        if ix == 0 {
            if !any_blasted {
                println!("Not enough asteroids!");
                break;
            } 
            any_blasted = false;
        }
    }

    println!("loc = {:?}", last_loc);
    println!("Result = {:?}", last_loc.0 * 100 + last_loc.1);
}
