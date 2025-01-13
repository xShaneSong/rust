fn main() {
    println!("Hello, world!");

    let mut test_str : std::str::Chars = "Hello, world!".chars();
    while let Some(c) = test_str.next() {
        println!("{}", c);
        
    }
}
