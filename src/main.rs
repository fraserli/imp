use std::fs::File;
use std::io::{Read, Write};

use chumsky::Parser;

fn parse(src: &str) {
    match imp::parse::parser().parse(&src).into_result() {
        Ok(expr) => {
            println!("{}\n{}", expr, expr.sexp());
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

        parse(&src);
    } else {
        loop {
            print!("IMP> ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            parse(&input);
        }
    }
}
