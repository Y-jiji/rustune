pub trait Search
where Self::Input: Clone {
    type Input;
    type Measure;
    fn put(&mut self, input: Self::Input, measure: Self::Measure);
    fn get(&mut self) -> Option<Self::Input>;
    fn best(self) -> (Self::Input, Self::Measure);
}

mod grid;
pub use grid::*;

mod naive_evolution;
pub use naive_evolution::*;