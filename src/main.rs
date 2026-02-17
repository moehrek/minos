use std::env;
use std::path::Path;

use minos::solver::MinosSolver;
use minos::{Cnf, Satisfiability};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    println!("Reading CNF from file {}", path.display());

    let cnf: Cnf = Cnf::from_file(&path).expect("dimacs file is valid");

    //println!("{:?}", cnf);

    let mut solver = MinosSolver::from_cnf(cnf);

    let result = solver.solve();
    println!("Result: {:?}", result);
}
