pub mod fun;
pub mod mng;
pub mod mark;
pub mod error;
pub mod knz;
pub mod sign;

use num_traits::PrimInt;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static ZUGANGSNUMMERN: Lazy<Mutex<Vec<usize>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub trait Nummer: PartialOrd {
    type IntSize: PrimInt;

    fn as_int(&self) -> Self::IntSize;
    fn to_some_int<R: From<Self::IntSize>>(&self) -> R {
        <R as From<Self::IntSize>>::from(self.as_int())
    }
}

pub trait Zeichen: PartialOrd {
    type Kennung: Nummer;
    fn as_kenn(&self) -> &Self::Kennung;
    fn as_kenn_mut(&mut self) -> &mut Self::Kennung;
    fn as_lauf_nmr(&self) -> usize;
    fn strict_eq(&self, rhs: &Self) -> bool {
        self.as_kenn() == rhs.as_kenn() && self.as_lauf_nmr() == rhs.as_lauf_nmr()
    }
    fn base_eq(&self, rhs: &Self) -> bool {
        self.as_kenn() == rhs.as_kenn()
    }
}

pub struct Katalog<K: Zeichen, V>(Vec<K>, Vec<V>);

impl<K, V> Katalog<K, V>
where
    K: Zeichen + Clone,
    V: Clone,
{
    pub fn remove(&self, key: K) -> Option<(K, V)> {
        if let Some(e) = self.0
            .iter()
            .enumerate()
            .find(|(_, k)| k == &&key)
        {
            let old_key: K = e.1.clone();
            let old_val: V = self.1
                .get(e.0)
                .unwrap()
                .clone();
            Some((old_key, old_val))
        } else {
            None
        }
    }

}
impl<K, V> Katalog<K, V>
where
    K: Zeichen,
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
        while let Some(k) = self.0.get(i) {
            if k == &key {
                return None
            } else if k > &key {
                self.0.insert(i, key);
                self.1.insert(i, val);
                return self.1.get(i)
            } else {
                i += 1;
            }
        };
        self.0.push(key);
        self.1.push(val);
        self.1.last()
    }
    pub fn exists(&self, key: K) -> bool {
        self.0
            .iter()
            .find(|k| **k == key)
            .is_some()
    }
}

impl<K, V> Katalog<K, V>
where
    K: Zeichen,
    V: PartialOrd,
{
    pub fn get_key(&self, val: V) -> Option<&K> {
        match self.1
            .iter()
            .enumerate()
            .find(|(_, v)| v == &&val)
        {
            Some((i, _)) => self.0.get(i),
            None => None,
        }
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
