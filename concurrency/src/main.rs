use std::thread;
mod thread_error;
// use panicking_test::do_copy_panicking;
use thread_error::do_copy;
mod panicking_test;

mod tpc;

fn main() {
    // create_thread_spawn();
    // create_thread_use_builder();

    // do_copy();

    // panicking_test::do_copy_panicking();

    tpc::tpc_channel();
}

// Create a thread using thread::spawn
fn create_thread_spawn() {
    let handle = thread::spawn(|| {
        println!("Hello from thread id {:?}.", thread::current().id());
    });

    handle.join().unwrap();
}

// Create a thread using thread::Builder
fn create_thread_use_builder() {
    let handle = thread::Builder::new()
        .name("create_thread_use_builder".into())
        .spawn(|| {
            println!("Hello from thread id {:?}.", thread::current().id());
        })
        .unwrap();

    handle.join().unwrap();
}