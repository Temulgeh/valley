use std::collections::hash_map::HashMap;
use crate::ast::*;

pub struct Interpreter {
    env: HashMap<String, i32>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { env: HashMap::new() }
    }

    pub fn eval(&mut self, ast: &AST) -> i32 {
        match ast {
            AST::Value(val) => *val,
            // should panic if the name doesn't exist, as the Checker should
            // have checked that it's fine to use. for now, uses 0
            AST::ID(name) => match self.env.get(name) {
                Some(value) => *value,
                None => 0
            },
            AST::BinOp(lhs, op, rhs) => match op {
                Op::Plus => self.eval(lhs) + self.eval(rhs),
                Op::Minus => self.eval(lhs) - self.eval(rhs),
                Op::Times => self.eval(lhs) * self.eval(rhs),
                Op::DivBy => self.eval(lhs) / self.eval(rhs),
                // returns 0 for now, will return unit when types are a thing
                Op::Assign => {
                    let value = self.eval(rhs);
                    self.assign(lhs, value);
                    0
                }
            },
            AST::Decl(name) => {
                // TODO: mutable and unmutable as two separate things
                self.env.insert(name.to_string(), 0);
                0
            }
            AST::ASTs(asts) => {
                let mut it = asts.iter().peekable();
                while let Some(ast) = it.next() {
                    let val = self.eval(ast);
                    if it.peek().is_none() {
                        return val;
                    }
                }
                unreachable!();
            }
        }
    }

    fn assign(&mut self, lhs: &AST, value: i32) {
        // does not check that the variable exists because the Checker will do
        // it when it exists; at that point this should panic if the variable
        // doesn't exist
        if let AST::ID(name) = lhs {
            self.env.insert(name.to_string(), value);
        }
    }
}

