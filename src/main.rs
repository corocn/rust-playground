struct Item {
    name: String
}

struct Item2 {
    name: String
}


fn main() {
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    //
    // for i in &v {
    //     println!("{}", i);
    // }

    let v = vec![
        Item { name: String::from("hoge1") },
        Item { name: String::from("hoge2") },
    ];

    for item in &v {
        println!("{}", item.name);
    }
}
