pub mod search;
use std::fmt::Debug;

pub use search::*;

pub mod program;
pub use program::*;

// an auto-tuner class
pub struct Tuner<P: Program, S: Search<Input = P::Input, Measure = P::Measure>> {
    pub program: P,
    pub search: S,
}

impl<P: Program, S: Search<Input = P::Input, Measure = P::Measure>> Tuner<P, S>
where P::Measure : Debug, P::Output : Debug {
    // consume itself and get the best tuning result
    pub fn start(mut self) -> (P::Input, P::Measure) {
        while let Some(input) = self.search.get() {
            let (_output, measure) = self.program.run(input.clone()).unwrap();
            self.search.put(input.clone(), measure);
        }
        self.search.best()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
