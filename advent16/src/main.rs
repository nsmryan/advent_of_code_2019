use std::collections::HashMap;

use rayon::prelude::*;


const INPUT: &str =
//"12345678";
//"03036732577212944063491565474664";
//"80871224585914546619083218645595";
//"19617804207202209144916044189917";
"59766832516471105169175836985633322599038555617788874561522148661927081324685821180654682056538815716097295567894852186929107230155154324411726945819817338647442140954601202408433492208282774032110720183977662097053534778395687521636381457489415906710702497357756337246719713103659349031567298436163261681422438462663511427616685223080744010014937551976673341714897682634253850270219462445161703240957568807600494579282412972591613629025720312652350445062631757413159623885481128914333982571503540357043736821931054029305931122179293220911720263006705242490442826574028623201238659548887822088996956559517179003476743001815465428992906356931239533104";

const RUN_TIMES: usize = 100;
const FAST: bool = true;
const DUPLICATE_TIMES: usize = 10000;

const RANGE_START: usize = 8;

pub type Message = Vec<i64>;

pub fn parse_input(input: &str) -> Message {
    return input.chars().map(|ch| ch as i64 - '0' as i64).collect::<Vec<i64>>();
}

pub fn greatest_power_of_two(num: usize) -> usize {
    let two: usize = 2;
    for ix in 1..=63 {
        let p = two.pow(ix);
        if p == num {
            return two.pow(ix);
        } else if p > num {
            return two.pow(ix-1);
        }
    }
    panic!("Num was too large?");
}

#[test]
pub fn test_pow_2() {
    assert_eq!(greatest_power_of_two(100), 64);
    assert_eq!(greatest_power_of_two(10), 8);
    assert_eq!(greatest_power_of_two(1024), 1024);
    assert_eq!(greatest_power_of_two(1 << 30), 1 << 30);
}

// returns offset and length of largest power of 2 range within the given range
pub fn find_subrange(start: usize, length: usize) -> (usize, usize) {

    let mut guess = start;
    let mut prev_guess = guess;

    let end = start + length;

    let mut power_of_two = RANGE_START / 2;
    let mut prev_power = power_of_two;

    while (guess + power_of_two) <= end {
        prev_guess = guess;
        prev_power = power_of_two;

        power_of_two *= 2;
        let mask = power_of_two + (power_of_two - 1);
        let unmask = power_of_two | !(power_of_two - 1);
        guess = (start & (power_of_two | !(power_of_two - 1))) +
                ((((start & (power_of_two - 1)) != 0) as usize) * power_of_two);
        //println!("Guessing {} and {}, mask {} ({:X}), unmask ({:X}), masked {}, add {}",
        //         guess,
        //         power_of_two,
        //         mask,
        //         mask,
        //         unmask,
        //         start & !(power_of_two - 1),
        //         (((start & mask != power_of_two) as usize)));
    }

    return (prev_guess, prev_power);
}

#[test]
pub fn test_find_subrange() {
    assert_eq!(find_subrange(16, 17), (16, 16));
    assert_eq!(find_subrange(50, 17), (54, 8));
    assert_eq!(find_subrange(64, 16), (64, 16));
    assert_eq!(find_subrange(65, 32), (80, 16));
    assert_eq!(find_subrange(31, 32), (32, 16));
    assert_eq!(find_subrange(33, 33), (48, 16));
    assert_eq!(find_subrange(128, 128), (128, 128));
    assert_eq!(find_subrange(1024, 256), (1024, 256));
    assert_eq!(find_subrange(1024000, 2048), (1024000, 2048));
}

pub fn fast_sum(range_sums: &HashMap<(usize, usize), i64>, message: &[i64], start: usize, length: usize) -> i64 {
    let mut sum = 0;

    let (subrange_offset, largest_subrange) = find_subrange(start, length);
    if largest_subrange <= RANGE_START {
        for ix in 0..length {
            sum += message[start + ix];
        }
    } else {

        let range_pair = (subrange_offset, subrange_offset + largest_subrange);
        //if range_pair.1 >= message.len() {
        //    println!("Start: {}, length: {}, end {}", start, length, start + length);
        //    println!("range offset {} {:X}, range length {}", subrange_offset, subrange_offset, largest_subrange);
        //    println!("Retrieving ({}, {})", range_pair.0, range_pair.1);
        //}
        //println!("{}, {} => ({}, {})", start, length, range_pair.0, range_pair.1);
        sum += range_sums.get(&range_pair).unwrap();

        if subrange_offset > start {
            sum += fast_sum(range_sums, message, start, subrange_offset - start);
        }

        if (subrange_offset + largest_subrange) < (start + length) {
            let end_start = subrange_offset + largest_subrange;
            let end_length = (start + length) - end_start;
            sum += fast_sum(range_sums, message, end_start, end_length);
        }
    }


    return sum;
}

