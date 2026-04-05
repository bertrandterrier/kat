use num_traits::PrimInt;
use std::{
    fmt::Debug,
    cmp::{ PartialEq, Eq, PartialOrd, Ord, Ordering },
    ops::{ Deref, DerefMut },
};
use crate::{
    error::Error,
    fun,
    mark::Marke,
    Nummer,
    mng::Menge,
};

/// The unique[^1] videntifier of the signature.
///
/// Semantically unique. Does not enforce uniqueness in process, only when saving.
#[derive(Debug)]
pub struct Kennzeichen<I: PrimInt>(Menge<Marke<I>>);

impl<I> Kennzeichen<I>
where
    Marke<I>: Copy,
    I: PrimInt + From<u32>,
{
    pub fn new(first: Marke<I>) -> Self {
        Self(Menge::new(first))
    }
    pub fn clone_to_new(&self) -> Self {
        Self(self.0.clone())
    }
    pub fn tail_is_num(&self) -> bool {
        match self.last() {
            Marke::Numeric(_) => true,
            | Marke::Literal(_) => false,
            | Marke::Sep(_) => {
                if self.len() < 2 {
                    false
                } else {
                    let new = self.root();
                    new.tail_is_num()
                }
            }
        }
    }
    pub fn trim_tail(&self) -> Self {
        let mut new = self.clone();
        new.trim_tail_mut();
        new
    }
    pub fn trim_tail_mut(&mut self) {
        match self.last() {
            Marke::Sep(_) => {
                self.root_mut();
                self.trim_tail();
            }
            | _ => {}
        }

    }
    pub fn add(&mut self, mrk: Marke<I>) {
        match mrk {
            Marke::Numeric(_) => {
                if self.tail_is_num() {
                    self.trim_tail_mut();
                    let new_mark = self
                        .last()
                        .merge_marks(&mrk)
                        .unwrap();
                    self.replace_last(new_mark);
                } else {
                    self.push_back(mrk)
                }
            }
            | _ => self.push_back(mrk),
        }
    }
    pub fn setup(s: &[Marke<I>]) -> Self {
        if s.len() < 1 {
            panic!("Cannot setup with empty slice.")
        } else {
            let mut m: Menge<Marke<I>> = Menge::new(s.first().unwrap().clone());
            if let Some(iter) = s.get(1..) {
                for elem in iter {
                    m.push_back(elem.clone())
                };
            };
            Self(m)
        }
    }
    /// Safe option to `self.unnest()`. Will default to same Kennzeichen as is, when on base level.
    pub fn root_mut(&mut self) {
        if self.len() < 2 { return };
        self.pop_back();
    }
    /// Safe option to `self.unnest()`. Will default to same Kennzeichen as is, when on base level.
    pub fn root(&self) -> Self {
        let mut new = self.clone();
        new.root_mut();
        new
    }
    pub fn unnest(&self) -> Option<Self> {
        if self.len() < 2 {
            None
        } else {
            let mut new = self.clone();
            new.pop_back();
            Some(new)
        }
    }
}

impl<T> Kennzeichen<T>
where
    T: Copy + PrimInt + From<u32> + Default + Debug,
{
    pub fn nest(&self) -> Self {
        let mut new = self.clone();
        new.nest_mut();
        new
    }
    pub fn next(&self) -> Self {
        let mut new = self.clone();
        new.next_mut();
        new
    }
    pub fn next_mut(&mut self) {
        let new_marke = if let Some(nxt) = self.last().next() {
            nxt
        } else {
            Marke::new_num()
        };
        self.replace_last(new_marke)
    }
    fn next_nest(&self) -> Marke<T> {
        match self.last() {
            Marke::Literal(_) => Marke::new_num(),
            Marke::Numeric(_) => Marke::new_letter(),
            Marke::Sep(_) => {
                if self.len() < 2 {
                    panic!("Kennzeichen should not start with a seperator `{:?}`", self.last())
                } else {
                    self.root().next_nest()
                }
            },
        }
    }
    pub fn nest_mut(&mut self) {
        let nxt_mark = self.next_nest();
        self.push_back(nxt_mark)
    }
}

impl<T> std::default::Default for Kennzeichen<T>
where
    T: From<u32> + PrimInt,
    Marke<T>: Default + Copy,
{
    fn default() -> Self {
        Self::new(Marke::default())
    }
}

impl Kennzeichen<u64> {
    pub fn new_std() -> Self {
        Self(Menge::new(Marke::Numeric(1_u64)))
    }
}

impl From<u64> for Kennzeichen<u64> {
    fn from(src: u64) -> Self {
        Self(Menge::new(Marke::Numeric(src)))
    }
}

