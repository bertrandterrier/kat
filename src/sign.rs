use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::Nummer;

pub static NUMMERN: Lazy<Mutex<Vec<u64>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub struct Signatur<N: Nummer<IntSize = u64>> {
    nummer: N,
    run_id: u128,
    format: Vec<Option<String>>,
}

impl<N> Signatur<N>
where
    N: Nummer<IntSize = u64>,
{
}