pub fn make_range_sums(message: &Vec<i64>) -> HashMap<(usize, usize), i64> {
    let mut range_sums: HashMap<(usize, usize), i64> = HashMap::new();
    let mut range_length = RANGE_START;
    for ix in 0..(message.len() / range_length) {
        let sum: i64 = (0..range_length).map(|digit_ix| message[ix*range_length + digit_ix]).sum();
        let pair = (ix*range_length, ix*range_length + range_length);
        range_sums.insert(pair, sum);
    }

    range_length *= 2;
    while range_length < message.len() {
        let prev_length = range_length / 2;
        for ix in 0..(message.len() / range_length) {
            let first = (2*ix * prev_length, 2*ix * prev_length + prev_length);
            let second = ((2*ix + 1) * prev_length, (2*ix + 1) * prev_length + prev_length);
            let first_sum = range_sums.get(&first).unwrap();
            let second_sum = range_sums.get(&second).unwrap();
            let sum = first_sum + second_sum;
            range_sums.insert((ix*range_length, ix*range_length + range_length), sum);
        }

        range_length *= 2;
    }

    return range_sums;
}

pub fn phase(message: &Message) -> Message {
    println!("Making range sums");
    let mut range_sums = make_range_sums(message);
    println!("finished range sums");

    let pattern = [0, 1, 0, -1];
    let new_message = (0..message.len()).into_par_iter().map(|ix| {
    //let new_message = (0..message.len()).into_iter().map(|ix| {
    //for ix in 0..message.len() {
        //if ix % 500000 == 0 {
            //println!("ix = {} of {}", ix, message.len());
        //}
        let mut sum: i64 = 0;

        // empty pattern
        /*
        sum = 0;
        */

        // compressed pattern
        /*
        for digit_ix in ix..message.len() {
            let pat = pattern[(((digit_ix + 1) / (ix + 1))) % pattern.len()];

            sum += message[digit_ix] * pat;
        }
        */

        // log2 region patterns
        let mut region_start = 0;
        let section_length = ix + 1;
        let mut iters = 0;
        //println!("Sum for {} (section length {})", ix, section_length);
        //println!("");
        loop {
            let mut pos_start = region_start + section_length;
            let mut neg_start = region_start + section_length * 3;
            if iters == 0 {
                pos_start -= 1;
                neg_start -= 1;
            }
            //println!("LOOPING {}: pos {}, neg {}", region_start, pos_start, neg_start);

            let mut pos_length = section_length;
            if pos_start + pos_length >= message.len() {
                pos_length = std::cmp::max(0, (message.len() - pos_start) as i64) as usize;
            }
            if pos_length > 0 {
                if FAST {
                    sum += fast_sum(&range_sums, message, pos_start, pos_length);
                } else {
                    for ix in 0..pos_length {
                        //print!("1");
                        //println!("pos ix {} = {}", pos_start + ix, message[pos_start + ix]);
                        sum += message[pos_start + ix];
                    }
                }
            } else {
                break;
            }
            
            let mut neg_length = section_length;
            if neg_start + neg_length >= message.len() {
                neg_length = std::cmp::max(0, (message.len() - neg_start) as i64) as usize;
            }
            if neg_length > 0 {
                if FAST {
                    sum -= fast_sum(&range_sums, message, neg_start, neg_length);
                } else {
                    for ix in 0..neg_length {
                        //print!("-");
                        //println!("neg ix {} = {}", neg_start + ix, message[neg_start + ix]);
                        sum -= message[neg_start + ix];
                    }
                }
            } else {
                break;
            }

            region_start += (section_length * 4);
            if iters == 0 {
                region_start -= 1;
            } 

            iters += 1;
        }
        //println!("");
        //println!("sum = {}", sum % 10);

        //new_message.push(sum.abs() % 10);
        return sum.abs() % 10;
    }).collect::<Vec<i64>>();

    println!("Finished phase!");

    return new_message;
}

fn main() {
    let mut message = parse_input(INPUT);

    let initial_message = message.clone();
    for _ in 1..DUPLICATE_TIMES {
        let mut to_dup = initial_message.clone();
        message.append(&mut to_dup);
    }

    println!("Message length = {}", message.len());

    let offset = 
             message[6] + 
             message[5] * 10 + 
             message[4] * 100 + 
             message[3] * 1000 + 
             message[2] * 10000 + 
             message[1] * 100000 +
             message[0] * 1000000;
    let offset = offset as usize;
    println!("Offset = {}", offset);

    println!("{}{}{}{}{}{}{}{}",
             message[0],
             message[1],
             message[2],
             message[3],
             message[4],
             message[5],
             message[6],
             message[7]);

    for ix in 0..RUN_TIMES {
        println!("Starting phase {}", ix);
        message = phase(&message);
        println!("{}{}{}{}{}{}{}{}",
                 message[0],
                 message[1],
                 message[2],
                 message[3],
                 message[4],
                 message[5],
                 message[6],
                 message[7]);
    }

    println!("Answer = {}{}{}{}{}{}{}{}",
             message[0],
             message[1],
             message[2],
             message[3],
             message[4],
             message[5],
             message[6],
             message[7]);


    println!("Final Message = {}{}{}{}{}{}{}{}",
             message[0 + offset],
             message[1 + offset],
             message[2 + offset],
             message[3 + offset],
             message[4 + offset],
             message[5 + offset],
             message[6 + offset],
             message[7 + offset]);

}
