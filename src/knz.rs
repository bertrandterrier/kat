use num_traits::PrimInt;
use std::{
    cmp::{ PartialEq, Eq, PartialOrd, Ord, Ordering },
    ops::{ Deref, DerefMut },
};
use crate::{
    GLOB_KNZ_REG,
    func,
    mng::Menge,
    mark::{ Marke },
    Nummer,
};

/// The unique[^1] videntifier of the signature.
///
/// Semantically unique. Does not enforce uniqueness in process, only when saving.
pub struct Kennzeichen<I: PrimInt>(Menge<Marke<I>>);

impl<I> Kennzeichen<I>
where
    Marke<I>: std::default::Default + Clone,
    I: PrimInt + Default + From<u8>,
{
    pub fn new() -> Self {
        Self(Menge::new(Marke::default()))
    }
    pub fn clone_to_new(&self) -> Self {
        Self(self.0.clone())
    }
    pub fn nest_mut(&mut self) {
        match self.last() {
            Marke::Literal(_) => self.push_back(Marke::new_letter()),
            Marke::Numeric(_) => self.push_back(Marke::new_num()),
        };
    }
    pub fn nest(&self) -> Self {
        let mut new = self.clone();
        new.nest_mut();
        new
    }
    pub fn next(&self) -> Self {
        let mut new = self.clone();
        match new.last().next() {
            Some(m) => new.replace_last(m),
            None => new.push_back(Marke::new_num())
        }
        new
    }
    pub fn remove_from_index(&self, idx: &mut Vec<Self>) {
        if let Some((i, _)) = idx.iter()
            .enumerate()
            .find(|(_, e)| e == &self)
        {
            idx.remove(i);
        } else {
            return
        }
    }
    pub fn prev(&self) -> Option<Self> {
        let mut new = self.clone();
        if let Some(p) = self.last().prev() { new.replace_last(p); }
        else if new.len() > 1 { new.pop_back(); }
        else { return None };
        Some(new)
    }
    pub fn next_free(&self, idx: &mut Vec<Self>) -> Self {
        let mut new = self.clone();
        while idx.contains(&new) {
            new = new.next()
        };
        return new
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
        Self(Menge::new(src, None))
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
    fn try_to_knz(&self) -> Option<Kennzeichen<u64>> { Some(self.clone()) }
    fn as_int(&self) -> Self::IntSize {
        func::build_seq_id(
            self.0
                .iter()
                .map(|m| m.as_int())
        )
    }
    fn is_unique(&self) -> bool { false }
}
