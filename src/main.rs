#![feature(inclusive_range_syntax)]

use std::collections::{HashSet, HashMap};
use std::collections::hash_map;

mod graph;
mod grammer;

use graph::*;
use grammer::{Grammer, Nonterminal};

fn nullable(grammers: &[Grammer], result: &mut HashSet<Nonterminal>) {
    let original_len = result.len();

    for g in grammers {
        if g.right.iter().all(|c| {
            if let &grammer::Character::Nonterminal(ref n) = c {
                result.contains(n)
            } else {
                false
            }
        }) {
            result.insert(g.left.clone());
        }
    }

    if result.len() != original_len {
        nullable(grammers, result);
    }
}

fn add_char(result: &mut HashMap<Nonterminal, HashSet<char>>, n: Nonterminal, c: char) -> bool {
    use hash_map::Entry::*;

    match result.entry(n.clone()) {
        Occupied(mut o) => {
            if !o.get().contains(&c) {
                o.get_mut().insert(c);
                return true;
            }
        },
        Vacant(v) => {
            v.insert(HashSet::new()).insert(c);
            return true;
        }
    }
    false
}

fn first(grammers: &[Grammer], nullable: &HashSet<Nonterminal>, result: &mut HashMap<Nonterminal, HashSet<char>>) {
    let mut dirty = false;

    for g in grammers {
        for c in g.right.iter() {
            use grammer::Character::*;

            match *c {
                Terminal(c) => {
                    dirty = dirty || add_char(result, g.left.clone(), c);
                    break;
                },
                Nonterminal(ref n) => {
                    let firsts = result.get(n).map(|m| m.clone());
                    if let Some(firsts) = firsts {
                        for c in firsts {
                            dirty = dirty || add_char(result, g.left.clone(), c);
                        }
                    }

                    if !nullable.contains(n) {
                        break;
                    }
                }
            }
        }
    }

    if dirty {
        first(grammers, nullable, result);
    }
}

fn follow(grammers: &[Grammer], nullable: &HashSet<Nonterminal>, first: &HashMap<Nonterminal, HashSet<char>>, result: &mut HashMap<Nonterminal, HashSet<char>>) {
    let mut dirty = false;

    for g in grammers {
        for i in 0..g.right.len() {
            use grammer::Character::*;

            if let Nonterminal(ref target) = g.right[i] {
                let mut reach_end = true;
                for j in (i + 1)..g.right.len() {
                    match g.right[j] {
                        Terminal(c) => {
                            reach_end = j == g.right.len() - 1;
                            dirty = dirty || add_char(result, target.clone(), c);
                            break;
                        },
                        Nonterminal(ref n) => {
                            if let Some(firsts) = first.get(n) {
                                for &c in firsts {
                                    dirty = dirty || add_char(result, target.clone(), c);
                                }
                            }

                            if !nullable.contains(n) {
                                reach_end = j == g.right.len() - 1;
                                break;
                            }
                        }
                    }
                }

                if reach_end {
                    let firsts = result.get(&g.left).map(|m| m.clone());
                    if let Some(firsts) = firsts {
                        for c in firsts {
                            dirty = dirty || add_char(result, target.clone(), c);
                        }
                    }
                }
            }
        }
    }

    if dirty {
        follow(grammers, nullable, first, result);
    }
}

fn insert_dots(grammers: Vec<Grammer>) -> Vec<Grammer> {
    let mut ret = vec![];
    for mut g in grammers.into_iter() {
        for i in 0..=g.right.len() {
            g.dot_pos = Some(i);
            ret.push(g.clone());
        }
    }
    ret
}

fn closure(g: Grammer, grammers: &[Grammer]) -> HashSet<Grammer> {
    let p = g.dot_pos.unwrap();
    if p == g.right.len() {
        let mut ret = HashSet::new();
        ret.insert(g);
        ret
    } else {
        use grammer::Character::*;
        match g.right[p] {
            Terminal(_) => {
                let mut ret = HashSet::new();
                ret.insert(g.clone()); // TODO: NLL
                ret
            },
            Nonterminal(ref n) => {
                let mut ret = HashSet::new();
                // TODO: improve searching
                for gg in grammers {
                    if gg.left == *n && gg.dot_pos.unwrap() == 0 {
                        ret.extend(closure(gg.clone(), grammers).into_iter());
                    }
                }
                ret.insert(g.clone()); // TODO: NLL
                ret
            },
        }
    }
}

fn main() {
    let s: Nonterminal = "S".to_string().into();
    let e: Nonterminal = "E".to_string().into();
    let grammers = 
        vec![
            Grammer {
                left: s.clone(),
                right: vec![e.clone().into(), '+'.into(), e.clone().into()],
                dot_pos: None,
            },
            Grammer {
                left: e.clone(),
                right: vec!['a'.into(), e.clone().into()],
                dot_pos: None,
            },
            Grammer {
                left: e.clone(),
                right: vec![],
                dot_pos: None,
            }
        ];
    let start = {
        let mut start = grammers[0].clone();
        start.dot_pos = Some(0);
        start
    };

    let null = {
        let mut null = HashSet::new();
        nullable(&grammers, &mut null);
        null
    };

    let firsts = {
        let mut firsts = HashMap::new();
        first(&grammers, &null, &mut firsts);
        firsts
    };

    let follows = {
        let mut follows = HashMap::new();
        follow(&grammers, &null, &firsts, &mut follows);
        follows
    };

    for grammer in grammers.iter() {
        println!("{}", grammer);
    }

    let grammers = insert_dots(grammers);
    for g in grammers.iter() {
        println!("{}", g);
    }

    for g in grammers.iter() {
        let closure = closure(g.clone(), &grammers);
        println!("{}", g);
        for gg in closure {
            println!("  {}", gg);
        }
    }

    let mut arena = Arena::new();
    arena.push(closure(start, &grammers));

    let mut done = 0;
    while done < arena.nodes.len() {
        for g in arena.nodes[done].value.clone().iter() {
            let pos = g.dot_pos.unwrap();

            if pos < g.right.len() {
                let mut gg = (*g).clone();
                gg.dot_pos = Some(pos + 1);

                // TODO: cache result of closure()
                let cls = closure(gg, &grammers);

                let mut found = false;
                for i in 0..arena.nodes.len() {
                    if arena.nodes[i].value == cls {
                        arena.edges[done].insert(g.right[pos].clone(), i);
                        found = true;
                        break;
                    }
                }

                if !found {
                    arena.push(cls);
                    arena.edges[done].insert(g.right[pos].clone(), arena.nodes.len() - 1);
                }
            }
        }
        done += 1;
    }

    for n in arena.nodes.iter() {
        println!("----------");
        println!("{}", n.id);
        for g in n.value.iter() {
            println!("{}", g);
        }
        println!();
        for (ref c, to) in arena.edges[n.id].iter() {
            println!("{:?} => {}", c, to);
        }
    }
    println!("----------");

    println!("{:?}", null);
    println!("{:?}", firsts);
    println!("{:?}", follows);
}

