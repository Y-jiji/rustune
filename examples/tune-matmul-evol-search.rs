use std::env::current_dir;
use std::process::Command;
use std::path::PathBuf;
use rustune::*;
use std::vec;

struct MatMulModal {
    out_path: PathBuf,
}

#[derive(Debug)]
enum MatMulError {
    ExecutableNotCompiled,
    ExecutionFailed(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Config(&'static str, usize);

impl Mix for Config {
    fn mix(x: Self, y: Self) -> Self {
        let Config(sx, lx) = x;
        let Config(sy, ly) = y;
        Config(sx, ly)
    }
}

impl Program for MatMulModal {
    type Input = Config;
    type Output = ();
    type Error = MatMulError;
    type Measure = i128;
    fn compile(&mut self, src_path: PathBuf, out_path: PathBuf) -> Result<(), Self::Error> {
        self.out_path = out_path.clone();
        for olevel in ["-O0", "-O1", "-O2", "-O3"] {
            let out = out_path.to_str().unwrap().to_owned() + olevel;
            let program = Command::new("clang").args([
                src_path.to_str().unwrap(), "-o", out.as_str(), olevel]).spawn().unwrap().wait();
            match program {
                Ok(_) => {},
                Err(_) => Err(MatMulError::ExecutableNotCompiled)?,
            }
        }
        Ok(())
    }
    fn run(&mut self, input: Self::Input) -> Result<(Self::Output, Self::Measure), Self::Error> {
        let executable_path = self.out_path.to_str().unwrap().to_owned() + input.0;
        let mut command = Command::new(executable_path);
        let one_run = command.args([format!("{}", input.1)]);
        match one_run.output() {
            Ok(x) => {
                let stdout = x.to_owned().stdout;
                let stdout = std::str::from_utf8(&stdout).unwrap().strip_suffix("\n").unwrap();
                let neg_time = match str::parse::<f64>(stdout) {
                    Ok(t) => {println!("{input:?}, {t}"); -(t*1000000f64) as i128},
                    Err(e) => {println!("{stdout}, {e}"); i128::MIN},
                };
                Ok(((), neg_time))
            },
            Err(e) => Err(MatMulError::ExecutionFailed(format!("{}", e)))?,
        }
    }
}

fn main() -> std::io::Result<()> {
    let current_dir = current_dir()?.join("examples");
    println!("current dir {current_dir:?}");
    let mut matmul_program = MatMulModal { out_path: PathBuf::from("") };
    matmul_program.compile(current_dir.join("matmul.c"), current_dir.join("matmul")).unwrap();
    let init_candidate = vec![Config("-O0", 128), Config("-O1", 64), Config("-O1", 32), Config("-O2", 16), Config("-O3", 8)];
    let search = NaiveEvolution::<vec::IntoIter<_>, _>::new(init_candidate, 5, 3, 5);
    let tuner = Tuner { program: matmul_program, search };
    let (best_cfg, max_score) = tuner.start();
    println!("best configuration {:?}", best_cfg);
    println!("min run time (sec) {:?}", (-max_score as f64) / 1000000f64);
    Ok(())
}