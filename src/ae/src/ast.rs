use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    /*Box로 표현한 이유는 Expr enum 안에 Expr로 정의가 되어있기 때문에,
    크기가 무한해야하기 떄문에 러스트에서는 허용 안함->레퍼런스로 표현해야 가능
    ->Box로 표현한것
    Box->힙메모리에 새롭게 잡으면서 레퍼런스로 힙의 시작 주소를 나타낸다?
    (자바의 레퍼런스 자료형이랑 같은 개념일까?)－＞맞는듯?
    */
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Add,
    Sub,
}

/*
Expr::= Num | Expr + Expr | Expr - Expr
->이게 사실 Expr Opcode Expr로 축약해서 표현 가능(Opcode로 +- 다 구현할 수 있으니까)
Num::= [0-9]+

*/

//pub type ExprBox = Box<Expr> ;

//그냥 표현을 간략하게 하기 위해서, 만든 도우미 함수들 느낌
pub fn add(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Op(l, Opcode::Add, r))
}

pub fn sub(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Op(l, Opcode::Sub, r))
}

pub fn num(n: i32) -> Box<Expr> {
    Box::new(Expr::Num(n))
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
        }
    }
}
