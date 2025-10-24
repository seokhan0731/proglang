/*  조건문 관련
rust에서의 if는 문장이 아닌 표현식->변수 정의할 때도,들어갈 수 있음
ex. let gr=if b<a{a}else{b} 이런식으로

loop->무한 루프를 따로 지정해둠
    break;으로 탈출하는건 마찬가지

루프에 이름 짓기 '표현1: loop{
                    loop{
                        break '표현1;
                    }
                   }
            ->내부 루프에서도 loop의 이름을 지어 한번에 탈출 가능

 */

fn main ()
{
	let a = 5 ;
	let b = 9 ;

	let gr = if b < a {
			a 
		}
		else {
			b
		} ;

	println!("gr: {}", gr) ;

	/*
	let mut i = 0 ;
	loop {
		if i == 10 {
			break ;
		}
		i = i + 1 ;
		print!("{} ", i) 
	}
	*/

	let mut i = 0 ;
	let mut j ; 
	'l1: loop {
		j = 0 ;
		loop {
			if i == 4 && j == 2 {
				break 'l1 ;
			}
			if j == 100 {
				break ;
			}
			j = j + 1 ;
		}
		i = i + 1 ;
	}
	println!("i: {}, j: {}", i, j) ;
}
