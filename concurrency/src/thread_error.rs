use std::thread;
use std::fs;

pub fn do_copy() {
    match copy_file() {
        Ok(_) => println!("File copied successfully."),
        Err(e) => println!("Error occurred: {:?}", e),
    }
}

// Copy a file from one location to another
fn copy_file() -> thread::Result<()> {
    let handle = thread::spawn(|| {
        fs::copy("D:\\example3.txt", "D:\\example_copy.txt").expect("Error occurred");
    });

    handle.join()
}

