unsafe fn print_dangerously() {
    println!("hoge");
}

fn main() {
    unsafe {
        print_dangerously();
    }
}
