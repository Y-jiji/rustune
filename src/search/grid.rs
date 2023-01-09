use super::*;
use std::iter::{Iterator, Zip};
use itertools::*;

pub struct GridSeach<T: Iterator, M: Ord> {
    grid: T,
    best: (T::Item, M),
}

impl<T: Iterator, M: Ord> GridSeach<T, M>
where T::Item: Clone {
    pub 
    fn new(iter: T, default: (T::Item, M)) -> Self {
        Self { grid: iter,  best: default}
    }
    // eat one grid search object and get a zipped object
    pub
    fn zip<F: Iterator>(self, other: F, default: F::Item) -> GridSeach<Zip<T, F>, M> {
        GridSeach {grid: self.grid.zip(other), best: ((self.best.0, default), self.best.1)}
    }
    pub
    // eat on grid search object and get a product object
    fn prod<F: Iterator+Clone>(self, other: F, default: F::Item) -> GridSeach<Product<T, F>, M> {
        GridSeach { grid: iproduct!(self.grid, other), best: ((self.best.0, default), self.best.1) }
    }
    // set best of self
    pub
    fn set_best(&mut self, best: (T::Item, M)) {
        self.best = best;
    }
}

impl<T: Iterator, M: Ord> Search for GridSeach<T, M> 
where T::Item: Clone {
    type Input = T::Item;
    type Measure = M;
    fn get(&mut self) -> Option<Self::Input> {
        return self.grid.next();
    }
    fn put(&mut self, input: Self::Input, measure: Self::Measure) {
        if measure > self.best.1 {self.best = (input, measure);}
    }
    // move out of self to get the best
    fn best(self) -> (Self::Input, Self::Measure) {
        self.best
    }
}