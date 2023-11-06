use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::convert::From;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Any {
    String(String),
    Number(f64),
    List(VecDeque<Any>),
    Set(BTreeSet<Any>),
    Map(BTreeMap<Any, Any>),
}

impl fmt::Display for Any {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self)
    }
}

impl From<f64> for Any {
    fn from(x: f64) -> Self {
        Any::Number(x)
    }
}

impl From<i64> for Any {
    fn from(x: i64) -> Self {
        Any::Number(x as f64)
    }
}

impl From<&str> for Any {
    fn from(s: &str) -> Self {
        Any::String(s.into())
    }
}

impl From<String> for Any {
    fn from(s: String) -> Self {
        Any::String(s)
    }
}

impl<const N: usize> From<[Any; N]> for Any {
    fn from(any_slice: [Any; N]) -> Self {
        Any::List(VecDeque::from_iter(any_slice.iter().cloned()))
    }
}

impl<const N: usize> From<[&str; N]> for Any {
    fn from(str_slice: [&str; N]) -> Self {
        Any::List(str_slice.iter().map(|&s| Any::from(s)).collect())
    }
}

impl FromIterator<Any> for Any {
    fn from_iter<T: IntoIterator<Item = Any>>(iter: T) -> Self {
        Any::List(iter.into_iter().collect())
    }
}

impl Add for Any {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Any::Number(a), Any::Number(b)) => Any::Number(a + b),
            (Any::Number(a), Any::String(b)) => Any::String(format!("{a}{b}")),
            (Any::String(a), Any::Number(b)) => Any::String(format!("{a}{b}")),
            _ => todo!("cannot add these types yet"),
        }
    }
}

impl Sub for Any {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Any::Number(a), Any::Number(b)) => Any::Number(a - b),
            _ => todo!("cannot subtract these types yet"),
        }
    }
}

impl Mul for Any {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Any::Number(a), Any::Number(b)) => Any::Number(a * b),
            (Any::String(a), Any::Number(b)) => Any::String(a.repeat(b as usize)),
            _ => todo!("cannot multiply these types yet"),
        }
    }
}

impl Div for Any {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Any::Number(a), Any::Number(b)) => Any::Number(a / b),
            (Any::String(a), Any::String(b)) => Any::List(a.split(&b).map(Any::from).collect()),
            (Any::String(a), Any::Number(b)) => {
                let mut out = VecDeque::new();
                for chunk in a.chars().chunks(b as usize).into_iter() {
                    let s: String = chunk.collect();
                    out.push_back(Any::from(s));
                }
                Any::List(out)
            }
            _ => todo!("cannot divide these types yet"),
        }
    }
}

// pub enum Number {
//     Float(f64),
//     Integer(i64),
//     Infinity,
//     NegInfinity,
// }

impl Any {
    // map: would map over value(s) and return the same type
    // sorting a list would just mean converting to a set and back to a list
    // Into/to_x() functions available between ALL types, upgrades and downgrades/lossless and lossy
    // ALL operators overloaded, each would essentially be a bunch of matches on the enum variants
    // many operators would cause automatic type upgrades to occur
    // string / string == string.split(string)
    // string * string == string.join(string)
    // Integer(1) + Float(0.5) == Float(1.5)
    // Float(0.5) + Float(0.5) == Integer(1)
    // string + number, or string - number, would resize the string and fill in LLM-like predicted values deterministically
    // number + string, might have to be the same operation. another alt would be to auto-convert the number to a string,
    // but do so by converting it to its english name like three hundred and seventy four.
    // string * number could repeat
    // string / number could divide up a string into chunks
    // string + string could have various behaviors if the strings parse out to values or something (really troll/cursed)
}
