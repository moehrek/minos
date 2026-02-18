use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::Path;

use crate::{Clause, Cnf, Literal, Variable};

impl Cnf {
    pub fn from_string(dimacs_string: &str) -> Result<Self, String> {
        let mut lines = dimacs_string
            .lines()
            .filter(|line| !line.is_empty() && line.chars().nth(0).unwrap() != 'c');

        let mut dimacs_header = lines.next().unwrap().split_whitespace();

        let first_token = dimacs_header.next().unwrap();

        if first_token != "p" {
            return Err(format!("Invalid DIMACS header: expected 'p', got '{}'", first_token));
        }

        let clause_format = dimacs_header
            .next()
            .ok_or("No clause format was supplied in DIMACS header".to_owned())?;

        if clause_format != "cnf" {
            return Err(format!("{clause_format} is not a supported format").to_owned());
        }

        let num_variables = dimacs_header
            .next()
            .ok_or("No number of variables was supplied in DIMACS header")?
            .parse::<u64>()
            .map_err(|_| "Invalid number of variables in DIMACS header".to_owned())?;

        let num_clauses = dimacs_header
            .next()
            .ok_or("No number of clauses was supplied in DIMACS header")?
            .parse::<u64>()
            .map_err(|_| "Invalid number of clauses variables in DIMACS header".to_owned())?;

        let clauses: Vec<Clause> = lines
            .map(|line| -> Result<Clause, String> {
                let mut literals: Vec<Literal> = vec![];
                let literal_strings = line.split_whitespace();

                for literal_string in literal_strings {
                    if literal_string == "0" {
                        break;
                    }

                    let raw_literal = literal_string.parse::<i64>().map_err(|_| {
                        format!("{literal_string} is not a valid literal").to_owned()
                    })?;

                    literals.push(Literal {
                        variable: Variable(raw_literal.abs() as u64),
                        polarity: raw_literal > 0,
                    })
                }

                Ok(Clause(literals))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Cnf {
            num_variables,
            clauses,
        })
    }

    pub fn from_file(path: &Path) -> Result<Self, String> {
        let dimacs_string: String = fs::read_to_string(path).map_err(|err| err.to_string())?;

        let cnf = Cnf::from_string(&dimacs_string)?;

        Ok(cnf)
    }
}
