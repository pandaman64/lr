#![feature(inclusive_range_syntax)]

use std::fmt::{Formatter, Display};
use std::collections::{HashSet, HashMap};
use std::collections::hash_map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Nonterminal(String);

impl From<String> for Nonterminal {
    fn from(c: String) -> Self {
        Nonterminal(c.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Character {
    Terminal(char),
    Nonterminal(Nonterminal)
}

impl From<char> for Character {
    fn from(c: char) -> Self {
        Character::Terminal(c)
    }
}

impl<T: Into<Nonterminal>> From<T> for Character {
    fn from(c: T) -> Self {
        Character::Nonterminal(c.into())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Grammer {
    left: Nonterminal,
    right: Vec<Character>,
    dot_pos: Option<usize>,
}

impl Display for Grammer {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.left.0)?;
        write!(f, " -> ")?;
        for (c, n) in self.right.iter().zip(0..self.right.len()) {
            if let Some(k) = self.dot_pos {
                if k == n {
                    write!(f, "・")?;
                }
            }
            match c {
                &Character::Terminal(c) => write!(f, "{}", c)?,
                &Character::Nonterminal(ref n) => write!(f, "{}", n.0)?
            }
        }
        if let Some(k) = self.dot_pos {
            if k == self.right.len() {
                write!(f, "・")?;
            }
        }
        Ok(())
    }
}

fn nullable(grammers: &[Grammer], result: &mut HashSet<Nonterminal>) {
    let original_len = result.len();

    for g in grammers {
        if g.right.iter().all(|c| {
            if let &Character::Nonterminal(ref n) = c {
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
            use Character::*;

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
            use Character::*;

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

    for g in insert_dots(grammers).iter() {
        println!("{}", g);
    }

    println!("{:?}", null);
    println!("{:?}", firsts);
    println!("{:?}", follows);
}

