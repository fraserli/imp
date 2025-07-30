use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};

use chumsky::Parser;
use imp::ast::Expr;

fn display_program(expr: &Expr, store: &BTreeMap<Box<str>, Expr>) -> String {
    format!(
        "{}, {{{}}}",
        expr,
        store
            .iter()
            .map(|(k, v)| format!("{k} -> {v}"))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn run(src: &str, store: &mut BTreeMap<Box<str>, Expr>) {
    match imp::parse::parser().parse(&src).into_result() {
        Ok(mut expr) => {
            println!("=> {}", display_program(&expr, store));

            while expr.can_transition() {
                expr.transition(store);
                println!("=> {}", display_program(&expr, store));
            }
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        let mut file = File::open(&args[1]).unwrap();

        let mut src = String::new();
        file.read_to_string(&mut src).unwrap();

        run(&src, &mut BTreeMap::new());
    } else {
        let mut store = BTreeMap::new();

        loop {
            print!("IMP> ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            run(&input, &mut store);
        }
    }
}
