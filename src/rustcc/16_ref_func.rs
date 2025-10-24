/*  스택과 힙의 구동 방식 차이
i32는 그냥 스택에 저장되는 놈->값을 넘길 때, 값을 복사해서 넘긴다(소유권 이동 x)
String 대표적으로 힙에 저장되는 놈, 힙에 원본 존재, 스택에 얘 전담 담당관 존재
->담당관이 값을 넘길 때, 소유권을 같이 넘겨버림


 */


fn print_name_num (name : String, num: i32) {
	println!("{} {}", name ,num) ;
	println!("{:p} {:p}", &name, &num) ;
}

fn main () {
	let s = String::from("Eric") ;
	let i : i32 = 111 ;

	print_name_num(s, i) ;  //Eric의 소유권 함수 매개변수 name으로 이동, name 소멸-> Eric소멸,
                            //s는 더 이상 사용 불가, i만 사용 가능
}

/* Copy trait: https://doc.rust-lang.org/std/marker/trait.Copy.html */
