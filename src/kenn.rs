use std::cmp::{ PartialEq, PartialOrd, Ordering, Eq, Ord };
use crate::{
    func,
    knz,
    Nummer, Zeichen,
};

pub struct Index<N: Nummer> {
    nmr: N,
    run_id: usize,
}

impl<N> Index<N>
where
    N: Nummer + Default,
{
    pub fn new() -> Self {
        Self {
            nmr: N::default(), 
            run_id: func::get_znr(),
        }
    }
}

impl<N> PartialEq for Index<N>
where
    N: Nummer,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.nmr == rhs.nmr && self.run_id == rhs.run_id
    }
}

impl<N: Nummer> PartialOrd for Index<N> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.nmr.partial_cmp(&rhs.nmr)
    }
}

impl<N: Nummer> Eq for Index<N> {}

impl<N: Nummer> Ord for Index<N> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        if let Some(o) = self.nmr.partial_cmp(&rhs.nmr) { o }
        else { self.run_id.cmp(&rhs.run_id) }
    }
}

impl<N: Nummer> Zeichen for Index<N> {
    type Kennung = N;
    fn as_kenn(&self) -> &Self::Kennung {
        &self.nmr
    }
    fn as_kenn_mut(&mut self) -> &mut Self::Kennung {
        &mut self.nmr
    }
    fn as_lauf_nmr(&self) -> usize {
        self.run_id
    }
}

pub type Signatur = Index<knz::Kennzeichen<u64>>;
