use fxhash::FxHasher;
use std::{
    hash::{ Hash, Hasher },
    time::{ SystemTime, UNIX_EPOCH },
};
use crate::ZUGANGSNUMMERN;

pub fn build_seq_id<I, T>(data: I) -> u64
where
    T: Hash + Clone,
    I: std::iter::IntoIterator<Item = T>,
{
    let mut hasher = FxHasher::default();
    data.into_iter()
        .for_each(|x| x.hash(&mut hasher));
    hasher.finish()
}

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time stopped working")
        .as_secs()
}

pub fn get_znr() -> usize {
    let mut reg = ZUGANGSNUMMERN.lock()
        .expect("Cannot lock LAUFNUMMERN");
    let new_nr = reg.len()  + 1;
    reg.push(new_nr);

    new_nr
}
