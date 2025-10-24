/*  그냥 코드를 읽을 수 있으면 된다. 자세한 사항은 12_enum.rs에 정리되어 있음

 */


/*
Solution은 enum인데 얘가 가질 수 있는 값은 튜플 두개와 NoSolution
 */
enum Solution {
    Double(f64, f64),
    Single(f64),
    NoSolution,
}

//매개 변수를 통해 열거형의 값을 정해준다
fn quad (a : f64, b : f64, c : f64) -> Solution 
{
    if a == 0.0 {
        Solution::Single(-1.0 * c / b)
    }
    else if b*b - 4.0 * a * c < 0.0 {
        Solution::NoSolution 
    }
    else if b * b == 4.0 * a * c {
         Solution::Single(-1.0 * b / (2.0 * a)) 
    }
    else {
        Solution::Double((-1.0 * b + (b*b - 4.0 * a * c).sqrt()) / (2.0 * a), 
                         (-1.0 * b - (b*b - 4.0 * a * c).sqrt()) / (2.0 * a))
    }
}

fn main () 
{
    let s : Solution ;
    s = quad(1.0, 2.0, 1.0) ;

    match s {
        Solution::NoSolution => println!("Unsolvable"),
        Solution::Single(x) => println!("{}", x),
        Solution::Double(x, y) => println!("{}, {}", x, y)
    }
}
