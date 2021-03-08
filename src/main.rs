fn main() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let mut i = a.iter().map(|x| x * x);

    let next = i.next();

    match next {
        Some(n) => println!("{}", n),
        None => println!("aaa")
    }
}