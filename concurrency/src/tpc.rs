// 线程间通信
use std::thread;
use std::sync::mpsc;

pub fn tpc_channel() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move || {
        let num_sec: Vec<String> = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
        ];
        for num in num_sec {
            tx1.send(num).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let num_sec: Vec<String> = vec![
            String::from("five"),
            String::from("six"),
            String::from("seven"),
            String::from("eight"),
        ];
        for num in num_sec {
            tx2.send(num).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for rx_val in rx {
        println!("Received: {}", rx_val);
    }

}