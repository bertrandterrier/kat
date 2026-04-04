use std::{
    fmt::{ Debug },
    cmp::{ Eq, Ord, Ordering, PartialEq, PartialOrd },
    default::Default,
    iter::{ FromIterator, Iterator }
};
use crate::error::Error;

#[derive(Debug)]
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
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn last_mut(&mut self) -> &mut T {
        if let Some(l) = self.1.last_mut() { l }
        else { &mut self.0 }
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
        self.1.remove(0);
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
    pub fn rest(&self) -> &[T] {
        &self.1
    }
}

impl<T: Clone> From<(T, Vec<T>)> for Menge<T> {
    fn from(src: (T, Vec<T>)) -> Self {
        let mut new = Menge::new(src.0);
        for e in src.1 {
            new.push_back(e.clone())
        };
        new 
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
    type Error = Error;
    fn try_from(src: &[T]) -> Result<Self, Self::Error> {
        if src.len() <= 0 { return Err(Error::EmptyMenge) }
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

impl<T: Clone> TryFrom<Vec<T>> for Menge<T> {
    type Error = Error;
    fn try_from(src: Vec<T>) -> Result<Self, Self::Error> {
        if src.len() < 1 {
            Err(Error::EmptyMenge)
        } else {
            let first = src
                .first()
                .unwrap()
                .clone();
            let mut m = Menge::new(first);
            if let Some(slc) = src.get(1..) {
                for e in slc {
                    m.push_back(e.clone())
                }
            }
            return Ok(m)
        }
    }
}

impl<T: Copy + Debug> TryFrom<Menge<Option<T>>> for Menge<T> {
    type Error = Error;
    fn try_from(src: Menge<Option<T>>) -> Result<Self, Self::Error> {
        let mut m = match src.first() {
            Some(t) => Self::new(t.clone()),
            None => return Err(Error::NoNullmenge(
                format!("{:?}", src)
            )),
        };
        for opt in src.1 {
            if let Some(t) = opt {
                m.push_back(t.clone());
            } else {
                return Err(Error::NoNullmenge(
                    format!("{:?}..?", m)
                ))
            }
        };
        Ok(m)
    }
}

impl<T: Clone> FromIterator<T> for Menge<T> {
    fn from_iter<I: IntoIterator<Item = T>>(src: I) -> Self {
        let mut iter = src.into_iter();
        let mut m: Menge<T> = Menge::new(iter
            .next()
            .unwrap());
        while let Some(t) = iter.next() {
            m.push_back(t);
        };
        m
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mark::Marke;
    static MENGE: &'static[(char, &'static str)] = &[
        ('a', "bc"),
        ('1', "234"),
        ('0', ""),
        ('i', "mmanuel"),
    ];
    #[test]
    fn char_menge() {
        let r: Vec<Menge<char>> = MENGE
            .iter()
            .map(|(f, r)| {
                let mut m: Menge<char> = Menge::new(*f);
                for c in r.chars() {
                    m.push_back(c);
                };
                m.pop_front();
                m
            })
            .collect::<Vec<Menge<char>>>();
        let expected: Vec<Menge<char>> = Vec::from(&[
            Menge('b', vec!['c']),
            Menge('2', vec!['3', '4']),
            Menge('0', Vec::new()),
            Menge('m', vec!['m', 'a', 'n', 'u', 'e', 'l']),
        ]);
        assert_eq!(r, expected)
    }
    #[test]
    fn marken_menge() {
        let v = Vec::from(&[
            Marke::Numeric(1_u32),
            Marke::to_letter_safe('a'),
            Marke::to_sep_safe('-'),
            Marke::Numeric(32_u32)
        ]);
        let m = Menge::try_from(v).unwrap();
        let expected = Menge::<Marke<u32>>(Marke::Numeric(1_u32), Vec::from(&[
            Marke::to_letter_safe('a'),
            Marke::to_sep_safe('-'),
            Marke::Numeric(32_u32)
        ]));
        assert_eq!(m, expected);
    }
}
