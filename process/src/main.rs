use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::panic;

fn main() {
    // create_new_process();
    // create_new_process_with_args();
    // abort_process();

    // check_process_status();

    abort_current_process_with_panic();
    // process::abort(); // abort current process
    // process::exit(64); // exit current process with code 64
}

// This function creates a new process
fn create_new_process() {
    Command::new("notepad.exe")
        .spawn()
        .expect("notepad command failed to start");
}

// This function creates a new process with arguments
fn create_new_process_with_arg() {
    Command::new("notepad.exe")
        .arg("D:\\example.txt")
        .spawn()
        .expect("notepad command failed to start");
}

// This function creates a new process with multi arguments
fn create_new_process_with_args() {
    Command::new("notepad.exe")
        .args(&["D:\\example2.txt"])
        .spawn()
        .expect("notepad command failed to start");
}

// This function aborts a process
fn abort_process() {
    Command::new("notepad.exe")
        .spawn()
        .expect("notepad command failed to start")
        .kill()
        .expect("notepad command failed to abort");
}

// check process's status
fn check_process_status() {
    let status = Command::new("notepad.exe")
        .status()
        .expect("notepad command failed to start");

    if status.success() {
        println!("notepad.exe exited successfully");
    } else {
        println!("notepad.exe exited with: {}", status);
    }
}

// This function demonstrates how to communicate between parent and child processes
fn process_communication_between_parent_and_child() {
    let process = match Command::new("program").stdout(Stdio::piped()).spawn() {
        Err(why) => panic!("couldn't spawn program: {}", why),
        Ok(process) => process,
    };

    let mut output = String::new();
    match process.stdout.unwrap().read_to_string(&mut output) {
        Err(why) => panic!("couldn't read program stdout: {}", why),
        Ok(_) => print!("program stdout:\n{}", output),
    }
}

// This function demonstrates how to send data from parent to child process
fn send_data_from_parent_to_child() {
    // Create a new process
    // The `stdin` field is set to `Stdio::piped()`, which means that the parent process can write to the child process's standard input
    let process = match Command::new("rev")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn rev: {}", why),
        Ok(process) => process,
    };
    match process.stdin.unwrap().write_all(b"hello\n") {
        Err(why) => panic!("couldn't write to rev stdin: {}", why),
        Ok(_) => println!("sent to rev"),
    }
    let mut child_output = String::new();
    match process.stdout.unwrap().read_to_string(&mut child_output) {
        Err(why) => panic!("couldn't read rev stdout: {}", why),
        Ok(_) => print!("rev stdout:\n{}", child_output),
    }
}

// This function demonstrates how to send data from child to parent process
fn set_env_variable_for_child_process() {
    Command::new("program")
        .env("FOO", "bar")
        .spawn()
        .expect("program command failed to start");

    Command::new("program")
        .envs([("FOO", "bar"), ("BAZ", "quux")].iter().cloned())
        .spawn()
        .expect("program command failed to start");

    Command::new("program")
        .env_clear()
        .spawn()
        .expect("program command failed to start");
}

// This function demonstrates how to abort the current process
fn abort_current_process_with_panic() {
    let _child_process = match Command::new("Invalid-Program")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped()).spawn() {
        Err(why) => panic!("couldn't spawn notepad: {}", why),
        Ok(process) => {
            println!("spawned notepad");
            process
        }
    };
}

fn test_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        println!("This is an example of custom pianic hook, panic occurred: {:?}", panic_info);
    }));
    let _child_process = match Command::new("Invalid-Program")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped()).spawn() {
        Err(why) => panic!("couldn't spawn notepad: {}", why),
        Ok(process) => {
            println!("spawned notepad");
            process
        }
    };
}