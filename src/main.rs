
fn main() {
    println!("Is the number even or odd?");
    println!("{}", is_even(6));
}

fn is_even(num: i32) -> bool {
   if num % 2 == 0 {
       return true;
   } else {
       return false;
   }
}
