//02_자료형의 중점
fn main () {
    /* 
     * scalar types: https://doc.rust-lang.org/book/ch03-02-data-types.html
     */

    /*  02_scalar.rs    러스트의 자료형 관련
    자료형->정수,실수, 문자, bool
    정수->i8,i16,i32,i64 식으로 i다음에 bit 수로 적는다! default는 i32 (unsigned는 u붙여서)
    실수->f32,f64 식으로 f다음에 bit 수! default는 f64
    bool->true, false (cpp과 동일)
    문자->char    (다른 언어와 다르게 얘는 1byte가 아님->유니코드, 이모지까지 다 가능)

    *타입 캐스팅->같은 정수, 실수여도 bit수 다르면 캐스팅 불가능
        ->as 키워드를 통한 명시적 형변환 a as i64 이런식으로 사용해야된다

     */
    let a : i16 = 100 ;
    let b : f64 = 3.14 ;
    println!("a: {}, b: {}", a, b) ;
    
    let c = b - 1.01 ;
    let d = a as f64 + b ;
    println!("c: {}, d: {}", c, d) ;
    
    let e = true ;
    let f : bool = false ;
    let g : char = '😊' ;
    println!("e: {}, f: {}, g: {}", e, f, g) ;
    
    let h = i8::MIN ;
    let i = i8::MAX ;
    println!("h: {}, i: {}", h, i) ;
}
