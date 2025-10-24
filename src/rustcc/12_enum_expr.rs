use ExprType::Num ;
use ExprType::Expr ;
use Opr::Plus ;
use Opr::Product ;

/*  사실상 ast구조 정의, ast구조에서 semantics를 통해 의미에 대응되는 과정까지
    원큐에 잡혀있는 코드임

    1.ExprType에 재귀적 구조->grammer가 재귀적으로 구성된 것과 같은 의미
    2. eval함수 문법구조를 semantics를 바탕으로 해석하는 함수
    3. e변수 생성 방식 정도 봐주면 된다.
 */
enum Opr {
    Plus,
    Product
}

enum ExprType {
    Num(i32),
    Expr(Box<ExprType>, Opr, Box<ExprType>)
}

/*  표현
ExprType::=<Num> | <Expr> <Opr> <Expr>
Opr::=<plus><product>
<plus>::='+'
<product>::='*'

 */

/* 만약 eval에서 레퍼런스타입으로 매개변수를 받지 않았따면 match문 쓸 때 *e로 사용
    결국 함수는 실값(value)을 리턴해야하기 때문에, 재귀함수를 통해서, 정수값 리턴하게함
    (expr)이 매칭되었을 때,
 */
fn eval (e: &ExprType) -> i32 {
    match e {
        Num(n) => *n,
        Expr(l, Plus, r) => eval(l) + eval(r),
        Expr(l, Product, r) => eval(l) * eval(r),
    }
}

fn main () 
{
    /*  바깥쪽부터 구조를 잡으면
    Expr1:(3,*,2)
    ->Expr2: expr1 +2
     따라서 최종값 8이 구조네
     */
    let e = Expr(Box::new(Expr(Box::new(Num(3)), Product, Box::new(Num(2)))), Plus, Box::new(Num(2))) ;

    println!("{}", eval(&e)) ;
}
