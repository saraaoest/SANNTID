use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Main program started! PID: {}", std::process::id());

    sleep(Duration::from_secs(5)); // Simulate running
    println!("Main program is crashing...");

    // Spawn a new instance of itself **before** crashing
    Command::new("./your_program")  // Replace with your compiled binary
        .spawn()
        .expect("Failed to start a new instance");

    panic!("Main program crashed!");
}