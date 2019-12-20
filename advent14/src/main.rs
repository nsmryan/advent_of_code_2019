use std::collections::HashMap;
use std::time::{Duration, Instant};


const INPUTS: [&str; 6] = [
// example (11312)
"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
// example (2210736)
"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
// example (180697)
"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
// example (31)
"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
//// example (165)
"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
// solution input
"3 DJDNR => 1 ZCMR
7 VWJH => 5 ZPGT
5 BHZP => 2 DJDNR
6 KCNKC, 19 MZWS => 4 PKVJF
21 GXSHP, 1 TWGP, 3 BGCW => 1 XHRWR
12 DZGWQ, 2 XRDL, 3 XNVT => 2 FTMC
7 VWJH, 33 BGCW, 1 TBVC => 9 DSDP
1 NMTGB, 4 KCNKC, 5 SBSJ, 4 MCZDZ, 7 DLCP, 2 GRBZF, 1 CLKP, 10 VQHJG => 6 DVCR
7 ZCMR => 9 VNTF
2 VNTF, 1 GKMN => 1 TZWBH
6 QMFV, 7 GRBZF => 7 RHDZ
8 PKVJF => 9 NJQH
110 ORE => 9 GZTS
4 DJDNR, 7 SFHV => 8 KQFH
1 ZTCZ, 5 LZFBP => 7 VWPMZ
2 GKMN, 6 TZWBH, 1 GXSHP => 1 MJHJH
2 DLCP, 4 NGJRN => 3 GRBZF
2 DJDNR, 1 GSRBL => 4 VWJH
7 RMQX => 3 SFHV
1 GZTS => 7 GSRBL
3 GZTS, 1 SFHV => 3 QLXCS
10 SFHV => 3 MKTHL
2 DJDNR, 2 BGCW, 4 FSTJ => 3 GKMN
2 KQFH, 7 GSRBL => 7 TWGP
22 RHDZ, 22 DZGWQ, 2 NGJRN, 14 XHRWR, 21 VWPMZ, 15 ZPXHM, 26 BHZP => 8 BPHZ
1 QLXCS => 6 ZBTS
12 DLCP, 9 DSDP => 9 ZPXHM
1 VNTF => 5 ZBTX
2 TZWBH, 2 JCDW => 1 CPLG
1 XHRWR, 7 FSTJ, 5 DZGWQ => 4 NGJRN
179 ORE => 3 RMQX
1 DSDP => 1 MZWS
140 ORE => 8 BHZP
1 LZFBP, 4 DZGWQ => 2 PMDK
1 GZTS => 1 GXSHP
10 CPLG, 8 MCZDZ => 5 ZTCZ
5 ZPGT, 4 THLBN, 24 GSRBL, 40 VNTF, 9 DVCR, 2 SHLP, 11 PMDK, 19 BPHZ, 45 NJQH => 1 FUEL
9 MKTHL => 7 KCNKC
5 NGJRN => 3 QMFV
1 ZTCZ, 6 VNTF => 2 VQHJG
5 FTMC, 5 ZBTX, 1 MJHJH => 1 CLKP
7 FSTJ => 6 DLCP
1 DSDP => 5 KTML
4 LZFBP, 8 MKTHL => 7 MCZDZ
1 SFHV => 1 DZGWQ
2 QLXCS => 4 ZMXRH
3 KQFH, 1 DJDNR => 7 TBVC
5 DSDP => 7 THLBN
9 BHZP, 1 VWJH => 6 BGCW
4 GXSHP => 6 JCDW
1 KQFH, 3 ZMXRH => 9 XNVT
6 TBVC => 4 GVMH
3 VWPMZ, 3 GRBZF, 27 MJHJH, 2 QMFV, 4 NMTGB, 13 KTML => 7 SHLP
1 GVMH => 2 FSTJ
2 VQHJG, 2 NJQH => 8 SBSJ
1 XNVT => 2 XRDL
2 KCNKC => 5 LZFBP
2 ZBTS, 8 DLCP => 4 NMTGB"
];

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Chem {
    name: String,
    count: usize,
}

