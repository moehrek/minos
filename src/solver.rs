use crate::{Clause, Cnf, Literal, Variable, Satisfiability};

pub struct MinosSolver {
    cnf: Cnf,
    assignments: Vec<Option<bool>>,
}

impl MinosSolver {
    pub fn from_cnf(cnf: Cnf) -> Self {
        let num_variables = cnf.num_variables;

        Self {
            cnf: cnf,
            assignments: vec![None; num_variables as usize],
        }
    }

    pub fn get_assignment(&self, literal: &Literal) -> Option<bool> {
        let assignment = self.assignments[(literal.variable.0 - 1) as usize];

        return assignment;
    }

    pub fn solve(&mut self) -> Satisfiability {
        self.dpll()
    }

    fn dpll(&mut self) -> Satisfiability {
        if let Some(is_satisfied) = self.cnf.is_satisfied(&self.assignments) {
            return is_satisfied;
        }

        let unit_propagations = self.cnf.get_unit_clauses(&self.assignments);
        for literal in unit_propagations.iter() { 
            //println!("Unit propagation: {}{}", if literal.polarity { "" } else { "-" }, literal.variable.0);

            match self.assignments[(literal.variable.0 - 1) as usize] {
                Some(assignment) => {
                    if assignment != literal.polarity {
                        return Satisfiability::Unsatisfiable;
                    }
                },
                None => self.assignments[(literal.variable.0 - 1) as usize] = Some(literal.polarity),
            }
        }

        match self.assignments.iter().position(|assignment| assignment.is_none()) {
            Some(variable_index) => {
                self.assignments[variable_index] = Some(true);
                let true_branch = self.dpll();

                if true_branch == Satisfiability::Satisfiable {
                    return Satisfiability::Satisfiable;
                }

                self.assignments[variable_index] = Some(false);
                let false_branch = self.dpll();

                self.assignments[variable_index] = None;

                for literal in unit_propagations.iter() {
                    //println!("Undoing unit propagation: {}{}", if !literal.polarity { "-" } else { "" }, literal.variable.0);
                    self.assignments[(literal.variable.0 - 1) as usize] = None;
                }

                return false_branch;
            },
            None => {
                return self.cnf.is_satisfied(&self.assignments).unwrap();
            }
        }
    }

    fn print_assignments(&self) {
        print!("[");
        for (index, assignment) in self.assignments.iter().enumerate() {
            if assignment.is_none() {
                continue;
            }
            print!("{}{}, ", if assignment.unwrap() { "" } else { "-" }, index + 1);
        }
        println!("]");
    }
}
