fn main() {
    let x = 5;

    let condition = true;
    let y = if condition { 5 } else { 6 };

    println!("{}", add(x, y));

    println!("----");

    let num = [10, 20, 30, 40, 50];

    for n in (10..20).rev() {
        println!("{}", n)
    }
}

fn add(x: i32, y: i32) -> i32 {
    println!("{} + {}", x, y);
    x + y
}
