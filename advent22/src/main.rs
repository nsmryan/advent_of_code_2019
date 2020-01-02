use std::collections::VecDeque;
use std::collections::HashSet;

// cut is the group Z
// cut n; cut m = cut (n + m) = p + (n + m)

// increment n = N - n * p
// increment n; increment m = N - (m * (N - n * p % N) % N)

// new stack; cut n; new stack = cut -n
// we end up cutting in the opposite direction when flipping between cuts

// increment m; new stack = new stack; increment (N-m)

// cut n; increment m = (N - m) * (p - ((N - m) * n))
// increment m; cut n = ((N - m) * p) - n

const NUM_CARDS: usize = 10007; //119315717514047;
const NUM_ITERS: u128 = 101741582076661;

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

pub type Card = i64;

pub type Deck = VecDeque<Card>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shuffle {
    num_cards: usize,
    mult: Card,
    sum: Card,
}

impl Shuffle {
    pub fn id(num_cards: usize) -> Shuffle {
        return Shuffle {
            num_cards,
            mult: 1,
            sum: 0,
        };
    }

    pub fn from_instr(num_cards: usize, instr: &Instr) -> Shuffle {
        match instr {
            Instr::Incr(incr) => Shuffle::incr(num_cards, *incr),
            Instr::NewStack => panic!("New stack not supported!"),
            Instr::Cut(cut) => Shuffle::cut(num_cards, *cut),
        }
    }

    pub fn incr(num_cards: usize, incr: Card) -> Shuffle {
        return Shuffle {
            num_cards,
            mult: incr,
            sum: 0,
        };
    }

    pub fn cut(num_cards: usize, cut: Card) -> Shuffle {
        return Shuffle {
            num_cards,
            mult: 1,
            sum: num_cards as Card - cut,
        };
    }

    pub fn combine(&self, other: &Shuffle) -> Shuffle {
       let mult = modulus(self.mult * other.mult, self.num_cards as Card);
       let sum = modulus(self.sum * other.mult + other.sum, self.num_cards as Card);

        return Shuffle {
            num_cards: self.num_cards,
            mult,
            sum,
        };
    }

    pub fn apply(&self, pos: Card) -> Card {
        let mut new_pos = modulus(pos * self.mult + self.sum, self.num_cards as Card);
        //let mut new_pos = modulus((pos + self.sum) * self.mult, self.num_cards as Card);

        return new_pos;
    }

    pub fn apply_deck(&self, deck: &mut Deck) {
        let old_deck = deck.clone();

        for ix in 0..deck.len() {
            let new_ix = self.apply(ix as Card) as usize;
            deck[new_ix] = old_deck[ix];
        }
    }
}

#[test]
pub fn test_cut_shuffle_card() {
    let num_cards = 10;
    let mut shuffle = Shuffle::cut(num_cards, 6);

    assert_eq!(shuffle.apply(0), 4);
    assert_eq!(shuffle.apply(1), 5);
    assert_eq!(shuffle.apply(2), 6);
    assert_eq!(shuffle.apply(3), 7);
}

