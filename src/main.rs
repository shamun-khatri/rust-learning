fn main() {
    println!("Is the number even or odd?");
    println!("{}", is_even(6));
    println!("{}", fib(10));
    let string = String::from("Hello, World!");
    println!("{}", ger_string_length(&string));
    
    let vec = vec![1,2,3,4,5,6,7,8,9];

    let evens = even_filter(&vec);
    println!("elements which are even is {:?}", evens);
}

fn even_filter(arr: &Vec<i32>)-> Vec<i32> {
    
    let mut res: Vec<i32> = Vec::new();
    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            res.push(arr[i]);
        }
    }
    return res;
}

fn ger_string_length(s: &str) -> usize {
    s.len()
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
