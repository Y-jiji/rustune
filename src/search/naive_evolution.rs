use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::vec;
use super::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::iter::zip;

// naive evolution search
pub struct NaiveEvolution<T: Iterator, M: Ord>
where T::Item: Mix+Ord+Clone {
    /// evalutated specimen
    keep: BinaryHeap<(Reverse<M>, T::Item)>,
    /// queueing items
    candidate: vec::IntoIter<T::Item>,
    /// size for specimen for each generation
    keep_size: usize, 
    /// generation count, generation cap
    gcount: usize, gcap: usize,
    /// count item in current generation
    icount: usize, icap: usize,
}

pub trait Mix {
    fn mix(x: Self, y: Self) -> Self;
}

impl<T: Iterator, M: Ord> NaiveEvolution<T, M>
where T::Item: Mix+Ord+Clone {
    pub
    fn new(candidate: Vec<T::Item>, size: usize, gcap: usize, icap: usize) -> Self {
        Self {
            keep: BinaryHeap::new(),
            candidate: candidate.into_iter(),
            keep_size: size,
            gcount: 0, gcap,
            icount: 0, icap,
        }
    }
}

impl<T: Iterator, M: Ord+Clone> Search for NaiveEvolution<T, M>
where T::Item: Mix+Ord+Clone {
    type Input = T::Item;
    type Measure = M;
    fn get(&mut self) -> Option<Self::Input> {
        self.icount += 1;
        self.candidate.next()
    }
    fn put(&mut self, input: Self::Input, measure: Self::Measure) {
        self.keep.push((Reverse(measure), input));
        if self.keep.len() > self.keep_size { self.keep.pop(); }
        if self.icount < self.icap { return; }
        if self.gcount == self.gcap { return; }
        self.icount = 0;
        self.gcount += 1;
        // generate new candidate items
        self.candidate = {
            let keep = self.keep.iter().map(|(_, x)| x.clone()).collect::<Vec<T::Item>>();
            let mut keep_shuffle = keep.clone();
            keep_shuffle.shuffle(&mut thread_rng());
            let keep = keep.into_iter().cycle();
            let keep_shuffle = keep_shuffle.into_iter().cycle();
            let mixed = zip(keep,keep_shuffle)
                .take(self.icap)
                .map(|(x, y)| {Mix::mix(x, y)});
            mixed.collect::<Vec<_>>().into_iter()
        }
    }
    fn best(self) -> (Self::Input, Self::Measure) {
        let (Reverse(measure), input) = self.keep.into_iter().nth_back(0).unwrap();
        (input, measure)
    }
}