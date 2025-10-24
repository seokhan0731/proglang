use lalrpop_util::lalrpop_mod;

pub mod ast;
use ast::Expr;
use ast::Expr::{Neg, Num, Op};
use ast::Opcode::{Add, Sub};

lalrpop_mod!(pub ae);

fn interp(e: Box<Expr>) -> i32 {
    match *e {
        Op(l, Add, r) => interp(l) + interp(r),
        Op(l, Sub, r) => interp(l) - interp(r),
        Neg(exp) => -1 * interp(exp),
        Num(n) => n,
    }
}

fn desugar(e: Box<Expr>) -> Box<Expr> {
    match *e {
        Op(l, Add, r) => Box::new(Op(desugar(l), Add, desugar(r))),
        Op(l, Sub, r) => Box::new(Op(desugar(l), Sub, desugar(r))),
        //-<expr>->0 - <expr> 이런 느낌인듯? ast 수준에서 해결하기 위해서?
        Neg(e) => Box::new(Op(Box::new(Num(0)), Sub, desugar(e))),
        Num(n) => Box::new(Num(n)),
    }
}

fn main() {
    let e0 = ae::ExprParser::new().parse("(-(5 - 1) + 3)").unwrap();

    println!("e0: {}", e0);
    println!("e0 AST: {:?}", e0);
    println!("interp(e0): {}", interp(e0.clone()));
    println!("");

    let e1 = desugar(e0.clone());
    println!("e1=desugar(e0): {}", e1);
    println!("e1 AST: {:?}", e1);
    println!("interp(e1): {}", interp(e1));
}
