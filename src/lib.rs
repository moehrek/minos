use std::collections::HashSet;
use std::ops::BitOr;

pub mod parser;
pub mod solver;
pub mod preprocessing;

#[derive(Debug, PartialEq)]
pub enum Satisfiability {
    Satisfiable,
    Unsatisfiable,
}

impl BitOr for Satisfiability {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Satisfiability::Satisfiable, _) | (_, Satisfiability::Satisfiable) => {
                Satisfiability::Satisfiable
            }
            _ => Satisfiability::Unsatisfiable,
        }
    }
}

impl std::fmt::Display for Satisfiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Satisfiability::Satisfiable => write!(f, "SATISFIABLE"),
            Satisfiability::Unsatisfiable => write!(f, "UNSATISFIABLE"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Variable(u64);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Literal {
    variable: Variable,
    polarity: bool,
}

#[derive(Debug, Clone)]
pub struct Clause(Vec<Literal>);

#[derive(Debug)]
pub struct Cnf {
    num_variables: u64,
    clauses: Vec<Clause>,
}

impl Cnf {
    pub fn is_satisfied(&self, assignments: &Vec<Option<bool>>) -> Option<Satisfiability> {
        for clause in self.clauses.iter() {
            let mut all_literals_assigned = true;
            let mut clause_satisfied = false;

            for literal in clause.0.iter() {
                let assignment = assignments[(literal.variable.0 - 1) as usize];

                match (assignment, literal.polarity) {
                    (Some(true), true) | (Some(false), false) => {
                        clause_satisfied = true;
                        break;
                    }
                    (None, _) => {
                        all_literals_assigned = false;
                    }
                    _ => {}
                }
            }

            match (all_literals_assigned, clause_satisfied) {
                (true, false) => return Some(Satisfiability::Unsatisfiable),
                (false, false) => return None,
                _ => {}
            }
        }

        Some(Satisfiability::Satisfiable)
    }

    pub fn get_unit_clauses(&self, assignments: &Vec<Option<bool>>) -> Vec<Literal> {
        let mut unit_clauses = vec![];

        for clause in self.clauses.iter() {
            let mut unassigned_literals = vec![];
            let mut clause_satisfied = false;

            for (index, literal) in clause.0.iter().enumerate() {
                let assignment = assignments[(literal.variable.0 - 1) as usize];

                match assignment {
                    None => unassigned_literals.push(index),
                    Some(assignment) => {
                        if assignment != literal.polarity {
                            continue;
                        }

                        clause_satisfied = true;
                        break;
                    }
                }
            }    

            if !clause_satisfied && unassigned_literals.len() == 1 {
                unit_clauses.push(clause.0[unassigned_literals[0]].clone());
            }
        }

        unit_clauses
    }
}

#[cfg(test)]
mod tests {
    use crate::{Clause, Cnf, Literal, Variable, Satisfiability};

    #[test]
    fn test_cnf_is_satisfied() {
        // (x1 OR NOT x2) AND (x2 OR x3)
        let cnf = Cnf {
            num_variables: 3,
            clauses: vec![
                Clause(vec![
                    Literal {
                        variable: Variable(1),
                        polarity: true,
                    },
                    Literal {
                        variable: Variable(2),
                        polarity: false,
                    },
                ]),
                Clause(vec![
                    Literal {
                        variable: Variable(2),
                        polarity: true,
                    },
                    Literal {
                        variable: Variable(3),
                        polarity: true,
                    },
                ]),
            ],
        };

        assert_eq!(
            cnf.is_satisfied(&vec![Some(true), Some(true), Some(true)]),
            Some(Satisfiability::Satisfiable) 
        );
        assert_eq!(
            cnf.is_satisfied(&vec![Some(false), Some(false), Some(true)]),
            Some(Satisfiability::Satisfiable)
        );
        assert_eq!(
            cnf.is_satisfied(&vec![Some(false), Some(true), Some(true)]),
            Some(Satisfiability::Unsatisfiable)
        );
        assert_eq!(
            cnf.is_satisfied(&vec![Some(true), Some(false), Some(false)]),
            Some(Satisfiability::Unsatisfiable)
        );
    }

    #[test]
    fn test_cnf_partially_assigned() {
        // (x1 OR NOT x2) AND (x2 OR x3)
        let cnf = Cnf {
            num_variables: 3,
            clauses: vec![
                Clause(vec![
                    Literal {
                        variable: Variable(1),
                        polarity: true,
                    },
                    Literal {
                        variable: Variable(2),
                        polarity: false,
                    },
                ]),
                Clause(vec![
                    Literal {
                        variable: Variable(2),
                        polarity: true,
                    },
                    Literal {
                        variable: Variable(3),
                        polarity: true,
                    },
                ]),
            ],
        };

        assert_eq!(cnf.is_satisfied(&vec![Some(true), None, None]), None);
        assert_eq!(cnf.is_satisfied(&vec![None, None, Some(true)]), None);
        assert_eq!(cnf.is_satisfied(&vec![None, None, None]), None);
    }

    #[test]
    fn test_one_unit_clause() {
        // (x1) AND (NOT x1 OR x2)
        let cnf = Cnf {
            num_variables: 3,
            clauses: vec![
                Clause(vec![Literal {
                    variable: Variable(1),
                    polarity: true,
                }]),
                Clause(vec![
                    Literal {
                        variable: Variable(1),
                        polarity: false,
                    },
                    Literal {
                        variable: Variable(2),
                        polarity: true,
                    },
                ]),
            ],
        };

        assert_eq!(
            cnf.is_satisfied(&vec![Some(true), None]),
            None 
        );
    }
}