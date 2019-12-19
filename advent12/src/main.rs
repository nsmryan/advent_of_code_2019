use std::collections::HashMap;
use std::collections::HashSet;


const NUM_STEPS: usize = 1000;

const INPUT: &str = 
"17,5,1
-2,-8,8
7,-6,14
1,-10,4";

pub struct Planet {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub vx: isize,
    pub vy: isize,
    pub vz: isize,
}

impl Planet {
    pub fn new(x: isize, y: isize, z: isize) -> Planet {
        return Planet {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        };
    }

    pub fn energy(&self) -> usize {
        let pot = self.x.abs() + self.y.abs() + self.z.abs();
        let kin = self.vx.abs() + self.vy.abs() + self.vz.abs();

        let energy = pot * kin;

        return energy as usize;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Axis {
    pub pos: isize,
    pub vel: isize,
}

impl Axis {
    pub fn new(pos: isize) -> Axis {
        return Axis { pos, vel: 0 };
    }

    pub fn step(&mut self) {
        self.pos += self.vel;
    }

    pub fn update(axis: &mut [Axis]) {
        for ix in 0..axis.len() {
            for other_ix in 0..axis.len() {
                if ix != other_ix {
                    axis[ix].vel += pull(axis[ix].pos, axis[other_ix].pos);
                }
            }
        }

        for ix in 0..axis.len() {
            axis[ix].step();
        }
    }
}

pub fn pull(first: isize, second: isize) -> isize {
    let diff = second - first;

    if diff < 0 {
        -1
    } else if diff > 0 {
        1
    } else {
        0
    }
}

pub fn sim_step(planets: &mut [Planet]) {
    for ix in 0..planets.len() {
        for other_ix in 0..planets.len() {
            if ix != other_ix {
                planets[ix].vx += pull(planets[ix].x, planets[other_ix].x);
                planets[ix].vy += pull(planets[ix].y, planets[other_ix].y);
                planets[ix].vz += pull(planets[ix].z, planets[other_ix].z);
            }
        }
    }

    for ix in 0..planets.len() {
        planets[ix].x += planets[ix].vx;
        planets[ix].y += planets[ix].vy;
        planets[ix].z += planets[ix].vz;
    }
}

pub fn energy(planets: &mut [Planet]) -> usize {
    let mut energy: usize = 0;

    for planet in planets {
        energy += planet.energy();
    }

    return energy;
}

pub fn steps_to_repeat(axis: &mut Vec<Axis>) -> usize {
    let start = axis.clone();
    Axis::update(axis);

    let mut step: usize = 1;
    while *axis != start {
        Axis::update(axis);
        step += 1;
    }

    return step;
}

pub fn is_prime(num: usize) -> bool {
    let up_bound = (num as f64).sqrt() as usize; 
    for ix in 2..=up_bound {
        if num % ix == 0 {
            return false;
        }
    }

    return true;
}

#[test]
pub fn test_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(!is_prime(4));
    assert!(!is_prime(10));
}

pub fn prime_factors(num: usize) -> HashMap<usize, usize> {
    let mut factors = HashMap::new();

    let up_bound = (num as f64).sqrt() as usize;
    for val in 2..=(num / 2) {
        if is_prime(val) {
            if num % val == 0 {
                if !factors.contains_key(&val) {
                    factors.insert(val, 0);
                }
                let current = factors.get_mut(&val).unwrap();

                let mut count: usize = 1;
                while num % val.pow(count as u32 + 1) == 0 {
                    count += 1;
                }
                dbg!(count);
                *current += count;
            }
        }
    }

    dbg!(&factors);

    return factors;
}

#[test]
pub fn test_prime_factors() {
    let factors = prime_factors(10);
    assert_eq!(1, *factors.get(&2).unwrap());

    let factors = prime_factors(20);
    assert_eq!(2, *factors.get(&2).unwrap());

    let factors = prime_factors(128);
    assert_eq!(7, *factors.get(&2).unwrap());
}

pub fn lcm(nums: &[usize]) -> usize {
    let mut factors = Vec::new();
    for num in nums.iter() {
        factors.push(prime_factors(*num));
    }
    dbg!(&factors);

    let mut all_primes = HashSet::new();
    for factor in factors.iter() {
        for key in factor.keys() {
            let key_clone = key.clone();
            all_primes.insert(key_clone);
        }
    }

    dbg!(&all_primes);

    let mut shared = 1;
    let mut unshared = 1;

    for prime in all_primes {
        if factors.iter().all(|mapping| mapping.contains_key(&prime)) {
            let new_factors = factors.clone();
            let min_factor = new_factors.iter().map(|mapping| mapping.get(&prime).unwrap()).min().unwrap();
            dbg!(prime, min_factor);
            shared *= prime * min_factor;

            for factor in factors.iter_mut() {
                let count = factor.get_mut(&prime).unwrap();
                *count -= min_factor;
            }
        }
    }

    for factor in factors.iter() {
        for (prime, count) in factor.iter() {
            if *count != 0 {
                unshared *= prime * count;
            }
        }
    }

    dbg!(shared);
    dbg!(unshared);

    return shared * unshared;
}

fn main() {
    let coords = 
        INPUT.split("\n")
             .map(|line| line.split(",").map(|val_str| val_str.parse::<isize>().unwrap()).collect::<Vec<isize>>())
             .collect::<Vec<Vec<isize>>>();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();
    for coord in coords.iter() {
        xs.push(Axis::new(coord[0]));
        ys.push(Axis::new(coord[1]));
        zs.push(Axis::new(coord[2]));
    }

    let start = xs.clone();
    Axis::update(&mut xs);

    let x_steps = steps_to_repeat(&mut xs);
    let y_steps = steps_to_repeat(&mut ys);
    let z_steps = steps_to_repeat(&mut zs);
    println!("xs = {}", x_steps);
    println!("ys = {}", y_steps);
    println!("zs = {}", z_steps);

    println!("Result = {}", lcm(&[x_steps, y_steps, z_steps]));

    /*
    let mut planets = Vec::new();
    for coord in coords.iter() {
        planets.push(Planet::new(coord[0], coord[1], coord[2]));
    }

    let mut step: usize = 0;
    loop {
        sim_step(&mut planets);
        step += 1;

        if step % 10000000 == 0 {
            println!("\n{}\n", step);
        }

        if energy(&mut planets) == 0 {
            break;
        }
    }

    println!("Result = {}", energy(&mut planets));
    println!("Result = {}", step);
    */
}
