pub mod func;
pub mod mng;
pub mod mark;
pub mod error;
pub mod knz;
pub mod sign;


use num_traits::PrimInt;
use knz::Kennzeichen as Knz;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static ZUGANGSNUMMERN: Lazy<Mutex<Vec<usize>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub trait Nummer: Clone + PartialOrd {
    type IntSize: PrimInt;
    fn as_int(&self) -> Self::IntSize;
    fn is_unique(&self) -> bool { false }
    fn to_some_int<R: From<Self::IntSize>>(&self) -> R {
        <R as From<Self::IntSize>>::from(self.as_int())
    }
    fn try_to_knz(&self) -> Option<Knz<Self::IntSize>>;
    //fn try_to_signature(&self) -> Option<Knz>;
}

pub trait Kennung: Nummer + std::fmt::Debug {
    type Nummer: Nummer;
    fn nummer(&self) -> &Self::Nummer; 
    fn nummer_mut(&self) -> &mut Self::Nummer;
    fn as_lauf_nr(&self) -> usize;
    fn as_knz(&self) -> &Knz<Self::IntSize>;
    fn strict_eq(&self, rhs: &Self) -> bool;
}

pub struct Catalogue<K: Kennung<IntSize = u64>, V>(Vec<K>, Vec<V>);

impl<K, V> Catalogue<K, V>
where
    K: Kennung<IntSize = u64>,
{
    pub fn new() -> Self {
        Self(Vec::new(), Vec::new())
    }
    pub fn get(&mut self, key: K) -> Option<&V> {
        if let Some((i, _)) = self.0
            .iter()
            .enumerate()
            .find(|(_, k)| **k == key)
        {
            self.1.get(i)
        } else {
            None
        }
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.0
            .iter()
            .zip(self.1.iter_mut())
    }
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.0
            .iter()
            .zip(self.1.iter())
    }
    pub fn add(&mut self, key: K, val: V) -> Option<&V> {
        let mut i = 0_usize;
        while self.0.get(i) == Some(key)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
