use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

fn main() {
    let _now = SystemTime::now();
    println!("{:?}", _now);

    let now = Instant::now(); // 测量时间起点
    // sleep(Duration::new(3, 0));

    let entries = fs::read_dir("C:\\Users\\songm\\Pictures").unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            println!("{:?}", entry.file_name());
            let path = entry.path();
            let ext = match path.extension() {
                Some(ext) => ext,
                None => continue,
            };
            println!("{:?}", ext);
            println!("{:?}", entry.path());
        }
    }
    println!("{:?}", now.elapsed().as_millis());
}