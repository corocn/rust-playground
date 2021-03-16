fn fizzbuzz(n: Option<i32>) {
    match n {
        Some(x) if x % 3 == 0 => println!("fizz"),
        Some(x) if x % 5 == 0 => println!("buzz"),
        Some(x) if x % 3 == 0 && x % 5 == 0 => println!("fizzbuzz"),
        Some(x) => println!("{}", x),
        None => panic!("hoge")
    }
}

type MyResult = Result<i32, String>;

fn error_handling(result: MyResult) -> MyResult {
    let code = result?;
    println!("code: {}", code);
    Ok(200)
}

struct Dog;
struct Cat;

trait Tweet {
    fn tweet(&self);
}

impl Tweet for Dog {
    fn tweet(&self) {
        println!("wanwan!");
    }
}

impl Tweet for Cat {
    fn tweet(&self) {
        println!("nyanya!");
    }
}

fn main() {
    let dog = Dog {};
    let cat = Cat {};

    dog.tweet();
    cat.tweet();

    let animal_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dog), Box::new(cat)];

    for animal in animal_vec {
        animal.tweet();
    }

    // let s = concat!("a", "b", 3);
    // let s = format!("{}-{:?}", s, ("D", 5));
    // println!("{}", s);
    //
    // println!("defined in file: {}", file!());
    // println!("defined on line: {}", line!());
    // println!("is test: {}", cfg!(unix));
    // println!("CARGO_HOME: {}", env!("CARGO_HOME"));


    // // let result: Result<i32, String> = Ok(200);
    // let result: Result<i32, String> = Err(String::from("Hi"));
    //
    // // match result {
    // //     Ok(code) => println!("code: {}", code),
    // //     Err(err) => println!("err: {}", err)
    // // }
    //
    // let result= error_handling(result);
    //
    // // let a = result.expect("hoge");
    // // println!("{}", a);
    //
    // if let Err(err) = result {
    //     println!("{}", err);
    // }
}
