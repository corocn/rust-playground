use std::rc::Rc;

// fn insertion_sort() {
//     let mut a = [5, 2, 4, 6, 1, 3];
//
//     let len = a.len();
//     for j in 1..len {
//         let key = a[j];
//         let mut i = j;
//         while i > 0 && a[i] < a[i - 1] {
//             a.swap(i , i - 1);
//             i = i - 1;
//         }
//     }
//     dbg!(a);
// }

fn main() {
    let a = Rc::new("hello".to_string());
    println!("Reference Count of a: {}", Rc::strong_count(&a));

    {
        let b = Rc::clone(&a);
        println!("Reference Count of a: {}", Rc::strong_count(&a));
        println!("Reference Count of b: {}", Rc::strong_count(&b));
    }

    println!("Reference Count of a: {}", Rc::strong_count(&a));
}