impl<T: PrimInt + Copy> Clone for Kennzeichen<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<Menge<Marke<T>>> for Kennzeichen<T>
where
    T: PrimInt
{
    fn from(src: Menge<Marke<T>>) -> Self {
        Self(src)
    }
}

impl<T> From<Marke<T>> for Kennzeichen<T>
where
    T: PrimInt,
    Marke<T>: Clone,
{
    fn from(src: Marke<T>) -> Self {
        Self(Menge::new(src))
    }
}

impl<T: PartialEq + PrimInt> PartialEq for Kennzeichen<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}

impl<T: Eq + PrimInt> Eq for Kennzeichen<T> {}

impl<T> PartialOrd for Kennzeichen<T>
where
    T: PrimInt,
    Menge<Marke<T>>: PartialOrd,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl<T> Ord for Kennzeichen<T>
where
    T: PrimInt,
    Menge<Marke<T>>: Ord,
{
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl<T: PrimInt> Deref for Kennzeichen<T> {
    type Target = Menge<Marke<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PrimInt> DerefMut for Kennzeichen<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Nummer for Kennzeichen<u64> {
    type IntSize = u64;
    fn as_int(&self) -> Self::IntSize {
        fun::build_seq_id(
            self.0
                .iter()
                .map(|m| m.as_int())
        )
    }
}

impl<T: PrimInt> TryFrom<Vec<Marke<T>>> for Kennzeichen<T> {
    type Error = Error;
    fn try_from(src: Vec<Marke<T>>) -> Result<Self, Self::Error> {
        match Menge::try_from(src) {
            Ok(m) => Ok(Kennzeichen(m)),
            Err(e) => Err(e),
        }
    }
}

impl<T: PrimInt + Copy> TryFrom<Option<Menge<Marke<T>>>> for Kennzeichen<T> {
    type Error = Error;
    fn try_from(src: Option<Menge<Marke<T>>>) -> Result<Self, Self::Error> {
        if let Some(m) = src {
            Ok(Self(m.clone()))
        } else {
            Err(Error::SomeParseErr)
        }
    }
}

impl<T: Copy + PrimInt + From<u32> + Debug> TryFrom<Menge<Option<Marke<T>>>> for Kennzeichen<T> {
    type Error = Error;
    fn try_from(src: Menge<Option<Marke<T>>>) -> Result<Self, Self::Error> {
        let mut new: Kennzeichen<T> = Kennzeichen::new(match src.first() {
            Some(m) => *m,
            None => return Err(Error::SomeParseErr),
        });
        for opt in src.rest() {
            if let Some(e) = opt {
                new.add(*e)
            } else {
                return Err(Error::SomeParseErr)
            }
        };
        Ok(new)
    }
}

pub fn parse_str<T>(src: &str) -> Option<Kennzeichen<T>>
where
    T: Copy + PrimInt + From<u32> + std::fmt::Debug + std::fmt::Display,
{
    let first: char = {
        if let Some(f) = src.chars().next() { f }
        else { return None }
    };
    let mut buf: Kennzeichen<T> = Kennzeichen::new(match Marke::try_from(first) {
        Ok(m) => m,
        Err(_) => return None,
    });
    for c in src.get(1..)
        .into_iter()
        .flat_map(|s| s.chars())
    {
        match Marke::try_from(c) {
            Ok(m) => buf.add(m),
            Err(_) => return None,
        }
    };
    Some(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_str_test() {
        static TESTS: &'static[&'static str]= &[
            "1",
            "2a",
            "ülübegi unt",
            "--hallo hallo, welt!",
            "3a-12",
            "4_100z",
        ];
        let r: Vec<Option<Kennzeichen<u64>>> = TESTS
            .iter()
            .map(|s| parse_str::<u64>(s))
            .collect();
        let expect: Vec<Option<Kennzeichen<u64>>> = vec![
            Some(Kennzeichen(
                Menge::from((Marke::Numeric(1_u64),
                    Vec::new()
            )))),
            Some(Kennzeichen(
                Menge::from((Marke::Numeric(2_u64),
                    vec![
                        Marke::to_letter_safe('a')
                    ]))
            )),
            None,
            None,
            Some(Kennzeichen(
                Menge::from((Marke::Numeric(3_u64),
                    vec![
                        Marke::to_letter_safe('a'),
                        Marke::to_sep_safe('-'),
                        Marke::Numeric(12_u64),
                    ]))

            )),
            Some(Kennzeichen(
                Menge::from((Marke::Numeric(4100_u64),
                    vec![
                        Marke::to_letter_safe('z'),
                    ]))
            ))
        ];
        assert_eq!(r, expect);
    }
}
