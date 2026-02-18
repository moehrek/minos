use std::env;
use std::path::Path;

use minos::solver::MinosSolver;
use minos::{Cnf, Satisfiability};
use minos::preprocessing::preprocess;

fn main() {
    let cnf: Cnf;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let path = Path::new(&args[1]);

        println!("Reading CNF from file {}", path.display());

        cnf = Cnf::from_file(&path).expect("dimacs file is invalid");
    } else {
        let stdio_string = std::io::read_to_string(std::io::stdin()).unwrap();
        
        println!("Reading CNF from stdio");

        cnf = Cnf::from_string(&stdio_string).expect("stdio is not valid cnf");
    }

    let cnf = preprocess(&cnf);

    let mut solver = MinosSolver::from_cnf(cnf);

    let result = solver.solve();
    println!("s {result}");
}
