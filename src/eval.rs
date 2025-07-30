use std::collections::BTreeMap;

use crate::ast::{Assignment, Expr, IfThenElse, Op, Operation, WhileLoop};

impl Expr {
    pub fn can_transition(&self) -> bool {
        match self {
            Self::Skip | Self::Boolean(_) | Self::Integer(_) => false,
            Self::Dereference(_)
            | Self::Assignment(_)
            | Self::Operation(_)
            | Self::IfThenElse(_)
            | Self::WhileLoop(_)
            | Self::Sequence(_) => true,
        }
    }

    pub fn transition(&mut self, store: &mut BTreeMap<Box<str>, Self>) {
        match self {
            Self::Skip | Self::Boolean(_) | Self::Integer(_) => {}
            Self::Dereference(location) => *self = store.get(location.as_ref()).unwrap().clone(),
            Self::Assignment(box Assignment { location, value }) => {
                if value.can_transition() {
                    value.transition(store);
                } else {
                    let location = std::mem::take(location);
                    let value = std::mem::take(value);

                    store.insert(location, value);
                    *self = Self::skip();
                }
            }
            Self::Operation(box Operation { op, lhs, rhs }) => {
                if lhs.can_transition() {
                    lhs.transition(store);
                } else if rhs.can_transition() {
                    rhs.transition(store);
                } else {
                    match (lhs, op, rhs) {
                        (Self::Integer(a), Op::Add, Self::Integer(b)) => {
                            *self = Self::integer(*a + *b)
                        }
                        (Self::Integer(a), Op::GreaterEqual, Self::Integer(b)) => {
                            *self = Self::boolean(a >= b)
                        }
                        (_, Op::Add, _) => panic!("invalid operands for addition"),
                        (_, Op::GreaterEqual, _) => panic!("invalid operands for comparison"),
                    }
                }
            }
            Self::IfThenElse(box IfThenElse {
                predicate,
                consequent,
                alternative,
            }) => {
                if predicate.can_transition() {
                    predicate.transition(store);
                } else {
                    *self = std::mem::take(match predicate {
                        Self::Boolean(true) => consequent,
                        Self::Boolean(false) => alternative,
                        _ => panic!("expected boolean"),
                    });
                }
            }
            Self::WhileLoop(box WhileLoop { predicate, body }) => {
                *self = Expr::if_then_else(
                    predicate.clone(),
                    Expr::sequence([body.clone(), std::mem::take(self)]),
                    Self::skip(),
                );
            }
            Self::Sequence(exprs) => {
                if matches!(exprs[0], Self::Skip | Self::Boolean(_) | Self::Integer(_)) {
                    // TODO: O(n)
                    exprs.remove(0);
                } else {
                    exprs[0].transition(store);
                }

                if exprs.len() == 1 {
                    *self = exprs.remove(0);
                }
            }
        }
    }
}
