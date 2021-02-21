pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn papapanic() {
    panic!("papapa")
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

#[test]
#[should_panic(expected="papapa")]
fn check_panic() {
    papapanic();
}

#[test]
fn it_works() -> Result<(), String> {
    if 2 + 3 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

// pub fn fizzbuzz(n: i32) -> String {
//     if n % 3 == 0 && n % 5 == 0 {
//         return String::from("fizzbuzz")
//     } else if n % 3 == 0 {
//         return String::from("fizz");
//     } else if n % 5 == 0 {
//         return String::from("buzz");
//     }
//
//     return n.to_string();
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test1() {
//         assert_eq!("1", fizzbuzz(1));
//         assert_eq!("2", fizzbuzz(2));
//         assert_eq!("fizz", fizzbuzz(3));
//         assert_eq!("4", fizzbuzz(4));
//         assert_eq!("buzz", fizzbuzz(5));
//         assert_eq!("fizzbuzz", fizzbuzz(15));
//     }
// }