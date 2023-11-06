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
