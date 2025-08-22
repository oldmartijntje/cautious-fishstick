use std::process::Command;

fn main() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run");

    let hello_1 = cmd.spawn().expect("failed to spawn").wait_with_output().expect("failed output");
    let hello_2 = cmd.spawn().expect("failed to spawn").wait_with_output().expect("failed output");

    println!("First run: {}", String::from_utf8_lossy(&hello_1.stdout));
    println!("Second run: {}", String::from_utf8_lossy(&hello_2.stdout));
}