impl Chem {
    pub fn from_str(string: String) -> Chem {
        let trimmed = string.trim();
        let mut split = trimmed.split_whitespace();
        let count = split.next().unwrap().parse::<usize>().unwrap();
        let name = split.next().unwrap().to_string();

        return Chem { name, count };
    }
}

pub fn parse_input(input: &str) -> HashMap<Chem, Vec<Chem>> {
    let mut chem_map = HashMap::new();

    let pairs =
        input.split("\n")
         .map(|line| {
             let mut elems = line.split("=>");
             let head = elems.next().unwrap();
             let tail = elems.next().unwrap();

             let out_chem = Chem::from_str(tail.to_string());

             let mut pairs = head.split(",");
             let mut inputs = Vec::new();
             while let Some(pair) = pairs.next() {
                 inputs.push(Chem::from_str(pair.to_string()));
             }

             return (out_chem, inputs);
         })
         .collect::<Vec<(Chem, Vec<Chem>)>>();

    for pair in pairs {
        chem_map.insert(pair.0.clone(), pair.1);
    }

    return chem_map;
}

fn main() {
    let chem_map = parse_input(INPUTS[2]);

    println!("{} total productions", chem_map.keys().len());

    let mut productions = HashMap::new();
    for (chem, input) in chem_map {
        productions.insert(chem.name.clone(), (chem.count, input));
    }

    let mut requirements: HashMap<String, usize> = HashMap::new();
    let mut leftovers: HashMap<String, usize> = HashMap::new();

    requirements.insert("FUEL".to_string(), 1);
    let mut ore_cost = 0;

    while requirements.len() > 0 {
        println!("Starting loop");
        let mut new_reqs = HashMap::new();

        let mut req_vec = requirements.iter().map(|pair| (pair.0.clone(), pair.1.clone())).collect::<Vec<(String, usize)>>();

        for (req_name, req_count_needed) in req_vec.iter() {
            if let Some((chem_count, input)) = productions.get(req_name) {
                let mut count_needed = *req_count_needed;
                if leftovers.contains_key(req_name) {
                    let left_count = leftovers.get_mut(req_name).unwrap();

                    if count_needed > *left_count {
                        count_needed -= *left_count;
                        leftovers.remove(req_name);
                    } else if count_needed < *left_count {
                        *left_count -= count_needed;
                        continue;
                    } else {
                        leftovers.remove(req_name);
                        continue;
                    }
                }

                let mut prod_times = count_needed / chem_count;
                if prod_times * chem_count < count_needed {
                    prod_times += 1;
                }

                let leftover = prod_times * chem_count - count_needed;
                if leftover > 0 {
                    leftovers.insert(req_name.clone(), leftover);
                }

                println!("Discharging {}, prod {} times", req_name, prod_times);
                for in_chem in input.iter() {
                    if !new_reqs.contains_key(&in_chem.name) {
                        new_reqs.insert(in_chem.name.clone(), 0);
                    }
                    let count = new_reqs.get_mut(&in_chem.name).unwrap();
                    *count += in_chem.count * prod_times;
                }
            } else {
                if req_name != "ORE" {
                    panic!();
                }
                ore_cost += req_count_needed;
            }
        }

        requirements = new_reqs;
    }

    println!("Leftovers {:?}", &leftovers);

    let leftover_per_iter = leftovers.clone();

    let mut ore_left: i64 = 1000000000000;
    let mut num_fuel = 0;
    while ore_left > 0 {
        num_fuel += 1;
        if ore_left >= ore_cost as i64 {
            ore_left -= ore_cost as i64;

            for (name, count) in leftover_per_iter.iter() {
                *leftovers.get_mut(name).unwrap() += count;
            }
        } else if ore_left < ore_cost as i64 {
            break;
        }
    }

    println!("Ore cost: {}", ore_cost);
    println!("Fuel made: {}", num_fuel);
}
