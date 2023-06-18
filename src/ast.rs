use std::fmt::{self, Debug, Formatter};


pub enum AST {
    Value(i32), // TEMP
    ID(String),
    BinOp(Box<AST>, Op, Box<AST>),
    Decl(String),
    ASTs(Vec<Box<AST>>)
}

pub enum Op {
    Plus,
    Minus,
    Times,
    DivBy,
    Assign
}


impl Debug for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Value(val) => write!(f, "{val}"),
            Self::ID(name) => write!(f, "{name}"),
            Self::BinOp(lhs, op, rhs) =>
                write!(f, "({:?} {:?} {:?})", *lhs, op, *rhs),
            Self::Decl(name) => write!(f, "let {name}"),
            Self::ASTs(asts) => {
                for ast in asts.iter() {
                    write!(f, "{ast:?};")?
                }
                fmt::Result::Ok(())
            }
        }
    }
}

impl Debug for Op {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Times => write!(f, "*"),
            Self::DivBy => write!(f, "/"),
            Self::Assign => write!(f, ":=")
        }
    }
}
