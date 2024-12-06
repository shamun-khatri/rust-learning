
fn main() {
    println!("Is the number even or odd?");
    println!("{}", is_even(6));
    println!("{}", fib(10));
}

fn fib(num: i32) -> i32 {

    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    for _ in 1..num {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}

fn is_even(num: i32) -> bool {
   if num % 2 == 0 {
       return true;
   } else {
       return false;
   }
}
