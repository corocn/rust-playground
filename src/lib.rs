pub fn fizzbuzz(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        return String::from("fizzbuzz")
    } else if n % 3 == 0 {
        return String::from("fizz");
    } else if n % 5 == 0 {
        return String::from("buzz");
    }

    return n.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("1", fizzbuzz(1));
        assert_eq!("2", fizzbuzz(2));
        assert_eq!("fizz", fizzbuzz(3));
        assert_eq!("4", fizzbuzz(4));
        assert_eq!("buzz", fizzbuzz(5));
        assert_eq!("fizzbuzz", fizzbuzz(15));
    }
}