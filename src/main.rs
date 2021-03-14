fn fizzbuzz(n: Option<i32>) {
    match n {
        Some(x) if x % 3 == 0 => println!("fizz"),
        Some(x) if x % 5 == 0 => println!("buzz"),
        Some(x) if x % 3 == 0 && x % 5 == 0 => println!("fizzbuzz"),
        Some(x) => println!("{}", x),
        None => panic!("hoge")
    }
}

fn main() {
    for x in (1..101) {
        let n = Some(x);
        fizzbuzz(n)
    }
}

trait Human {
    fn say(&self);
}

struct Yamada;

impl Human for Yamada {
    fn say(&self) {
        println!("yeah!")
    }
}