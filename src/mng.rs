use std::{
    cmp::{ Eq, Ord, Ordering, PartialEq, PartialOrd },
    default::Default,
    iter::{ FromIterator, Iterator }
};
use crate::{
    error::ParseError,
};

pub struct Menge<T>(T, Vec<T>);

impl<T> Menge<T> {
    pub fn len(&self) -> usize {
        self.1.len() + 1
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.0).chain(self.1.iter())
    }
    pub fn rev(&self) -> impl Iterator<Item = &T> {
        self.1
            .iter()
            .rev()
            .chain(std::iter::once(&self.0))
    }
    pub fn last(&self) -> &T {
        if let Some(l) = self.1.last() { l }
        else { &self.0 }
    }
    pub fn first(&self) -> &T {
        &self.0
    }
    pub fn replace_last(&mut self, new: T) {
        if self.1.len() > 0 {
            self.1.pop();
            self.1.push(new);
        } else {
            self.0 = new;
        }
    } 
}

impl<T: Clone> Menge<T> {
    pub fn new(first: T) -> Self {
        Self(first, Vec::new())
    }
    pub fn extend(&mut self, v: Vec<T>) {
        for t in v {
            self.1.push(t)
        }
    }
    pub fn push_front(&mut self, t: T) {
        self.1.insert(0, self.0.clone());
        self.0 = t;
    }
    pub fn push_back(&mut self, t: T) {
        self.1.push(t)
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let new_first = match self.1.get(0) {
            None => return None,
            Some(f) => f.clone(),
        };
        let old_first = self.0.clone();
        self.0 = new_first;
        Some(old_first)
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.1.pop()
    }
    pub fn get(&mut self, idx: usize) -> Option<&T> {
        if idx == 0 {
            Some(&self.0)
        } else if idx > self.1.len() {
            None
        } else {
            self.1.get(idx)
        }
    }
}


impl<T: PartialEq + Clone> Menge<T> {
    pub fn starts_with(&self, rhs: &Self) -> bool {
        if rhs.len() > self.len() {
            return false
        };
        self.iter()
            .zip(rhs.iter())
            .map(|(l, r)| l == r)
            .all(|b| b)
    }
    pub fn ends_with(&self, rhs: &Self) -> bool {
        if rhs.len() > self.len() {
            return false
        };
        self.rev()
            .zip(rhs.rev())
            .map(|(l, r)| l == r)
            .all(|b| b)
    }
    pub fn contains(&self, target: &T) -> bool {
        if &self.0 == target { true }
        else { self.1.contains(target) }
    }
}

impl<T: Clone + Default> Default for Menge<T> {
    fn default() -> Self {
        Self(T::default(), Vec::new())
    }
}

impl<T: Clone> TryFrom<&[T]> for Menge<T> {
    type Error = ParseError;
    fn try_from(src: &[T]) -> Result<Self, Self::Error> {
        if src.len() <= 0 { return Err(ParseError::EmptyMenge) }
        else if src.len() == 1 {
            let first = src.get(0).unwrap();
            Ok(Self(first.clone(), Vec::new()))
        }
        else {
            let first = src.get(0)
                .unwrap();
            Ok(Self(
                first.clone(),
                src.get(1..) 
                    .unwrap()
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<T>>()
                ))
        }
    }
}

impl<T: PartialEq> PartialEq for Menge<T> {
    fn eq(&self, rhs: &Self) -> bool {
        if self.len() != rhs.len() { return false };
        self.iter()
            .zip(rhs.iter())
            .map(|(l, r)| l == r)
            .all(|b| b)
    }
}

impl<T: Eq> Eq for Menge<T> {}

impl<T: PartialOrd> PartialOrd for Menge<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        for (l, r) in self.iter().zip(rhs.iter()) {
            match l.partial_cmp(r) {
                Some(Ordering::Equal) => continue,
                None => return None,
                Some(o) => return Some(o),
            }
        };
        self.len().partial_cmp(&rhs.len())
    }
}

impl<T: Ord> Ord for Menge<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        for (l, r) in self.iter().zip(rhs.iter()) {
            match l.cmp(r) {
                Ordering::Equal => continue,
                o => return o,
            }
        };
        self.len().cmp(&rhs.len())
    }
}

impl<T> FromIterator<T> for Menge<T>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(src: I) -> Self {
        let mut buf = src.into_iter();
        let first = buf.next()
            .expect("Empty Menge is not allowed.")
            .clone();
        Self(first, {
            let mut v: Vec<T> = Vec::new();
            while let Some(e) = buf.next() {
                v.push(e.clone())
            };
            v
        })
    }
}

impl<T: Clone> From<Menge<T>> for Vec<T> {
    fn from(src: Menge<T>) -> Self {
        Vec::from(src
            .iter()
            .map(|t| t.clone())
            .collect::<Vec<T>>())
    }
}

impl<T: Copy> Clone for Menge<T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
        self.1.clone_from(&source.1);
    }
}
