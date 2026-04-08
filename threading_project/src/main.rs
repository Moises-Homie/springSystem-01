// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;

// // Message to be sent to the workers
// enum Message {
//     NewJob(Job),
//     Terminate,
// }

// // Job type is a boxed closure that can be sent across threads
// type Job = Box<dyn FnOnce() + Send + 'static>;

// // ThreadPool struct
// struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }

// impl ThreadPool {
//     fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);

//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc::new(Mutex::new(receiver));

//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         ThreadPool { workers, sender }
//     }

//     fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job = Box::new(f);
//         self.sender.send(Message::NewJob(job)).unwrap();
//     }
// }

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         println!("Sending terminate messages to workers...");

//         for _ in &self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         println!("Shutting down workers...");

//         for worker in &mut self.workers {
//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

// // Worker struct
// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
//         let thread = thread::spawn(move || loop {
//             let message = receiver.lock().unwrap().recv().unwrap();

//             match message {
//                 Message::NewJob(job) => {
//                     println!("Worker {} got a job; executing...", id);
//                     job();
//                 }
//                 Message::Terminate => {
//                     println!("Worker {} terminating.", id);
//                     break;
//                 }
//             }
//         });

//         Worker {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

// fn main() {
//     let pool = ThreadPool::new(4);

//     for i in 1..=10 {
//         pool.execute(move || {
//             println!("Processing task {}", i);
//             thread::sleep(std::time::Duration::from_millis(500));
//             println!("Completed task {}", i);
//         });
//     }

//     println!("Main thread waiting for tasks to complete...");
// }


//Assignment 4
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;

    // Create a channel for sending numbers
    let (tx, rx) = mpsc::channel::<i32>();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = Vec::new();

    // Create 2 producer threads
    let num_producers = 2;
    let items_per_producer = ITEM_COUNT / num_producers;

    for id in 1..=num_producers {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            producer(id, tx_clone, items_per_producer);
        }));
    }

    // Create 3 consumer threads
    let num_consumers = 3;

    for id in 1..=num_consumers {
        let rx_clone = Arc::clone(&rx);
        handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Send termination signal once for each consumer
    for _ in 0..num_consumers {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let value = rng.gen_range(1..100);
        println!("Producer {} sending {}", id, value);
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(200));
    }

    println!("Producer {} finished.", id);
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
        }

        println!("Consumer {} received {}", id, value);
        thread::sleep(Duration::from_millis(300));
    }

    println!("Consumer {} exiting.", id);
}
