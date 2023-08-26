use std::sync::mpsc;
use std::thread;
use std::time::Duration;


/* 
Message-passing concurrency
Multiple Producer | Single Consumer
*/ 

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    // tx1
    // move ~ moves transmission handles to thread closures (as opposed to borrow)
    thread::spawn(move || {
        let num_vec =vec![1,2,3];
        for num in num_vec{
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    // tx2
    // move ~ moves transmission handles to thread closures - taking ownership (as opposed to borrow)
    thread::spawn(move || {
        let num_vec =vec![4,5,6];
        for num in num_vec{
            tx2.send(num).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    // rx
    for recieved_val in rx{
        println!("Received {}",recieved_val);
    }

}


// ChatGPT example!
// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     // Create an mpsc channel
//     let (tx, rx) = mpsc::channel();

//     // Create a producer thread
//     let producer_thread = thread::spawn(move || {
//         for i in 1..=5 {
//             let message = format!("Message {}", i);
//             tx.send(message).unwrap(); // Send the message into the channel
//             thread::sleep(std::time::Duration::from_millis(500)); // Simulate some work
//         }
//     });

//     // Receive messages in the main thread (consumer)
//     for received_message in rx.iter() {
//         println!("Received: {}", received_message);
//     }

//     // Wait for the producer thread to finish
//     producer_thread.join().unwrap();
// }


