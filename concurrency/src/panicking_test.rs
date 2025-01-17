use std::fs;
use std::thread;

pub struct Filenames {
    pub source: String,
    pub destination: String,
}

impl Drop for Filenames {
    fn drop(&mut self) {
        if thread::panicking() {
            println!("dropped due to panic!");
        } else {
            println!("dropped without panic.");
        }
    }
}

fn copy_file(filenames: Filenames) -> thread::Result<()> {
    let handle = thread::spawn(move || {
        fs::copy(&filenames.source, &filenames.destination).expect("Error occurred");
    });

    handle.join()
}

pub fn do_copy_panicking() {
    let filenames = Filenames {
        source: "D:\\example3.txt".into(),
        destination: "D:\\example_copy.txt".into(),
    };

    match copy_file(filenames) {
        Ok(_) => println!("File copied successfully."),
        Err(e) => println!("Error occurred: {:?}", e),
    }
}