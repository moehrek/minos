use std::collections::HashMap;

use crate::{Cnf, Literal, Variable, Clause};

pub fn preprocess(cnf: &Cnf) -> Cnf {
    let mut unit_literals: HashMap<Variable, bool> = HashMap::new();
    let mut new_clauses: Vec<Clause> = Vec::with_capacity(cnf.clauses.len());

    for clause in cnf.clauses.iter() {
        if clause.0.len() == 1 {
            let unit_literal = &clause.0[0];
            unit_literals.insert(unit_literal.variable, unit_literal.polarity);
        }
    }

    for clause in cnf.clauses.iter() {
        let mut new_clause = Clause(Vec::new());
        let mut append_clause = true;

        for literal in clause.0.iter() {
            match (unit_literals.get(&literal.variable), literal.polarity) {
                (Some(&true), true) | (Some(&false), false) => {
                    append_clause = false;
                    break;
                },
                (Some(&false), true) | (Some(&true), false)  => {
                    continue;
                },
                (None, _) => {}
            }

            new_clause.0.push(literal.clone());
        }

        if append_clause {
            new_clauses.push(new_clause);
        }
    }

    Cnf {
        num_variables: cnf.num_variables,
        clauses: new_clauses
    }
}