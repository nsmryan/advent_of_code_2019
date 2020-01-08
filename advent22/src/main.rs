use std::boxed::Box;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

use modinverse::modinverse;
use num::pow::pow;

// cut c = p - c
// new stack = N - p - 1
// increment i = p * i


// cut is the group Z
// cut n; cut m = cut (n + m) = p + (n + m)

// increment n = N - n * p
// increment n; increment m = N - (m * (N - n * p % N) % N)

// new stack; cut n; new stack = cut -n
// we end up cutting in the opposite direction when flipping between cuts

// increment m; new stack = new stack; increment (N-m)

// cut n; increment m = (N - m) * (p - ((N - m) * n))
// increment m; cut n = ((N - m) * p) - n

const NUM_CARDS: usize = 119315717514047;
//const NUM_CARDS: usize = 10007;
//const NUM_ITERS: u128 = 1;
const NUM_ITERS: u128 = 101741582076661;

const IX: usize = 4;


const INPUT: [&str; 5] = [
// 0 3 6 9 2 5 8 1 4 7
"increment 7
new stack
new stack",

// (p + 6) * 7 * -1 + 10 + 1
// -7 * p + 11
// 3 0 7 4 1 8 5 2 9 6
"cut 6
increment 7
new stack",

// (p * 7) * 9 - 2
// p * 3 + 2
// 1 * 3 + 2 = 5
// 5 * 3 + 2 = 7
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

pub type Card = i128;

pub type Deck = VecDeque<Card>;


#[derive(Clone, PartialEq, Debug)]
pub enum Op {
    Var,
    Prim(Card),
    Add(Box<Op>, Box<Op>),
    Mult(Box<Op>, Box<Op>),
}

impl Op {
    pub fn add(op1: Op, op2: Op) -> Op {
        return Op::Add(Box::new(op1), Box::new(op2));
    }

    pub fn mult(op1: Op, op2: Op) -> Op {
        return Op::Mult(Box::new(op1), Box::new(op2));
    }

    pub fn depth(&self) -> usize {
        match self {
            Op::Add(op1, op2) => {
                let d1 = op1.depth();
                let d2 = op2.depth();

                return 1 + std::cmp::max(d1, d2);
            }

            Op::Mult(op1, op2) => {
                let d1 = op1.depth();
                let d2 = op2.depth();

                return 1 + std::cmp::max(d1, d2);
            }

            Op::Prim(prim) => {
                return 1;
            }

            Op::Var => {
                return 1;
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Op::Add(op1, op2) => {
                let d1 = op1.size();
                let d2 = op2.size();

                return 1 + d1 + d2;
            }

            Op::Mult(op1, op2) => {
                let d1 = op1.size();
                let d2 = op2.size();

                return 1 + d1 + d2;
            }

            Op::Prim(prim) => {
                return 1;
            }

            Op::Var => {
                return 1;
            }
        }
    }

    pub fn simplify(&self) -> Op {
        let mut new_op = self.clone();
        let mut old_op = new_op.clone();

        match self {
            Op::Add(op1, op2) => {
                let new_op1 = op1.simplify();
                let new_op2 = op2.simplify();

                new_op = Op::add(new_op1, new_op2);

                if let Some(simplified) = new_op.simplify_op() {
                    new_op = simplified;
                }
            }

            Op::Mult(op1, op2) => {
                let new_op1 = op1.simplify();
                let new_op2 = op2.simplify();

                new_op = Op::mult(new_op1, new_op2);

                if let Some(simplified) = new_op.simplify_op() {
                    new_op = simplified;
                }
            }

            Op::Prim(prim) => {
                return Op::Prim(*prim);
            }

            Op::Var => {
                return Op::Var;
            }
        }

        return new_op;
    }

    pub fn simplify_op(&self) -> Option<Op> {
        match self.clone() {
            Op::Prim(card) => {
                return None;
            }

            Op::Add(op1, op2) => {
                match (*op1.clone(), *op2.clone()) {
                    (op, Op::Var) => {
                        return Some(Op::add(Op::Var, op));
                    }

                    (Op::Prim(p1), Op::Prim(p2)) => {
                        return Some(Op::Prim(p1 + p2));
                    }

                    (Op::Prim(p1), Op::Add(left, right)) => {
                        match *left {
                            Op::Prim(p2) => {
                                return Some(Op::add(Op::Prim(p1 + p2), *right.clone()));
                            }
                            _ => {
                                match *right {
                                    Op::Prim(p2) => {
                                        return Some(Op::add(Op::Prim(p1 + p2), *left.clone()));
                                    }
                                    _ => return None,
                                }
                            }
                        }
                    }

                    (Op::Add(left, right), Op::Prim(p1)) => {
                        match *left {
                            Op::Prim(p2) => {
                                return Some(Op::add(Op::Prim(p1 + p2), *right.clone()));
                            }
                            _ => {
                                match *right {
                                    Op::Prim(p2) => {
                                        return Some(Op::add(Op::Prim(p1 + p2), *left.clone()));
                                    }
                                    _ => return None,
                                }
                            }
                        }
                    }

                    _ => {
                        return None;
                    }
                }
            }

            Op::Mult(op1, op2) => {
                match (*op1.clone(), *op2.clone()) {
                    (op, Op::Var) => {
                        return Some(Op::Mult(Box::new(Op::Var), Box::new(op)));
                    }

                    (Op::Prim(p1), Op::Prim(p2)) => {
                        return Some(Op::Prim(p1 * p2));
                    }

                    (Op::Prim(p), Op::Add(left, right)) => {
                        return Some(Op::add(left.scale(p), right.scale(p)));
                    }

                    (Op::Add(left, right), Op::Prim(p)) => {
                        return Some(Op::add(left.scale(p), right.scale(p)));
                    }

                    _ => {
                        return None;
                    }
                }
            }

            Op::Var => {
                return None;
            }
        }
    }

    pub fn scale(&self, prim: Card) -> Op {
        match self.clone() {
            Op::Prim(card) => {
                return Op::Prim(modulus(card * prim, NUM_CARDS as Card));
            }

            Op::Add(op1, op2) => {
                return Op::add(op1.scale(prim), op2.scale(prim));
            }

            Op::Mult(op1, op2) => {
                return Op::mult(*op1, op2.scale(prim));
            }

            Op::Var => {
                return Op::mult(Op::Var, Op::Prim(prim));
            }
        }
    }

    pub fn eval(&self, prim: Card) -> Card {
        match self {
            Op::Prim(card) => {
                return *card;
            }

            Op::Add(op1, op2) => {
                return modulus((op1.eval(prim) + op2.eval(prim)), NUM_CARDS as Card);
            }

            Op::Mult(op1, op2) => {
                return modulus((op1.eval(prim) * op2.eval(prim)), NUM_CARDS as Card);
            }

            Op::Var => {
                return prim;
            }
        }
    }

    pub fn substitute(&self, other: &Op) -> Op {
        match self {
            Op::Prim(card) => {
                return self.clone();
            }

            Op::Add(op1, op2) => {
                return Op::add(op1.substitute(other), op2.substitute(other));
            }

            Op::Mult(op1, op2) => {
                return Op::mult(op1.substitute(other), op2.substitute(other));
            }

            Op::Var => {
                return other.clone();
            }
        }
    }

    pub fn print(&self) {
        self.print_helper();
        println!("");
    }

    fn print_helper(&self) {
        match self {
            Op::Prim(card) => {
                print!("{}", card);
            }

            Op::Add(op1, op2) => {
                print!("(+ ");
                op1.print_helper();
                print!(" ");
                op2.print_helper();
                print!(")");
            }

            Op::Mult(op1, op2) => {
                print!("(* ");
                op1.print_helper();
                print!(" ");
                op2.print_helper();
                print!(")");
            }

            Op::Var => {
                print!("x");
            }
        }
    }
}

#[test]
pub fn test_simplify() {
    let expr = Op::add(Op::Var, Op::Prim(2));
    assert_eq!(expr, expr.simplify());

    let expr = Op::add(Op::Prim(3), Op::Prim(2));
    assert_eq!(Op::Prim(5), expr.simplify());

    let expr = Op::mult(Op::Prim(3), Op::Prim(2));
    assert_eq!(Op::Prim(6), expr.simplify());

    let expr = Op::mult(Op::Prim(3), Op::Var);
    assert_eq!(Op::mult(Op::Var, Op::Prim(3)), expr.simplify());

    let expr = Op::mult(Op::Prim(3), Op::add(Op::Prim(4), Op::Prim(5)));
    assert_eq!(Op::Prim(27), expr.simplify());

    let expr = Op::mult(Op::Prim(3), Op::add(Op::Var, Op::Prim(5)));
    assert_eq!(Op::add(Op::mult(Op::Var, Op::Prim(3)), Op::Prim(15)), expr.simplify());
}

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

pub fn parse_input_as_ops(input: &str) -> Op {
    let mut op = Op::Var;

    for line in input.split("\n") {
        if line.starts_with("increment") {
            let mut inc_line = line.split(" ");
            inc_line.next();
            let inc_amount = inc_line.next().unwrap().parse::<Card>().unwrap();

            op = Op::Mult(Box::new(Op::Prim(inc_amount)), Box::new(op));

        } else if line.starts_with("cut") {
            let mut cut_line = line.split(" ");
            cut_line.next();
            let cut_amount = cut_line.next().unwrap().parse::<Card>().unwrap();

            op = Op::Add(Box::new(Op::Prim(NUM_CARDS as Card - cut_amount)), Box::new(op));

        } else if line.starts_with("new stack") {
            op = Op::add(Op::Prim(NUM_CARDS as Card - 1), Op::mult(Op::Prim(-1), op));

        } else {
            panic!(format!("Unexpected line '{}'", line));
        }
    }

    return op;
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
    let orig_op = parse_input_as_ops(INPUT[IX]);

    println!("Original op(10) = {}", orig_op.eval(10));

    orig_op.print();
    println!("Original size = {}", orig_op.size());
    let op = orig_op.simplify();
    op.print();
    println!("Final size = {}", op.size());

    assert_eq!(orig_op.eval(1), op.eval(1));
    assert_eq!(orig_op.eval(10), op.eval(10));
    assert_eq!(orig_op.eval(10000), op.eval(10000));
    assert_eq!(orig_op.eval(0), op.eval(0));
    assert_eq!(orig_op.eval(2), op.eval(2));

    let answer = op.eval(2019);
    println!("Answer = {}", answer);
    //let add = 7389;
    //let mult = 1688;

    let add: Card = 109167937990320;
    let mult: Card = 35481670129518;

    let card_pos = 2020;
    //let card_pos = 3074;

    let unadd = NUM_CARDS as Card - add;
    let unmult = modinverse(mult, NUM_CARDS as Card).unwrap();

    println!("unmult = {}", unmult);
    println!("mult * unmult = {}", modulus(mult * unmult, NUM_CARDS as Card));

    assert_eq!(card_pos, modulus((op.eval(card_pos) + unadd) * unmult, NUM_CARDS as Card));
    let twice = op.eval(op.eval(card_pos));
    let unonce = modulus((twice + unadd) * unmult, NUM_CARDS as Card);
    let untwice = modulus((unonce + unadd) * unmult, NUM_CARDS as Card);
    assert_eq!(card_pos, untwice);

    let unop = Op::mult(Op::add(Op::Var, Op::Prim(unadd)), Op::Prim(unmult));

    println!("Substitutions");
    let chunk_size = 1000000;
    let mut subs = Op::Var;
    let mut test_val = card_pos;
    for ix in 0..chunk_size {
        test_val = op.eval(test_val);
        subs = subs.substitute(&unop);
        subs = subs.simplify();
    }
    assert_eq!(card_pos, subs.eval(test_val));
    subs = subs.simplify();
    assert_eq!(card_pos, subs.eval(test_val));
    dbg!(&subs);

    assert_eq!(card_pos, unop.eval(op.eval(card_pos)));
    assert_eq!(card_pos, unop.eval(unop.eval(op.eval(op.eval(card_pos)))));

    println!("Start main loop");
    let mut val = card_pos;
    let mut remaining_iters = NUM_ITERS;
    let mut exec_iters: u128 = 0;
    while remaining_iters > 0 {
        if exec_iters % 1000000000000 == 0 {
            println!("Ix = {} trillions", exec_iters / 1000000000000);
        }

        if remaining_iters > chunk_size {
            val = subs.eval(val);
            remaining_iters -= chunk_size;
            exec_iters += chunk_size;
        } else {
            val = unop.eval(val);
            remaining_iters -= 1;
            exec_iters += 1;
        }
    }

    println!("Answer = {}", val);
}
