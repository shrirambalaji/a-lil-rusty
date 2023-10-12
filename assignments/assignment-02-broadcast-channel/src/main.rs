use broadcast::Broadcast;
use std::thread;

mod broadcast;

fn main() {
    let mut broadcaster: Broadcast<String> = Broadcast::new();
    let subscriber_01 = broadcaster.subscribe();
    let thread_01 = thread::spawn(move || {
        for message in subscriber_01 {
            println!("Subscriber 01: {:?}", message);
        }
    });

    let subscriber_02 = broadcaster.subscribe();
    let thread_02 = thread::spawn(move || {
        for message in subscriber_02 {
            println!("Subscriber 02: {:?}", message);
        }
    });

    let subscriber_03 = broadcaster.subscribe();
    let thread_03 = thread::spawn(move || {
        for message in subscriber_03 {
            println!("Subscriber 03: {:?}", message);
        }
    });

    let producer = thread::spawn(move || {
        broadcaster.broadcast(String::from("Hola"));
    });

    thread_01.join().unwrap();
    thread_02.join().unwrap();
    thread_03.join().unwrap();
    producer.join().unwrap()
}
