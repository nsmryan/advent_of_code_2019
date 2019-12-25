use std::collections::VecDeque;
use std::collections::HashSet;


const NUM_CARDS: usize = 5596 + 10007; //119315717514047;

const IX: usize = 4;

const INPUT: [&str; 5] = [
// 0 3 6 9 2 5 8 1 4 7
"increment 7
new stack
new stack",

// 3 0 7 4 1 8 5 2 9 6
"cut 6
increment 7
new stack",

// 6 3 0 7 4 1 8 5 2 9
"increment 7
increment 9
cut -2",

// 9 2 5 8 1 4 7 0 3 6
"new stack
cut -2
increment 7
cut 8
cut -4
increment 7
cut 3
increment 9
increment 3
cut -1",

"increment 54
cut -667
increment 15
cut -1826
increment 55
cut -8444
increment 44
cut 910
increment 63
cut 4025
increment 45
cut 6430
increment 53
cut -3727
new stack
increment 6
cut -5464
new stack
increment 48
cut 6238
increment 23
cut 8614
increment 50
cut -987
increment 26
cut -9808
increment 47
cut -8088
increment 5
new stack
cut 5787
increment 49
cut 795
increment 2
cut -536
increment 26
new stack
cut -6327
increment 63
cut 2511
increment 38
cut -2622
new stack
increment 9
cut 8201
new stack
increment 48
cut -2470
increment 19
cut 8669
new stack
increment 28
cut -2723
new stack
increment 15
cut -5101
new stack
cut 464
increment 68
cut 2695
increment 53
cut -8523
increment 32
cut -1018
increment 66
cut 9127
increment 3
new stack
increment 14
cut 725
new stack
cut -2273
increment 65
cut 6306
increment 55
cut -6710
increment 54
cut 7814
increment 23
cut 8877
increment 60
cut 3063
increment 40
cut -2104
increment 72
cut -4171
increment 21
cut 7919
increment 53
cut -3320
increment 49
new stack
cut -8201
new stack
increment 54
new stack
cut 6321
increment 50
cut 7244
increment 23"
];

pub type Card = u128;

pub type Deck = VecDeque<Card>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instr {
    Incr(isize),
    NewStack,
    Cut(isize),
}


impl Instr {
    pub fn unapply(self, loc: i128, num_cards: i128) -> i128 {
        match self {
            Instr::Incr(inc_amount) => {
                //println!("Increment {}", inc_amount);

                (loc * inc_amount as i128 % num_cards) 
            }

            Instr::NewStack => {
                //println!("New Stack");
                num_cards - loc - 1
            }

            Instr::Cut(cut_amount) => {
                let cut_loc = loc + (-1 as i128 * cut_amount as i128);
                if cut_loc < 0 {
                    num_cards - cut_loc.abs()
                } else {
                    cut_loc % num_cards
                }
            }
        }
    }

