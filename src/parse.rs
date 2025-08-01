use chumsky::pratt::*;
use chumsky::prelude::*;
use chumsky::text::*;

use crate::ast::{Expr, Op};

pub fn parser<'a>() -> impl Parser<'a, &'a str, Expr, extra::Err<Rich<'a, char>>> {
    let skip = just("skip").to(Expr::skip());

    let boolean = choice((just("true").to(true), just("false").to(false))).map(Expr::boolean);

    let integer = just('-')
        .or_not()
        .then(digits(10))
        .to_slice()
        .try_map(|s: &str, span| s.parse::<i64>().map_err(|e| Rich::custom(span, e)))
        .map(Expr::integer);

    let location = ident().map(|s: &str| Into::<Box<str>>::into(s));

    let dereference = just('!').ignore_then(location.map(Expr::dereference));

    recursive(|sequence| {
        let expression = recursive(|expression| {
            let assignment = location
                .then(just(":=").padded().ignore_then(expression.clone()))
                .map(|(location, value)| Expr::assignment(location, value));

            let if_then_else = just("if")
                .ignore_then(expression.clone())
                .then_ignore(just("then"))
                .then(expression.clone())
                .then_ignore(just("else"))
                .then(expression.clone())
                .map(|((predicate, consequent), alternative)| {
                    Expr::if_then_else(predicate, consequent, alternative)
                });

            let while_loop = just("while")
                .ignore_then(expression.clone())
                .then_ignore(just("do"))
                .then(expression)
                .map(|(predicate, body)| Expr::while_loop(predicate, body));

            let paren_expression = sequence.delimited_by(just('('), just(')'));

            let atom = choice((
                skip,
                boolean,
                integer,
                dereference,
                assignment,
                if_then_else,
                while_loop,
                paren_expression,
            ))
            .padded();

            let op = |c| just(c).padded();

            atom.pratt((
                infix(left(1), op("+"), |lhs, _, rhs, _| {
                    Expr::operation(Op::Add, lhs, rhs)
                }),
                infix(left(0), op(">="), |lhs, _, rhs, _| {
                    Expr::operation(Op::GreaterEqual, lhs, rhs)
                }),
            ))
        });

        let sequence = expression
            .clone()
            .separated_by(just(';').padded())
            .at_least(2)
            .collect::<Vec<_>>()
            .map(Expr::sequence);

        choice((sequence, expression))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(src: &str, expected: &str) {
        assert_eq!(parser().parse(src).unwrap().sexp(), expected);
    }

    #[test]
    fn parse_basic() {
        test("skip", "skip");
        test("true", "true");
        test("false", "false");
        test("123", "123");
        test("-123", "-123");
        test("!a", "!a");
    }

    #[test]
    fn parse_assignment() {
        test("a := 1", "(:= a 1)");
        test("a := 1 + 2", "(:= a (+ 1 2))");
        test("a := (b := 1; !b)", "(:= a (; (:= b 1) !b))");
        test("a := if !cond then 1 else 2", "(:= a (if !cond 1 2))");
    }

    #[test]
    fn parse_operation() {
        test("1 + 2", "(+ 1 2)");
        test("1 + 2 + 3", "(+ (+ 1 2) 3)");
        test("1 >= 2", "(>= 1 2)");
        test("1 + 2 >= 3 + 4", "(>= (+ 1 2) (+ 3 4))");
        test("(1 + 2 >= 3) + 4", "(+ (>= (+ 1 2) 3) 4)");
    }

    #[test]
    fn parse_if() {
        test("if 1 then 2 else 3", "(if 1 2 3)");
    }

    #[test]
    fn parse_while() {
        test("while 1 do 2", "(while 1 2)");
    }

    #[test]
    fn parse_complex() {
        test(
            "i := 0; a := 10; b := 0; while !a >= !b + 2 do (a := !b + 5; i := !i + 1); a := 0",
            "(; (:= i 0) (:= a 10) (:= b 0) (while (>= !a (+ !b 2)) (; (:= a (+ !b 5)) (:= i (+ !i 1)))) (:= a 0))",
        );

        test(
            "while 10 >= !i do if !b then a := 1 else a := 2; a := 3",
            "(; (while (>= 10 !i) (if !b (:= a 1) (:= a 2))) (:= a 3))",
        );

        test(
            "while 10 >= !i do (if !b then a := 1 else a := 2; a := 3)",
            "(while (>= 10 !i) (; (if !b (:= a 1) (:= a 2)) (:= a 3)))",
        );

        test(
            "while 10 >= !i do if !b then a := 1 else (a := 2; a := 3)",
            "(while (>= 10 !i) (if !b (:= a 1) (; (:= a 2) (:= a 3))))",
        );
    }
}
