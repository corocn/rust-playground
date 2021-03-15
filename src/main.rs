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

fn main() {
    // let result: Result<i32, String> = Ok(200);
    let result: Result<i32, String> = Err(String::from("Hi"));

    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("err: {}", err)
    // }

    let result= error_handling(result);

    // let a = result.expect("hoge");
    // println!("{}", a);

    if let Err(err) = result {
        println!("{}", err);
    }
}
