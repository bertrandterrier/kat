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
        .expect("Cannot lock ZUGANGSNUMMERN");
    let new_nr: usize = match reg.last() {
        Some(i) => i + 1,
        None => 1,
    };
    reg.push(new_nr);
    new_nr
}

pub fn remove_znr(znr: usize) {
    let mut reg = ZUGANGSNUMMERN.lock()
        .expect("Cannot lock ZUSANGSNUMMERN");
    if let Some((i, _)) = reg
        .iter()
        .enumerate()
        .find(|(_, e)| *e == &znr)
    {
        reg.remove(i);
    };
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn zugangs_nummer() {
        let mut buf: Vec<usize> = Vec::new(); 
        for _ in 0..10 {
            buf.push(get_znr());
        };
        let expected: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(buf, expected);
    }
}
