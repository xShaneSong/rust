use std::io::Read;
use std::fs::File;

fn main() {
    chain_test();
}

fn chain_test() -> std::io::Result<()> {
    let f1 = File::open("D:\\example.txt")?;
    let f2 = File::open("D:\\example2.txt")?;
    
    let mut chained = f1.chain(f2);

    let mut buffer = String::new();
    chained.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

fn read_files(handle: &mut impl Read) -> std::io::Result<String> {
    let mut buffer = String::new();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn chain_test_with_error() {
    let mut chained_handle;

    let file1 = "file1.txt";
    let file2 = "file1.txt";
    if let Ok(f1) = File::open(file1) {
        if let Ok(f2) = File::open(file2) {
            chained_handle = f1.chain(f2);

            let content= read_files(&mut chained_handle);
            match content {
                Ok(c) => println!("{}", c),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Error opening file2");
        }
    } else {
        println!("Error opening file1");
    }
}
