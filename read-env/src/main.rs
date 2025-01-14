use dotenv::dotenv;
use std::env;

fn main() {
    for argument in env::args() {
        println!("{}", argument);
    }


    dotenv().ok();
    for (key, value) in env::vars() {
        // println!("{}: {}", key, value);
    }
    // env::var("USERDOMAIN")
    println!("USERDOMAIN: {}", env::var("USERDOMAIN").unwrap());
}
