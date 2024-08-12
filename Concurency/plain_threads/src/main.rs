use std::thread;
use std::time::Duration;
fn main() {
    let thread_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread spawn print number {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..4 {
        println!("Thread main print number {i}");
        thread::sleep(Duration::from_millis(1));
    }
    //Waiting all threads finish their job using handle
    thread_handle.join().unwrap();
    //using move with closures
    let my_vec = vec![1, 2, 3];
    let thread_handle_vec = thread::spawn(|| {
        println!("Print in the vector {:?}", my_vec);
    });
}
