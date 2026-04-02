pub mod func;
pub mod mng;
pub mod dok;
pub mod mark;
pub mod error;
pub mod knz;
mod sig;

use num_traits::PrimInt;
use knz::Kennzeichen as Knz;
use once_cell::sync::Lazy;
use std::{
    sync::Mutex,
    hash::HashMap,
};

pub static ZUGANGSNUMMERN: Lazy<Mutex<Vec<usize>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub trait Nummer {
    type IntSize: PrimInt;
    fn as_int(&self) -> Self::IntSize;
    fn is_unique(&self) -> bool { false }
    fn to_some_int<R: From<Self::IntSize>>(&self) -> R {
        <R as From<Self::IntSize>>::from(self.as_int())
    }
    fn try_to_knz(&self) -> Option<Knz<Self::IntSize>>;
    //fn try_to_signature(&self) -> Option<Knz>;
}

pub trait Kennung: Nummer {
    type Nummer: Nummer;
    fn nummer(&self) -> &Self::Nummer; 
    fn nummer_mut(&self) -> &mut Self::Nummer;
    fn as_lauf_nr(&self) -> usize;
    fn as_knz(&self) -> &Knz<Self::IntSize>;
    fn strict_eq(&self, rhs: &Self) -> bool;
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
