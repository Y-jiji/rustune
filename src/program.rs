use std::fmt::Debug;
use std::path::*;

/// an instance satisfying ProgramTest 
pub trait Program
where Self::Error: Debug,
      Self::Input: Clone {
    type Input;
    type Output;
    type Error;
    type Measure;
    /// compile
    fn compile(&mut self, src_path: PathBuf, out_path: PathBuf) -> Result<(), Self::Error>;
    /// define the pipeline of the test from Input, returns output and measured time
    fn run(&mut self, input: Self::Input) -> Result<(Self::Output, Self::Measure), Self::Error>;
}