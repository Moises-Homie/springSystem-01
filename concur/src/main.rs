fn assignment_1() {
    use std::thread;

    let mut handles = vec![];

    for id in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting", id);
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("Thread {} finished", id);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads completed.");
}

fn assignment_2() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = counter.clone();

        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

fn main() {
    assignment_1();
    assignment_2();
}
