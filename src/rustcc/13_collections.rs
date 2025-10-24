/*  배열과 벡터
배열=> let 변수 이름=[값1,값2]  ->값, 추가 삭제 불가, 스택에 저장
슬라이스 방법
let 변수 이름=&배열 변수 이름[범위표현]
부분 문자열도 그렇게 &이름 이런식으로 잘라오네

벡터->선언법 2가지
1. let 변수 이름: Vec<type>=Vec::new()
2. let mut vec1=vec![값1,값2,값3]

사용법은 pushback대신 push

마찬가지로 vector에 mut 안 붙이면 push,pop불가
 */
fn main () {
	let arr1 = [1, 2, 3, 4, 5] ;
	let slice1 = &arr1[2..4] ;

	println!("slice: {:?}", slice1) ;

	/*
	let mut vec1 : Vec<&str> = Vec::new() ;
	let mut vec2 = vec![11, 12, 13] ;

	vec1.push("A") ;
	vec1.push("B") ;
	vec1.push("C") ;

	vec2.push(14) ;
	vec2.push(1500) ;

	for e in vec1 {
		print!("{} ", e) ;
	}
	println!("") ;
	*/
}
