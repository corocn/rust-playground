fn main() {
    for n in 1..101 {
            println!("{}", n);
        if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else if n & 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        }
    }
}