#[test]
pub fn test_cut_shuffle() {
    let num_cards = 10;
    let mut shuffle = Shuffle::cut(num_cards, 6);

    let mut deck = VecDeque::new();
    for ix in 0..10 {
        deck.push_back(ix);
    }

    shuffle.apply_deck(&mut deck);

    assert_eq!(&deck, &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
}

#[test]
pub fn test_incr_shuffle_card() {
    let num_cards = 10;
    let mut shuffle = Shuffle::incr(num_cards, 3);

    assert_eq!(shuffle.apply(0), 0);
    assert_eq!(shuffle.apply(1), 3);
    assert_eq!(shuffle.apply(2), 6);
    assert_eq!(shuffle.apply(3), 9);
    assert_eq!(shuffle.apply(4), 2);
}

#[test]
pub fn test_incr_shuffle() {
    let num_cards = 10;
    let mut shuffle = Shuffle::incr(num_cards, 3);

    let mut deck = VecDeque::new();
    for ix in 0..10 {
        deck.push_back(ix);
    }

    shuffle.apply_deck(&mut deck);

    dbg!(shuffle);

    assert_eq!(&deck, &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
}

/*
#[test]
pub fn test_small_shuffle() {
    let num_cards = 10;
    let mut shuffle = Shuffle::cut(num_cards, 6);
    dbg!(shuffle);
    shuffle = shuffle.combine(&Shuffle::incr(num_cards, 3));

    let mut deck = VecDeque::new();
    for ix in 0..10 {
        deck.push_back(ix);
    }

    shuffle.apply_deck(&mut deck);

    dbg!(shuffle);

    assert_eq!(8, shuffle.apply(0));
    assert_eq!(&deck, &[6, 9, 2, 5, 8, 1, 4, 7, 0, 3]);
}
*/


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instr {
    Incr(Card),
    NewStack,
    Cut(Card),
}

impl Instr {
    pub fn unapply(self, loc: i64, num_cards: i64) -> i64 {
        match self {
            Instr::Incr(inc_amount) => {
                (loc * inc_amount as i64 % num_cards) 
            }

            Instr::NewStack => {
                num_cards - loc - 1
            }

            Instr::Cut(cut_amount) => {
                let cut_loc = loc + (-1 as i64 * cut_amount as i64);
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
                scratch.clear();
                for ix in 0..deck.len() {
                    scratch.push_back(0);
                }

                for ix in 0..deck.len() {
                    let mut pos = ix as Card * inc_amount;
                    let pos = pos as u128 % deck.len() as u128;
                    scratch[pos as usize] = deck[ix as usize];
                }

                for (ix, card) in scratch.iter().enumerate() {
                    deck[ix] = *card;
                }
            }

            Instr::NewStack => {
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
                    deck.rotate_right((cut_amount).abs() as usize);
                } else {
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
            let inc_amount = inc_line.next().unwrap().parse::<Card>().unwrap();
            instr.push(Instr::Incr(inc_amount));

        } else if line.starts_with("cut") {
            let mut cut_line = line.split(" ");
            cut_line.next();
            let cut_amount = cut_line.next().unwrap().parse::<Card>().unwrap();
            instr.push(Instr::Cut(cut_amount));

        } else if line.starts_with("new stack") {
            instr.push(Instr::NewStack);

        } else {
            panic!(format!("Unexpected line '{}'", line));
        }
    }

    return instr;
}

pub fn modulus(card: Card, num_cards: Card) -> Card {
    let moded = card % num_cards;

    if moded < 0 {
        return moded + num_cards;
    }

    return moded;
}

#[test]
pub fn test_mod() {
    assert_eq!(1, modulus(1, 10));
    assert_eq!(9, modulus(-1, 10));
    assert_eq!(9, modulus(-11, 10));
    assert_eq!(1, modulus(11, 10));
    assert_eq!(1, modulus(21, 10));
    assert_eq!(9, modulus(-21, 10));
    assert_eq!(3, modulus(3, 10));
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

#[test]
pub fn test_incr_invert() {
    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..7 {
        deck.push_back(ix);
    }
    let orig_deck = deck.clone();

    Instr::Incr(3).apply(&mut deck, &mut scratch);
    Instr::Incr(5).apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &orig_deck);

    let mut deck = VecDeque::with_capacity(NUM_CARDS);
    let mut scratch = VecDeque::with_capacity(NUM_CARDS);
    for ix in 0..7 {
        deck.push_back(ix);
    }
    let orig_deck = deck.clone();

    Instr::Incr(2).apply(&mut deck, &mut scratch);
    Instr::Incr(4).apply(&mut deck, &mut scratch);
    assert_eq!(&deck, &orig_deck);
}

fn main() {
    let mut instrs = parse_input(INPUT[IX]);

    let mut any_change = true;
    while any_change {
        any_change = false;

        for ix in 0..(instrs.len() - 1) {
            match (instrs[ix], instrs[ix + 1]) {
                (Instr::NewStack, Instr::NewStack) => {
                    instrs.remove(ix);
                    instrs.remove(ix);
                    any_change = true;
                    break;
                }

                (Instr::NewStack, Instr::Incr(incr)) => {
                    instrs[ix] = Instr::Incr(NUM_CARDS as Card - incr);
                    instrs[ix+1] = Instr::NewStack;
                    any_change = true;
                    break;
                }

                (Instr::NewStack, Instr::Cut(cut)) => {
                    instrs[ix] = Instr::Cut(-1 * cut);
                    instrs[ix+1] = Instr::NewStack;
                    any_change = true;
                    break;
                }

                _ => { }
            }
        }
    }

    let mut shuffle = Shuffle::id(NUM_CARDS);
    for instr in instrs.iter() {
        shuffle = shuffle.combine(&Shuffle::from_instr(NUM_CARDS, instr));
    }

    dbg!(shuffle);

    println!("Answer? {}", shuffle.apply(2019));
    println!("Answer? {}", shuffle.apply(3074));
}
