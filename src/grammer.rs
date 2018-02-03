use std;
use std::fmt::{Formatter, Display, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nonterminal(String);

impl From<String> for Nonterminal {
    fn from(c: String) -> Self {
        Nonterminal(c.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Character {
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
pub struct Grammer {
    pub left: Nonterminal,
    pub right: Vec<Character>,
    pub dot_pos: Option<usize>,
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

