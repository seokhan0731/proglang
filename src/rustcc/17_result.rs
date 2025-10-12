use std::io;

fn main(){

    let mut input=String::new();


    io::stdin().read_line(&mut input);  //개행 전까지 한 줄 입력받기

    let r:i32=input.trim().parse(); //trim()->앞뒤 개행문자 등등 제거, parse()->i32로 파싱 의도

    /*  컴파일 오류 발생
    cuz 파싱해서 결과값을 넘겨줄 때, Result라는 걸로 넘겨주기 때문
    Result도 enum 자료형 느낌이라 생각해주면 된다->값의 가질 수 있는 가능성들의 집합
    -> let r:Result<i32, std::num::ParseIntError>
    그 후, match 문으로 정상적으로 넘어오는 경우와
    에러코드가 넘어오는 경우를 나누어서 출력해야된다.

    정상적으로 나오는 경우는 Ok(n) 이렇게 Ok에 감싸서 넘어온다.
    이 작업 맨날 해주기 귀찮으면
    input.trim().parse().unwrap()으로 해주면 된다.->일단 이정도로만 챙겨둘까?
    ->easyrust 32강? 그것도 봐줘야할듯

    */
    println!("{}",r);
}