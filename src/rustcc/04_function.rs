/*
_04 여기서 주의 깊게 봐야될건 함수 정의 및 사용법
함수 사용법
fn 이름(매개변수:type)->리턴 type{}
코드블럭 떄와 마찬가지로 rtn 값은 ;없는 문장
만약 모든 문장에 ; 존재->그 함수의 rtn은 void라는 것
 */

fn two () -> i32 {
    2 
}   //함수 two는 매개변수는 아무것도 존재하지 않고 2를 반환합니다

fn double (n: i32) -> i32 {
    n * 2 
}   //함수 double은 매개변수로 n을 받는데, n의 type은 i32이고,
                                        //반환형의 타입도 i32, n을 *2해서 rtn한다



fn main () {
    let r = two() ; 
    println!("r: {}, double(r): {}", r, double(r)) ;

    /*
    {
        let r = 3 ;
        println!("double(r): {}", double(r)) ;

        println!("multiply(5,4): {}", multiply(5, 4))
    }
    */
}


fn multiply (fact: i32, n: i32) -> i32 {
    fact * n
}


