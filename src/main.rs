fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world　hello";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{}", word)
    }

    let mut c = 1;
    let mut x = &c;

    c += 1;

    println!("{:?}", c);


    // dbg!(value);
}
