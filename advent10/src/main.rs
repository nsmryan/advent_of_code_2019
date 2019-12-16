
const INPUT: &str =
//".###.###.###.#####.#
//#####.##.###..###..#
//.#...####.###.######
//######.###.####.####
//#####..###..########
//#.##.###########.#.#
//##.###.######..#.#.#
//.#.##.###.#.####.###
//##..#.#.##.#########
//###.#######.###..##.
//###.###.##.##..####.
//.##.####.##########.
//#######.##.###.#####
//#####.##..####.#####
//##.#.#####.##.#.#..#
//###########.#######.
//#.##..#####.#####..#
//#####..#####.###.###
//####.#.############.
//####.#.#.##########.";
// (5, 8) => 33
"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
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

    while diff_pos.0.abs() % 2 != 0 || diff_pos.1.abs() % 2 != 0 {
        println!("diff_pos = ({}, {})", diff_pos.0, diff_pos.1);
        if diff_pos.0.abs() % 2 == 0 {
            diff_pos.0 /= 2;
        }

        if diff_pos.1.abs() % 2 == 0 {
            diff_pos.1 /= 2;
        }

        if field[(station.0 + diff_pos.0) as usize][(station.1 + diff_pos.1) as usize] == Loc::Astroid {
            blocked = true;
            break;
        }

        if diff_pos.0 == 0 || diff_pos.1 == 0 || (diff_pos.0.abs() % 2 != 0 && diff_pos.1.abs() % 2 != 0) {
            break;
        }
    }

    //diagonal
    if diff_pos.0.abs() == diff_pos.1.abs() {
    }

    // on-axis
    if diff_pos.0 == 0 || diff_pos.1 == 0 {
    }

    return !blocked;
}

fn main() {
    let field = parse_string(INPUT);

    let mut best_loc = (0, 0);
    let mut num_best_asteroids = 0;
    for station_y in 0..field_height(&field) {
        for station_x in 0..field_width(&field) {
            println!("Trying station ({}, {})", station_x, station_y);
            let mut current_num_asteroids = 0;
            for y in 0..field_height(&field) {
                for x in 0..field_width(&field) {
                    if x == station_x && y == station_y {
                        continue;
                    }

                    if can_see_from((station_x as isize, station_y as isize), &field, (x as isize, y as isize)) {
                        current_num_asteroids += 1;
                    }
                }
            }

            if current_num_asteroids > num_best_asteroids {
                num_best_asteroids = current_num_asteroids;
                best_loc = (station_x, station_y);
            }
        }
    }

    println!("Result = {}", num_best_asteroids);
}

