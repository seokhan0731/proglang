fn main () {
    /* Hello world */

    print!("Hello, world!") ;

    let rust = "Rust" ;

    println!(" {}!", rust) ;
}

/**01_hellowrodl.rs

1. fn main()->프로그램의 기본 시작점(int main()이랑 동일하게 생각하면 된다)
2. print!, println!->!붙으면 매크로 정의되어 있다!(둘 다 printf랑 동급이라 생각하면 된다.)
3. 변수 선언 방법
    let 변수 이름: type=초기값
        기본적으로 immutable이 default->초기화 한 값으로 계속 간다. static과 같은 맥락으로 두면 된다
4. 출력 방법(포맷 느낌)->print!("{}",변수이름), 사실 {변수이름} 이것도 가능

 */