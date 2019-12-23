use std::collections::VecDeque;


const NUM_CARDS: usize = 10007;

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

pub type Card = usize;

pub type Deck = VecDeque<Card>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instr {
    Incr(isize),
    NewStack,
    Cut(isize),
}

impl Instr {
    pub fn apply(self, deck: &mut Deck, scratch: &mut Deck) {
        match self {
            Instr::Incr(inc_amount) => {
                //println!("Increment {}", inc_amount);

                scratch.clear();
                for ix in 0..NUM_CARDS {
                    scratch.push_back(0);
                }
                for ix in 0..NUM_CARDS {
                    let mut pos = ix as isize * inc_amount;
                    let pos = pos as usize % NUM_CARDS;
                    scratch[pos] = deck[ix];
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

    Instr::NewStack.apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
pub fn test_cut() {
    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..10 {
        deck.push_back(ix);
    }

    Instr::Cut(3).apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..10 {
        deck.push_back(ix);
    }

    Instr::Cut(-3).apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
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

    dbg!(&instrs);

    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..NUM_CARDS {
        deck.push_back(ix);
    }

    for instr in instrs {
        //dbg!(&deck);
        instr.apply(&mut deck, &mut scratch);
    }

    dbg!(&deck);

    let card_pos = deck.iter().position(|card| *card == 2019).unwrap();

    println!("Answer = {}", card_pos);
}
