//cargo init to make cargo.toml file 
//run by cargo run main.rs

// or use: cargo new my_project

//run a single rustfile: 
//rustc main.rs
//main.exe on windows, ./main on linux  




use std::process::Command;
use std::env;

//PROGRAM 4
fn main() {
    println!("Starting launcher...");

    // Full path to the compiled binary (use your_program as the name of your binary)
    let path_to_program = "./target/debug/your_program"; // Ensure this path is correct

    let script = format!(
        r#"
    while true; do
        echo "Launching main program..."
        {}
        echo "Main program crashed! Restarting..."
        sleep 2
    done
    "#,
        path_to_program
    );

    Command::new("x-terminal-emulator")  // Opens a new terminal
        .arg("-e")
        .arg(format!("bash -c '{}'", script)) // Runs script inside a new terminal
        .spawn()
        .expect("Failed to launch terminal");

    println!("Main program is running in a new terminal!");
}

//PROGRAM 3
// use std::process::{Command, Stdio}
// use std::io::prelude::*;

// fn main() {
//     let mut cmd = if cfg!(target_family = "windows") { 
//         let mut cmd = Command::new("powershell");
//         cmd.arg("-Command")
//             .arg("$input | Measure-Object -Line -Word -Character")
//             .arg("Start-Sleep -Seconds 60")
//             .spawn()
//             .expect("failed to execute process");
//         cmd
//     } else {
//         let _child = Command::new("sh")
//             .arg("-c")
//             .arg("sleep 60")  // Child process runs for 60 seconds
//             .spawn()  // Starts the process but does NOT wait for it
//             .expect("Failed to start process");

//         panic!("Main program crashed!"); // Simulate a crash
//     };
// }

//PROGRAM 2
// use std::io::prelude::*;
// use std::process::{Command, Stdio};

// //make a variable str 
// //TODO: understand PANGRAM
// static PANGRAM: &'static str = "the quick brown fox jumps over the lazy dog\n";

// fn main() {
//     // Spawn the `wc` command: word count
//     // if running on windows ... , else
//     let mut cmd = if cfg!(target_family = "windows") { 
//         let mut cmd = Command::new("powershell");
//         cmd.arg("-Command").arg("$input | Measure-Object -Line -Word -Character");
//         cmd
//     } else {
//         Command::new("wc")
//     };


//     let process = match cmd
//                                 .stdin(Stdio::piped())
//                                 .stdout(Stdio::piped())
//                                 .spawn() { //starts the prossess
//         Err(why) => panic!("couldn't spawn wc: {}", why),
//         Ok(process) => process,
//     };

//     // Write a string to the `stdin` of `wc`.
//     //
//     // `stdin` has type `Option<ChildStdin>`, but since we know this instance
//     // must have one, we can directly `unwrap` it.
//     match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
//         Err(why) => panic!("couldn't write to wc stdin: {}", why),
//         Ok(_) => println!("sent pangram to wc"),
//     }

//     // Because `stdin` does not live after the above calls, it is `drop`ed,
//     // and the pipe is closed.
//     //
//     // This is very important, otherwise `wc` wouldn't start processing the
//     // input we just sent.

//     // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
//     let mut s = String::new();
//     match process.stdout.unwrap().read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read wc stdout: {}", why),
//         Ok(_) => print!("wc responded with:\n{}", s),
//     }
// }


//PROGRAM 1
// use std::process::Command;

// fn main(){
//     //making the child prosess
//     let output = Command::new("rustc")
//         .arg("--version")
//         .output().unwrap_or_else(|e| {
//             panic!("failed to execute process: {}", e)
//     });

//     if output.status.success() {
//         let s = String::from_utf8_lossy(&output.stdout);

//         print!("rustc succeeded and stdout was:\n{}", s);
//     } else {
//         let s = String::from_utf8_lossy(&output.stderr);

//         print!("rustc failed and stderr was:\n{}", s);
//     }

//     //TODO:
//     // if(fail){
//     //     pub fn exit(code: i32);
//     // }

// }