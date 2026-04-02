use num_traits::PrimInt;
use crate::{
    knz::Kennzeichen as Knz,
};

pub trait Nummer: Ord {
    type IntSize: PrimInt;
    fn as_int(&self) -> Self::IntSize;
    fn is_unique(&self) -> bool { false }
    fn to_some_int<R: From<Self::IntSize>>(&self) -> R {
        <R as From<Self::IntSize>>::from(self.as_int())
    }
    fn try_as_knz(&self) -> Option<&Knz<Self::IntSize>>;
    //fn try_to_signature(&self) -> Option<Knz>;
}

pub trait Kennung: Nummer
where
{
    fn as_knz(&self) -> Knz<Self::IntSize>;
    fn strict_eq(&self, rhs: Self) -> bool;
}
