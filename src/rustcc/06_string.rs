/*  &str과 String->소유권 관련 문제

    1. 스택 vs 힙
    &str과 같은 레퍼런스는 결국, 스택에 생성, String과 같은 객체는 힙에 저장된다
    ** %str은 텍스트세그먼트라고 생삭하면 된디. 얘가 mut이든 뭐든 간에, 수정 자체가 불가능하다

    &str 구조: 시작 주소와 문자열 길이(바이트) 저장, 얘는 가리키고 있는 값 자체 수정 절대 불가능(name이 여기에 해당)
    String: 힙 메모리에 할당되는 크기가 변할 수 있는 수정 가능한 소유권이 존재하는 문자열

    String 구조: 얘의 해당하는 문자열은 힙 영역에 저장, 그리고 그 문자열의 주소와 길이(바이트), 힙에서의 할당된 용량
                의 정보를 갖는 변수는 스택에 저장된다(other_name이 이에 해당)
                ->할당된 영역을 다 채운다면, 다음 글자 추가할 때, rust에서 더 큰 공간을 새로 할당 후 복사
                얘의 소유권을 잡고 있는 놈이 소멸한다면, 얘도 동시에 소멸->메모리 안정성


    size_of_val->&str을 보낼 때, 그 실제 텍스트 값의 바이트 크기가 나옴
                    ex.서태지->3+3+3(한글은 3byte)
               ("텍스트")도 마찬가지로, 그 실제 텍스트 값의 바이트 크기가 나옴



         ->String을 보낼 때, 소유권을 넘겨주지 않기 위해서 레퍼런스로 보내줘야된다.
            뭘 보내든 24가 나온다. 그냥 기억하면 됌(시작 주소(8)+바이트 길이 정보(8)+저장 용량(8)


    **&str과 String에서의 mut
    &str에서의 mut->얘가 가리키는 놈을 다른 대상으로 바꿀 수 있어요!, 하지만 얘가 rom 영역에 있는 "서태지"의 내용은
                    못 바꿔요
    String에서의 mut-> 두 가지 의미, 가리키는 놈 자체를 다른 놈 가리키도록 바꾸기(1), 가리키는 놈의 값을 수정(ex.push)
    
 */
fn main () {
    let name = "서태지" ; // "서태지"는 읽기 전용 영역에 존재, name은 스택에 얘의 시작주소와 바이트길이 정보 소유
    let other_name = String::from("Adrian Fahrenheit Țepeș") ;
    //other_name 얘는 String으로 힙에, other_name 자체는 스택에서 이 String을 관리한다(힙에서의 시작주소, 힙에 할당된 용량, 바이트 길이)

    println!("{:?} {:?}", name, other_name) ;

    //format!("원하는 형식({}포함)",변수1,변수2 등)->원하는 형식으로 포맷팅해준다
    let together = format!("1: {}, 2:{}", name, other_name) ;
    println!("{}", together) ;

    /*
    println!("{:?} {:?}", name, other_name) ;
    //dbg!(name, other_name) ;
    */

    //서태지, 9, 13    (그 자체의 길이를 뽑아줌) size_of_val("")얘는 저기에 있는 문자열 바이트 길이 자체
    println!("{} {} {}", name, std::mem::size_of_val(name), std::mem::size_of_val("서  태  지")) ;

    //24, 25->String을 레퍼런스로 size_of_val로 보내면 24나옴(뭘하든간에, 시작주소, 바이트길이, 저장용량의 총 바이트수는 24)
    println!("{} {}", std::mem::size_of_val(other_name), std::mem::size_of_val("Adrian Fahrenheit Țepeș")) ;


    //->String의 실제 byte를 알고 싶으면 .len을 써라
    println!("{} {}", other_name.len(), std::mem::size_of_val("Adrian Fahrenheit Țepeș")) ;


}
