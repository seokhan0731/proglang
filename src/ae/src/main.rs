use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::{add, sub, num} ;
use ast::Expr::{Op, Num} ;
use ast::Opcode::{Add, Sub} ;

lalrpop_mod!(pub ae) ;

//pdf 15p를 적용한 코드
fn interp (e: Box<Expr>) -> i32 
{
    match *e {
        Op(l, Add, r) => interp(l) + interp(r),
        Op(l, Sub, r) => interp(l) - interp(r),
        Num(n) => n
    } 
}


fn main() 
{
    //e0->((5-1)+3)의 ast? ->e1과 동일한 동작(e1은 도우미 함수로 대신한 느낌)
    let e0 = Box::new( Op( Box::new( Op(Box::new( Num(5) ), 
                                     Sub, 
                                     Box::new( Num(1) )) 
                                    ), 
                           Add, 
                           Box::new( Num(3) ))
                      ) ;
    println!("e0: {}", e0) ;
    println!("interp(e0): {}", interp(e0)) ;
    println!("") ;

    //도우미 함수로 구현한 동일한 코드
    let e1 = add(sub(num(5), num(1)), num(3)) ;    
    println!("e1: {}", e1) ;
    println!("interp(e1): {}", interp(e1)) ;
    println!("") ;

    //이건 lalrpop의 파서 돌려서 만든 동일한 코드
    let e2 = ae::ExprParser::new().parse("((5 - 1) + 3)").unwrap() ;
	println!("e2: {}", e2) ;
    println!("interp(e2): {}", interp(e2)) ;
}
