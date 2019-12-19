use std::collections::HashMap;


const INPUT: &str =
"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
//"3 DJDNR => 1 ZCMR
//7 VWJH => 5 ZPGT
//5 BHZP => 2 DJDNR
//6 KCNKC, 19 MZWS => 4 PKVJF
//21 GXSHP, 1 TWGP, 3 BGCW => 1 XHRWR
//12 DZGWQ, 2 XRDL, 3 XNVT => 2 FTMC
//7 VWJH, 33 BGCW, 1 TBVC => 9 DSDP
//1 NMTGB, 4 KCNKC, 5 SBSJ, 4 MCZDZ, 7 DLCP, 2 GRBZF, 1 CLKP, 10 VQHJG => 6 DVCR
//7 ZCMR => 9 VNTF
//2 VNTF, 1 GKMN => 1 TZWBH
//6 QMFV, 7 GRBZF => 7 RHDZ
//8 PKVJF => 9 NJQH
//110 ORE => 9 GZTS
//4 DJDNR, 7 SFHV => 8 KQFH
//1 ZTCZ, 5 LZFBP => 7 VWPMZ
//2 GKMN, 6 TZWBH, 1 GXSHP => 1 MJHJH
//2 DLCP, 4 NGJRN => 3 GRBZF
//2 DJDNR, 1 GSRBL => 4 VWJH
//7 RMQX => 3 SFHV
//1 GZTS => 7 GSRBL
//3 GZTS, 1 SFHV => 3 QLXCS
//10 SFHV => 3 MKTHL
//2 DJDNR, 2 BGCW, 4 FSTJ => 3 GKMN
//2 KQFH, 7 GSRBL => 7 TWGP
//22 RHDZ, 22 DZGWQ, 2 NGJRN, 14 XHRWR, 21 VWPMZ, 15 ZPXHM, 26 BHZP => 8 BPHZ
//1 QLXCS => 6 ZBTS
//12 DLCP, 9 DSDP => 9 ZPXHM
//1 VNTF => 5 ZBTX
//2 TZWBH, 2 JCDW => 1 CPLG
//1 XHRWR, 7 FSTJ, 5 DZGWQ => 4 NGJRN
//179 ORE => 3 RMQX
//1 DSDP => 1 MZWS
//140 ORE => 8 BHZP
//1 LZFBP, 4 DZGWQ => 2 PMDK
//1 GZTS => 1 GXSHP
//10 CPLG, 8 MCZDZ => 5 ZTCZ
//5 ZPGT, 4 THLBN, 24 GSRBL, 40 VNTF, 9 DVCR, 2 SHLP, 11 PMDK, 19 BPHZ, 45 NJQH => 1 FUEL
//9 MKTHL => 7 KCNKC
//5 NGJRN => 3 QMFV
//1 ZTCZ, 6 VNTF => 2 VQHJG
//5 FTMC, 5 ZBTX, 1 MJHJH => 1 CLKP
//7 FSTJ => 6 DLCP
//1 DSDP => 5 KTML
//4 LZFBP, 8 MKTHL => 7 MCZDZ
//1 SFHV => 1 DZGWQ
//2 QLXCS => 4 ZMXRH
//3 KQFH, 1 DJDNR => 7 TBVC
//5 DSDP => 7 THLBN
//9 BHZP, 1 VWJH => 6 BGCW
//4 GXSHP => 6 JCDW
//1 KQFH, 3 ZMXRH => 9 XNVT
//6 TBVC => 4 GVMH
//3 VWPMZ, 3 GRBZF, 27 MJHJH, 2 QMFV, 4 NMTGB, 13 KTML => 7 SHLP
//1 GVMH => 2 FSTJ
//2 VQHJG, 2 NJQH => 8 SBSJ
//1 XNVT => 2 XRDL
//2 KCNKC => 5 LZFBP
//2 ZBTS, 8 DLCP => 4 NMTGB";

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

pub fn parse_input(input: &str) -> HashMap<Chem, Vec<Vec<Chem>>> {
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
        if !chem_map.contains_key(&pair.0) {
            chem_map.insert(pair.0.clone(), Vec::new());
        }

        let chem_lists = chem_map.get_mut(&pair.0).unwrap();
        chem_lists.push(pair.1);
    }

    return chem_map;
}

fn main() {
    let chem_map = parse_input(INPUT);


    let mut avail: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    avail.insert("ORE".to_string(), vec!((1, 1)));

    let mut changed = true;
    while changed {
        let chem_avail_names = avail.keys().map(|name| name.clone().to_string()).collect::<Vec<String>>();
        
        changed = false;
        for (chem, input_lists) in chem_map.iter() {
            for inputs in input_lists.iter() {
                let all_avail =
                    inputs.iter()
                          .all(|in_chem| chem_avail_names.iter().any(|avail_name| in_chem.name == *avail_name));

                if all_avail {
                    let mut total_cost = 0;

                    for req in inputs {
                        let mut min_cost = 1000000000;

                        for prod in avail.get(&req.name).unwrap().iter() {
                            let cost;
                            if req.count <= prod.0 {
                                cost = prod.1
                            } else if req.count % prod.0 == 0{
                                cost = prod.1 * (req.count / prod.0);
                            } else {
                                cost = prod.1 * ((req.count / prod.0) + 1);
                            }

                            min_cost = std::cmp::min(min_cost, cost);
                        }

                        total_cost += min_cost;
                    }

                    if !avail.contains_key(&chem.name) {
                        avail.insert(chem.name.clone(), Vec::new());
                    }

                    let new_prod = (chem.count, total_cost);
                    let prod_list = avail.get_mut(&chem.name).unwrap();
                    if !prod_list.contains(&new_prod) {
                        prod_list.push(new_prod);
                        changed = true;
                    }
                }
            }
        }
    }

    dbg!(&avail);

    println!("Total ORE cost: {:?}", avail.get("FUEL").unwrap());
}
