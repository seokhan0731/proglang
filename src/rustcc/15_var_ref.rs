/*  여기서 중점을 둬야 될건 소유권의 이동
    스택과 힙의 소유권에 대해 알아야하는데
    스택에 저장되는 값은, 값을 넘길 때 소유권의 복사본을 넘겨줌 ->소유권 issue 발생x
    but 힙에 저장되는 값은, 값을 넘길 때 소유권 자체를 넘겨줌 -> 소유권 issue 발생
            cuz 소유권은 only 1
        마찬가지로 힙의 owner인 스택값이 유효하지 않게 된다면, 도미노처럼 힙의 메모리도 해제된다

    함수의 매개변수로 값을 넘겨줄때도 소유권의 이동이 이뤄진다. 이 소유권을 rtn 않는다면, 소유권을 갖고
    있는 놈이 없어지기 때문에(cuz 코드블럭) 자연스럽게 힙에 할당된 메모리도 해제된다.


    함수내에서 힙에 할당한 후, 값을 레퍼런스로 rtn하는 행위 (컴파일 x)
    cuz 소유권은 함수 내에 있는데, 함수가 종료되는 순간, 힙에 할당된 놈이 메모리에서 해제되므로,
    레퍼런스를 리턴해도 원본이 없는 상태
 */


fn main ()
{

    /*  String은 대표적으로 힙에 저장되는 값
        country는 Austria의 시작 주소, 바이트 길이, 용량의 정보를 스택에 저장하고 있따
        처음에 one한테 소유권을 넘겨준다(country->one)
        이에 따라 two=country는 유효하지 않는다
     */
	let country = String::from("Austria") ; //소유권 country
	let one = country ; //소유권 one
	let two = country ; //소유권 one

	println!("{}", one) ;
	//println!("{}", two) ; //one이 소유권을 갖고 있으니까 컴파일 안된다

	/*
	print_country(country) ;    //소유권 one(비문)
	print_country(country) ;    //만약 소유권이 이전에 country한테 있었다면, 이전 문장에서 매개변수
	                               country에 소유권을 넘겨줘서 비문에 해당한다
	*/

	/*  //비문 cuz return_str에서 레퍼런스를 리턴하고자 하는데, 함수 종료시, 이 원본은 죽음
	let ref_three = return_str() ;
	println!("{}", ref_three) ;
	*/

	/*  이건 가능 s는 mut으로 선언, 얘 자체의 값이 바뀔 수도 있고, String의 객체 내용도 수정 가능
	        매개변수를 레퍼런스로 넘겼으니, 본질적인 소유권을 넘기지는 않음, 소유권을 빌려줌
	let mut s = String::from("Title") ;
	add_tag(&mut s) ;
	println!("{}", s) ;
	*/
}

/*
fn print_country (country: String) {
	println!("{}", country) ;
}
*/

/*
fn return_str () -> &str {
	let country = String::from("German") ;
	let country_ref = &country ;
	country_ref 
}
*/

/*
fn add_tag (name: &mut String) {
	name.push_str("-tag") ;
}
*/
