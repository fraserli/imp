#[derive(Debug, Clone)]
pub enum Expr {
    Skip,
    Boolean(bool),
    Integer(i64),
    Dereference(Box<str>),
    Assignment(Box<Assignment>),
    Operation(Box<Operation>),
    IfThenElse(Box<IfThenElse>),
    WhileLoop(Box<WhileLoop>),
    Sequence(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub location: Box<str>,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub op: Op,
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    GreaterEqual,
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub first: Expr,
    pub second: Expr,
}

#[derive(Debug, Clone)]
pub struct IfThenElse {
    pub predicate: Expr,
    pub consequent: Expr,
    pub alternative: Expr,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub predicate: Expr,
    pub body: Expr,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => f.write_str("+"),
            Op::GreaterEqual => f.write_str(">="),
        }
    }
}

impl Expr {
    pub fn skip() -> Self {
        Self::Skip
    }

    pub fn boolean(b: bool) -> Self {
        Self::Boolean(b)
    }

    pub fn integer(n: i64) -> Self {
        Self::Integer(n)
    }

    pub fn dereference<T: AsRef<str>>(s: T) -> Self {
        Self::Dereference(s.as_ref().into())
    }

    pub fn assignment<T: AsRef<str>>(s: T, value: Expr) -> Self {
        Self::Assignment(Box::new(Assignment {
            location: s.as_ref().into(),
            value: value,
        }))
    }

    pub fn operation(op: Op, lhs: Expr, rhs: Expr) -> Self {
        Self::Operation(Box::new(Operation { op, lhs, rhs }))
    }

    pub fn if_then_else(predicate: Expr, consequent: Expr, alternative: Expr) -> Self {
        Self::IfThenElse(Box::new(IfThenElse {
            predicate,
            consequent,
            alternative,
        }))
    }

    pub fn while_loop(predicate: Expr, body: Expr) -> Self {
        Self::WhileLoop(Box::new(WhileLoop { body, predicate }))
    }

    pub fn sequence<I: IntoIterator<Item = Expr>>(it: I) -> Self {
        Self::Sequence(it.into_iter().collect())
    }

    pub fn sexp(&self) -> String {
        match self {
            Expr::Skip => "skip".to_owned(),
            Expr::Boolean(b) => match b {
                true => "true",
                false => "false",
            }
            .to_owned(),
            Expr::Integer(n) => n.to_string(),
            Expr::Dereference(location) => format!("!{location}"),
            Expr::Assignment(box Assignment { location, value }) => {
                format!("(= {location} {})", value.sexp())
            }
            Expr::Operation(box Operation { op, lhs, rhs }) => {
                format!("({op} {} {})", lhs.sexp(), rhs.sexp())
            }
            Expr::IfThenElse(box IfThenElse {
                predicate,
                consequent,
                alternative,
            }) => format!(
                "(if {} {} {})",
                predicate.sexp(),
                consequent.sexp(),
                alternative.sexp()
            ),
            Expr::WhileLoop(box WhileLoop { predicate, body }) => {
                format!("(while {} {})", predicate.sexp(), body.sexp())
            }
            Expr::Sequence(expressions) => format!(
                "(; {})",
                expressions
                    .iter()
                    .map(Expr::sexp)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }

    fn display_with_parens(&self) -> bool {
        match self {
            Expr::Skip
            | Expr::Boolean(_)
            | Expr::Integer(_)
            | Expr::Dereference(_)
            | Expr::Operation(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct E<'a>(&'a Expr);

        impl<'a> std::fmt::Display for E<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0.display_with_parens() {
                    write!(f, "{}", self.0)
                } else {
                    write!(f, "({})", self.0)
                }
            }
        }

        match self {
            Self::Skip => f.write_str("skip"),
            Expr::Boolean(b) => f.write_str(if *b { "true" } else { "false" }),
            Expr::Integer(n) => f.write_str(&n.to_string()),
            Expr::Dereference(location) => write!(f, "!{location}"),
            Expr::Assignment(box Assignment { location, value }) => {
                write!(f, "{} = {}", location, E(value))
            }
            Expr::Operation(box Operation { op, lhs, rhs }) => {
                write!(f, "{} {op} {}", E(lhs), E(rhs))
            }
            Expr::IfThenElse(box IfThenElse {
                predicate,
                consequent,
                alternative,
            }) => write!(
                f,
                "if {} then {} else {}",
                E(predicate),
                E(consequent),
                E(alternative)
            ),
            Expr::WhileLoop(box WhileLoop { predicate, body }) => {
                write!(f, "while {} do {}", E(predicate), E(body))
            }
            Expr::Sequence(expressions) => f.write_str(
                &expressions
                    .iter()
                    .map(|e| format!("{e}"))
                    .collect::<Vec<_>>()
                    .join("; "),
            ),
        }
    }
}
