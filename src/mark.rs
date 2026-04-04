use num_traits::PrimInt;
use std::{
    cmp::{ PartialEq, Eq, PartialOrd, Ord, Ordering },
    ops::{ Deref, DerefMut },
    fmt::{ Display, Formatter, Debug },
};
use crate::{
    Nummer,
    error::Error,
};

pub struct Kennletter(u8);

impl Kennletter {
    pub fn new(c: char) -> Self {
        Self(c.to_ascii_lowercase() as u8)
    }
    pub fn next(&self) -> Option<Self> {
        if self.0 >= b'z' { None }
        else { Some(Self(self.0 + 1)) }
    }
    pub fn prev(&self) -> Option<Self> {
        if self.0 <= b'a' { None }
        else { Some(Self(self.0 - 1)) }
    }
    pub fn as_char(&self) -> char {
        self.0 as char
    }
    pub fn as_byte(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Kennletter {
    type Error = Error;
    fn try_from(src: u8) -> Result<Self, Self::Error> {
        match src {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Self(src.to_ascii_lowercase())),
            _ => Err(Error::InvalLetter(src as char)),
        }
    }
}

impl TryFrom<char> for Kennletter {
    type Error = Error;
    fn try_from(src: char) -> Result<Self, Self::Error> {
        match src {
            'a'..='z' | 'A'..='Z' => Ok(Self(src.to_ascii_lowercase() as u8)),
            | _ => Err(Error::InvalLetter(src)),
        }
    }
}

impl Clone for Kennletter {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Kennletter {}

impl<T> TryFrom<char> for Marke<T>
where
    T: PrimInt + From<u32>
{
    type Error = Error;
    fn try_from(src: char) -> Result<Self, Self::Error> {
        match src {
            'a'..='z'|'A'..='Z' => Ok(Marke::to_letter_safe(src)),
            '0'..='9' => {
                let n = <T as From<u32>>::from(src.to_digit(10).unwrap());
                Ok(Marke::Numeric(n))
            },
            | _ => {
                match SepMark::try_from(src) {
                    Ok(s) => Ok(Marke::Sep(s)),
                    _ => Err(Error::InvalMarke(src))
                }
            }
        }
    }
}

impl PartialEq for Kennletter {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl Eq for Kennletter {}

impl PartialOrd for Kennletter {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl Ord for Kennletter {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl PartialEq<u8> for Kennletter {
    fn eq(&self, rhs: &u8) -> bool {
        self.as_byte().eq(rhs)
    }
}

impl Deref for Kennletter {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Kennletter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq<char> for Kennletter {
    fn eq(&self, rhs: &char) -> bool {
        self.0 as char == rhs.to_ascii_lowercase()
    }
}

impl PartialOrd<char> for Kennletter {
    fn partial_cmp(&self, rhs: &char) -> Option<Ordering> {
        self.0.partial_cmp(&(rhs.to_ascii_lowercase() as u8))
    }
}

#[derive(Debug)]
pub struct SepMark(u8);

impl SepMark {
    pub fn new() -> Self {
        Self(b':')
    }
    pub fn as_byte(&self) -> u8 {
        self.0
    }
    pub fn as_char(&self) -> char {
        self.0 as char
    }
}

impl Display for SepMark {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl TryFrom<u8> for SepMark {
    type Error = Error;
    fn try_from(src: u8) -> Result<Self, Self::Error> {
        match src {
            b':' | b'/' | b';' | b'-' | b'+' | b'=' | b'*' | b'_' | b'\'' | b'"' => Ok(Self(src)),
            _ => Err(Error::InvalSepMark(src as char))
        }
    }
}

impl TryFrom<char> for SepMark {
    type Error = Error;
    fn try_from(src: char) -> Result<Self, Self::Error> {
        match src {
            ':' | '/' | ';' | '-' | '+' | '=' | '*' | '_' | '\'' | '"' => Ok(Self(src as u8)),
            _ => Err(Error::InvalSepMark(src))
        }
    }
}

impl PartialEq for SepMark {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl Eq for SepMark {}

impl PartialEq<SepMark> for char {
    fn eq(&self, rhs: &SepMark) -> bool {
        rhs.eq(self)
    }
}
impl PartialEq<char> for SepMark {
    fn eq(&self, rhs: &char) -> bool {
        &self.as_char() == rhs
    }
}

impl PartialEq<u8> for SepMark {
    fn eq(&self, rhs: &u8) -> bool {
        &self.0 == rhs
    }
}

impl Clone for SepMark {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for SepMark {}

#[derive(Debug)]
pub enum Marke<T: PrimInt> {
    Literal(Kennletter),
    Numeric(T),
    Sep(SepMark),
}


impl<T: PrimInt> Marke<T> {
    pub fn new_letter() -> Self {
        Self::Literal(Kennletter::new('a'))
    }
    pub fn to_sep(c: char) -> Option<Self> {
        Some(Marke::Sep(
            SepMark::try_from(c)
                .ok()?))
    }
    pub fn to_sep_safe(c: char) -> Self {
        Marke::Sep(SepMark::try_from(c)
            .unwrap())
    }
    pub fn to_letter(c: char) -> Option<Self> {
        match c {
            'a'..='z' | 'A'..='Z' => {
                let kenn_lttr = Kennletter::new(c.to_ascii_lowercase());
                Some(Self::Literal(kenn_lttr))
            }
            _ => None,
        }
    }
    pub fn to_letter_safe(c: char) -> Self {
        match Self::to_letter(c) {
            Some(m) => m,
            None => panic!("Cannot translate `{}` to Kennletter.", c),
        }
    }
}

impl<T: PrimInt + From<u32>> Marke<T> {
    pub fn merge_marks(self, m: &Marke<T>) -> Option<Self> {
        match (self, m) {
            (Self::Numeric(n0), Self::Numeric(n1)) => {
                let new = (n0 * <T as From<u32>>::from(10_u32)) + *n1;
                Some(Marke::Numeric(new))
            }
            | _ => None,
        }
    }
    pub fn merge_digits(self, d: T) -> Option<Self> {
        match self {
            Self::Numeric(n) => {
                let new = (n * <T as From<u32>>::from(10_u32)) + d;
                Some(Self::Numeric(new))
            }
            | _ => None,
        }
    }
}

impl<T: PrimInt + Default> Marke<T> {
    pub fn new_num() -> Self {
        Self::Numeric(T::default())
    }
}
impl<T: PrimInt + From<u32> + Default> Marke<T> {
    pub fn prev(&self) -> Option<Self> {
        match self {
            Self::Numeric(n) => {
                if *n <= <T as From<u32>>::from(1_u32) {
                    None
                } else {
                    let num = *n - <T as From<u32>>::from(1_u32);
                    Some(Self::Numeric(num))
                }
            }
            | Self::Literal(l) => {
                if let Some(lttr) = l.prev() { Some(Self::Literal(lttr)) }
                else { None }
            }
            | _ => None,
        }
    }
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Numeric(n) => {
                let num = *n + <T as From<u32>>::from(1_u32);
                Some(Self::Numeric(num))
            },
            Self::Literal(l) => {
                if let Some(nxt) = l.next() { Some(Self::Literal(nxt)) }
                else { None }
            }
            | _ => None,
        }
    }
}

impl<T: PrimInt> Clone for Marke<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Literal(l) => Self::Literal(*l),
            Self::Numeric(n) => Self::Numeric(*n),
            Self::Sep(s) => Self::Sep(*s)
        }
    }
}

impl<T: PrimInt> Copy for Marke<T> {}

impl<I: PrimInt> PartialEq for Marke<I> {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Literal(l0), Self::Literal(l1)) => {
                l0 == l1
            }
            | (Self::Numeric(n0), Self::Numeric(n1)) => {
                n0 == n1
            }
            | (Self::Sep(s0), Self::Sep(s1)) => {
                s0 == s1
            }
            _ => false
        }
    }
}

impl<I: PrimInt> PartialEq<char> for Marke<I> {
    fn eq(&self, rhs: &char) -> bool {
        match self {
            Self::Literal(l) => &l.as_char() == rhs || l.as_char() == rhs.to_ascii_lowercase(),
            Self::Numeric(_) => false,
            Self::Sep(s) => &s.as_char() == rhs,
        }
    }
}

impl<T: PrimInt + PartialOrd> PartialOrd for Marke<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (self, rhs) {
            (Self::Literal(l0), Self::Literal(l1)) => l0.partial_cmp(l1),
            (Self::Numeric(n0), Self::Numeric(n1)) => n0.partial_cmp(n1),
            | _ => None,
        }
    }
}

