/*12_enum.rs
    열거형과 match
    rust에서 열거형은 c의 union과 같은 맥락
    필드값들이 해당 enum의 값이 될 수 있는 가능성들은 가진 놈들임
    enum도 하나의 자료형으로 match와 함께 사용된다

    선언: enum 이름{변수 1, 변수2}
    사용: 이름::변수1=> 변수 1일 때 실행될 코드

    match->switch문이라 생각하면 된다
    match 변수 이름{
        변수가 가질 수 있는 값의 범위 =>실행코드(ex. enum이름::값)
        (범위를 표현할때는 x..y 이런식으로
        ->변수가 이런 값을 가질 때, enum의 값은 이게 됩니다
        _=>default라 생각하면 된다
    }
    **match를 enum과 함께 쓴다면 enum의 모든 케이스를 다 조건으로 넣어줘야한다
    그리고 일반적인 조건문에 match를 넣을 수는 x

    enum도 결국 힙에 저장되네->소유권 이슈
    =>값을 넘겨줄때 레퍼런스로 넘겨주는게 좋겠네
 */

enum ThingsInTheSky {
    Sun,
    Stars,
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    /*
    if 6 <= time && time <= 18 {
        ThingsInTheSky::Sun
    }
    else {
        ThingsInTheSky::Stars
    }
    */
    match time {
        6..=18 => ThingsInTheSky::Sun, 
        _ => ThingsInTheSky::Stars,
    }
    
}

fn check_skystate (state: &ThingsInTheSky) -> () {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!")
    }
}


fn main() {
    let time = 8; 
    let skystate : ThingsInTheSky = create_skystate(time); 
    check_skystate(&skystate);
}