    pub fn apply(self, deck: &mut Deck, scratch: &mut Deck) {
        match self {
            Instr::Incr(inc_amount) => {
                //println!("Increment {}", inc_amount);

                scratch.clear();
                for ix in 0..deck.len() {
                    scratch.push_back(0);
                }

                for ix in 0..deck.len() {
                    let mut pos = ix as isize * inc_amount;
                    let pos = pos as u128 % deck.len() as u128;
                    scratch[pos as usize] = deck[ix as usize];
                }

                for (ix, card) in scratch.iter().enumerate() {
                    deck[ix] = *card;
                }
            }

            Instr::NewStack => {
                //println!("New Stack");

                scratch.clear();
                for card in deck.iter() {
                    scratch.push_front(*card);
                }

                for (ix, card) in scratch.iter().enumerate() {
                    deck[ix] = *card;
                }
            }

            Instr::Cut(cut_amount) => {
                if cut_amount < 0 {
                    //println!("Rotate right {}", cut_amount.abs());
                    deck.rotate_right((cut_amount).abs() as usize);
                } else {
                    //println!("Rotate left {}", cut_amount.abs());
                    deck.rotate_left(cut_amount as usize);
                }
            }
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instr> {
    let mut instr = Vec::new();

    for line in input.split("\n") {
        if line.starts_with("increment") {
            let mut inc_line = line.split(" ");
            inc_line.next();
            let inc_amount = inc_line.next().unwrap().parse::<isize>().unwrap();
            instr.push(Instr::Incr(inc_amount));

        } else if line.starts_with("cut") {
            let mut cut_line = line.split(" ");
            cut_line.next();
            let cut_amount = cut_line.next().unwrap().parse::<isize>().unwrap();
            instr.push(Instr::Cut(cut_amount));

        } else if line.starts_with("new stack") {
            instr.push(Instr::NewStack);

        } else {
            panic!(format!("Unexpected line '{}'", line));
        }
    }

    return instr;
}

#[test]
pub fn test_new_stack() {
    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..10 {
        deck.push_back(ix);
    }
    let deck_initial = deck.clone();

    Instr::NewStack.apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

    Instr::NewStack.apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &deck_initial);
}

#[test]
pub fn test_cut() {
    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..10 {
        deck.push_back(ix);
    }
    let deck_initial = deck.clone();

    Instr::Cut(3).apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..10 {
        deck.push_back(ix);
    }

    let mut deck = deck_initial.clone();
    Instr::Cut(-3).apply(&mut deck, &mut scratch);
    Instr::Cut(-3).apply(&mut deck, &mut scratch);

    let two_cuts = deck.clone();

    let mut deck = deck_initial.clone();
    Instr::Cut(-6).apply(&mut deck, &mut scratch);

    assert_eq!(&deck, &two_cuts);
}

#[test]
pub fn test_incr() {
    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..10 {
        deck.push_back(ix);
    }

    Instr::Incr(3).apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
}

fn main() {
    let instrs = parse_input(INPUT[IX]);


    let num_iters: u128 = 101741582076661;
    let num_cards: u128 = 119315717514047;
    let extra_cards = num_cards % 10007;
    let repeats_after: u128 = 5003;
    let remaining_iters = num_iters % repeats_after;

    //println!("repetitions = {}", num_iters / repeats_after);
    //println!("extra iters = {}", num_iters % repeats_after);
    //println!("extra cards = {}", num_cards % 10007);

    
    /*
    {
        let num_cards = 10007;
        let mut deck = VecDeque::with_capacity(num_cards as usize);
        let mut scratch = VecDeque::with_capacity(num_cards as usize);
        for ix in 0..(num_cards as u128) {
            deck.push_back(ix);
        }

        for instr in instrs.iter() {
            instr.apply(&mut deck, &mut scratch);
        }

        println!("start attempt");
        let mut unapplied = Vec::new();
        for ix in 0..num_cards {
            let mut loc = deck[ix as usize] as i128;
            for instr in instrs.iter() {
                loc = instr.unapply(loc, num_cards as i128);
            }
            unapplied.push(loc);
        }

        println!("deck = {:?}", &deck);
        println!("undeck = {:?}", &unapplied);
    }
    */

    let mut loc = 2020;
    let mut first = 0;
    for iter in 0..num_iters {
        for instr in instrs.iter() {
            loc = instr.unapply(loc, num_cards as i128);
        }

        if iter < 5 {
            println!("loc = {:9}", loc);
        }

        if loc < 0 {
            loc = num_cards as i128 - loc;
        }

        if iter % 1000000 == 0 {
            println!("iter = {}", iter);
        }

        if loc == 2020 {
            println!("repeat at {}", iter);
            break;
        }
    }
    println!("Loc = {}", loc);
    return;


    let try_num_cards = 10007; // + extra_cards;
    let mut deck = VecDeque::with_capacity(NUM_CARDS as usize);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS as usize);
    for ix in 0..(try_num_cards as u128) {
        deck.push_back(ix);
    }

    let mut left = HashSet::new();
    for ix in 0..try_num_cards {
        left.insert(ix);
    }
    let mut cycles: Vec<Vec<u128>> = Vec::new();

    for instr in instrs.iter() {
        instr.apply(&mut deck, &mut scratch);
    }

    let mut next_attempt = 0;
    while left.len() > 0 {
        while next_attempt < try_num_cards && !left.contains(&next_attempt) {
            next_attempt += 1;
        }

        if next_attempt == try_num_cards {
            break;
        }

        //println!("next attempt = {}", next_attempt);

        let mut next_ix: u128 = next_attempt;
        let mut cycle = Vec::new();
        while left.contains(&next_ix) {
            //println!("left = {}, cycle = {}", left.len(), cycle.len());
            left.remove(&next_ix);
            cycle.push(next_ix as u128);

            //println!("deck len = {}, next_ix = {}", deck.len(), next_ix);
            next_ix = deck.iter().position(|card| *card == next_ix).unwrap() as u128;
        }
        cycles.push(cycle);
    }
    for (ix, cycle) in cycles.iter().enumerate() {
        println!("cycle[{}] = {}", ix, cycle.len());
    }
}