impl<T> Nummer for Marke<T>
where
    T: PrimInt + From<u32> + From<u8>,
{
    type IntSize = T;
    fn as_int(&self) -> Self::IntSize {
        match self {
            Self::Literal(l) => <Self::IntSize as From<u8>>::from(l.as_byte()),
            Self::Numeric(n) => *n,
            Self::Sep(s) => <Self::IntSize as From<u8>>::from(s.as_byte()),
        }
    }
}

pub trait Mark<P: PrimInt> {
    fn to_mark(&self) -> Marke<P>;
}

impl Mark<u64> for char {
    fn to_mark(&self) -> Marke<u64> {
        let l = Kennletter::new(*self);
        Marke::Literal(l)
    }
}

impl Mark<u64> for u64 {
    fn to_mark(&self) -> Marke<u64> {
        Marke::Numeric(*self)
    }
}

// DEBUG AND DISPLAY
impl Debug for Kennletter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Kennletter({}->{})", self.0, self.0 as char)
    }
}
impl Display for Kennletter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    } 
}

#[cfg(test)]
mod test {
    use super::*;

    static LETTERS: &'static[char] = &[ 'a', 'b', 'Y', '/', 'ü', 'z', ];

    #[test]
    fn kennletter() {
        let r = LETTERS
            .iter()
            .map(|l| Kennletter::try_from(*l))
            .collect::<Vec<Result<Kennletter, Error>>>();
        let expected: Vec<Result<Kennletter, Error>> = Vec::from([
            Ok(Kennletter(b'a')),
            Ok(Kennletter(b'b')),
            Ok(Kennletter(b'y')),
            Err(Error::InvalLetter('/')),
            Err(Error::InvalLetter('ü')),
            Ok(Kennletter(b'z')),
        ]);
        assert_eq!(r, expected)
    }
    static SEQ_MARKS: &'static[char] = &[
        '_', ';', '.', ':', 'a', '=', '+', '>', '-', '/',
    ];
    #[test]
    fn seq_mark() {
        let r = SEQ_MARKS
            .iter()
            .map(|c| SepMark::try_from(*c))
            .collect::<Vec<Result<SepMark, Error>>>();
        let expected = Vec::from([
            Ok(SepMark(b'_')),
            Ok(SepMark(b';')),
            Err(Error::InvalSepMark('.')),
            Ok(SepMark(b':')),
            Err(Error::InvalSepMark('a')),
            Ok(SepMark(b'=')),
            Ok(SepMark(b'+')),
            Err(Error::InvalSepMark('>')),
            Ok(SepMark(b'-')),
            Ok(SepMark(b'/')),
        ]);
        assert_eq!(r, expected)
    }
    static MARKEN_FROM_CHAR: &'static[char] = &[
        'a', 'Ü', 'z', '4', '-', 'U', ':', '.', '0',
    ];
    #[test]
    fn char_marken() {
        let r = MARKEN_FROM_CHAR
            .iter()
            .map(|c| Marke::try_from(*c))
            .collect::<Vec<Result<Marke<u64>, Error>>>();
        let expected: Vec<Result<Marke<u64>, Error>> = Vec::from([
            Ok(Marke::Literal(Kennletter(b'a'))),
            Err(Error::InvalMarke('Ü')),
            Ok(Marke::Literal(Kennletter(b'z'))),
            Ok(Marke::Numeric(4_u64)),
            Ok(Marke::Sep(SepMark(b'-'))),
            Ok(Marke::Literal(Kennletter(b'u'))),
            Ok(Marke::Sep(SepMark(b':'))),
            Err(Error::InvalMarke('.')),
            Ok(Marke::Numeric(0_u64)),
        ]);
        assert_eq!(r, expected);
    }
}
