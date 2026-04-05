use std::{
    hash::{ Hash, Hasher, DefaultHasher },
    collections::HashMap,
    cmp::{ PartialEq, Eq, PartialOrd, Ord, Ordering },
};
use crate::{
    ZUGANGSNUMMERN,
    Nummer,
};

pub trait Zeichen {
    type Nummer: Nummer + Hash;
    fn lauf_nmr(&self) -> usize;
    fn knz(&self) -> &Self::Nummer;
}

pub struct Signatur<N: Nummer> {
    z_num: usize,
    num: N,
}

impl<N: Nummer> Signatur<N> {
    pub fn new<S>(src: S) -> Self
    where
        S: Into<N>,
    {
        let mut reg = ZUGANGSNUMMERN
            .lock()
            .unwrap();
        let z_num = reg.len();
        reg.push(z_num);
        Self { z_num, num: src.into()  }
    }
}

impl<N: Nummer> PartialEq for Signatur<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.nummer() == rhs.nummer() && self.zugang() == rhs.zugang()
    }
}

impl<N: Nummer> Eq for Signatur<N> {}

impl<N: Nummer> PartialOrd for Signatur<N> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.nummer().partial_cmp(&rhs.nummer())
    }
}

impl<N: Nummer> Ord for Signatur<N> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        if let Some(o) = self.nummer().partial_cmp(&rhs.nummer()) { o }
        else { self.zugang().cmp(&rhs.zugang()) }
    }
}

impl<N: Nummer + Hash> Zeichen for Signatur<N> {
    type Nummer = N;
    fn knz(&self) -> &Self::Nummer {
        &self.num
    }
    fn lauf_nmr(&self) -> usize {
        self.z_num
    }
}

pub struct Katalog<N: Nummer + Hash, K: Zeichen<Nummer = N>, V>(HashMap<K, HashMap<usize, Option<V>>);

impl<K, V> Katalog<K, V>
where
    K: Zeichen,
    V: Clone,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn add(&mut self, key: K, val: V) {
        let mut entry = {
            if let Some(v) =  {
                if let Some(e) = v.get() 
            }
        };
    }
}

impl<K, V> std::ops::Deref for Katalog<K, V>
where
    K: Zeichen,
    V: Clone
{
    type Target = HashMap<K, HashMap<usize, Option<V>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> std::ops::DerefMut for Katalog<K, V>
where
    K: Zeichen,
    V: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
