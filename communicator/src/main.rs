use std::thread;

fn main() {
    let num_threads = 11; // number of CPU cores

    for i in 0..num_threads {
        thread::spawn(move || {
            println!("Thread {i} starting...");
            loop {
                let _ = 2 * 2;
            }
        });
    }
    
    loop {
        thread::park();
    }
}
