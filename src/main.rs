struct Item {
    name: String
}

struct Item2 {
    name: String
}


fn main() {
    let hello = "Hello World";

    let s = &hello[0..4];

    println!("{}", s);
}
