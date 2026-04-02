use num_traits::PrimInt;
use std::{
    cmp::{ PartialEq, Eq, PartialOrd, Ord, Ordering },
    ops::{ Deref, DerefMut, Add },
};
use crate::{
    Nummer,
    mng::Menge,
    knz::Kennzeichen as Knz,
    error::ParseError,
};

pub struct Kennletter(u8);

impl Kennletter {
    pub fn new(c: &char) -> Self {
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

impl Clone for Kennletter {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Kennletter {}

impl TryFrom<u8> for Kennletter {
    type Error = ParseError;
    fn try_from(src: u8) -> Result<Self, Self::Error> {
        match src {
            b'a'..b'z'|b'A'..b'Z' => Ok(Self(src.to_ascii_lowercase())),
            _ => Err(ParseError::InvalLetter(src as char))
        }
    }
}
impl TryFrom<char> for Kennletter {
    type Error = ParseError; 
    fn try_from(src: char) -> Result<Self, Self::Error> {
        match src {
            'a'..'z'|'A'..'Z' => Ok(Self(src.to_ascii_lowercase() as u8)),
            _ => Err(ParseError::InvalLetter(src))
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

impl Nummer for Kennletter {
    type IntSize = u8;
    fn is_unique(&self) -> bool { false }
    fn as_int(&self) -> Self::IntSize { self.0 }
    fn try_to_knz(&self) -> Option<Knz<Self::IntSize>> { None }
    fn to_some_int<R: From<Self::IntSize>>(&self) -> R {
        <R as From<Self::IntSize>>::from(self.0)
    }
}

pub enum Marke<T: PrimInt> {
    Literal(Kennletter),
    Numeric(T),
}

impl<T: PrimInt> Marke<T> {
    pub fn new_letter() -> Self {
        Self::Literal(Kennletter::new(&'a'))
    }
    pub fn to_letter(c: &char) -> Option<Self> {
        match c {
            'a'..'z' | 'A'..'Z' => {
                let kenn_lttr = Kennletter::new(&c.to_ascii_lowercase());
                Some(Self::Literal(kenn_lttr))
            }
            _ => None,
        }
    }
    pub fn to_letter_safe(c: &char) -> Self {
        match Self::to_letter(c) {
            Some(m) => m,
            None => panic!("Cannot translate `{}` to Kennletter.", c),
        }
    }
}

impl<T: PrimInt + Default> Marke<T> {
    pub fn new_num() -> Self {
        Self::Numeric(T::default())
    }
}
impl<T: PrimInt + From<u8> + Default> Marke<T> {
    pub fn prev(&self) -> Option<Self> {
        match self {
            Self::Numeric(n) => {
                if *n <= <T as From<u8>>::from(b'1') {
                    None
                } else {
                    let num = *n - <T as From<u8>>::from(b'1');
                    Some(Self::Numeric(num))
                }
            }
            | Self::Literal(l) => {
                if let Some(lttr) = l.prev() { Some(Self::Literal(lttr)) }
                else { None }
            }
        }
    }
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Numeric(n) => {
                let num = *n + <T as From<u8>>::from(b'1');
                Some(Self::Numeric(num))
            },
            Self::Literal(l) => {
                if let Some(nxt) = l.next() { Some(Self::Literal(nxt)) }
                else { None }
            }
        }
    }
}

impl<T: PrimInt> Clone for Marke<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Literal(l) => Self::Literal(*l),
            Self::Numeric(n) => Self::Numeric(*n),
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
            _ => false
        }
    }
}

impl<I: PrimInt> PartialEq<char> for Marke<I> {
    fn eq(&self, rhs: &char) -> bool {
        match self {
            Self::Literal(l) => &l.as_char() == rhs || l.as_char() == rhs.to_ascii_lowercase(),
            Self::Numeric(_) => false,
        }
    }
}

impl<I: PrimInt + From<u8>> From<u8> for Marke<I> {
    fn from(src: u8) -> Self {
        match src {
            b'a'..b'z' => Self::Literal(Kennletter(src)),
            b'A'..b'Z' => Self::Literal(Kennletter(src.to_ascii_lowercase())),
            _ => Self::Numeric(<I as From<u8>>::from(src))
        }
    }
}

impl<I: PrimInt + From<u8>> TryFrom<char> for Marke<I> {
    type Error = ParseError;
    fn try_from(src: char) -> Result<Self, Self::Error> {
        let r = match src {
            'a'..'z' | 'A'..'Z' => {
                let kl = Kennletter::try_from(src.to_ascii_lowercase())
                    .unwrap();
                Self::Literal(kl)
            },
            '0'..'9' => Self::Numeric(<I as From<u8>>::from(src as u8)),
            _ => return Err(ParseError::InvalMarkChar(src)),
        };
        return Ok(r)
    }
}

impl<T: PrimInt + PartialOrd> PartialOrd for Marke<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (self, rhs) {
            (Self::Literal(_), Self::Numeric(_)) | (Self::Numeric(_), Self::Literal(_)) => None,
            (Self::Literal(l0), Self::Literal(l1)) => l0.partial_cmp(l1),
            (Self::Numeric(n0), Self::Numeric(n1)) => n0.partial_cmp(n1),
        }
    }
}

impl<T> Nummer for Marke<T>
where
    T: PrimInt + From<u8>,
{
    type IntSize = T;
    fn as_int(&self) -> Self::IntSize {
        match self {
            Self::Literal(l) => <Self::IntSize as From<u8>>::from(l.as_byte()),
            Self::Numeric(n) => *n,
        }
    }
    fn is_unique(&self) -> bool { false }
    fn try_to_knz(&self) -> Option<Knz<T>> {
        Some(Knz::from(self.clone()))
    } 
}

pub trait Mark<P: PrimInt> {
    fn to_mark(&self) -> Marke<P>;
}

impl Mark<u64> for char {
    fn to_mark(&self) -> Marke<u64> {
        let l = Kennletter::new(self);
        Marke::Literal(l)
    }
}

impl Mark<u64> for u64 {
    fn to_mark(&self) -> Marke<u64> {
        Marke::Numeric(*self)
    }
}

impl<T: PrimInt + From<u8>> Mark<T> for u8 {
    fn to_mark(&self) -> Marke<T> {
        match self {
            b'a'..b'z' => Marke::Literal(
                Kennletter::try_from(*self)
                    .expect("")
            ),
            b'A'..b'Z' => Marke::Literal(
                Kennletter::try_from(self.to_ascii_lowercase())
                    .expect("")
            ),
            _ => Marke::Numeric(<T as From<u8>>::from(*self))
        }
    }
}
