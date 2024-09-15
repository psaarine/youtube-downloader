mod common;

use std::ops::Deref;

use crate::common::wait_job;
fn main() {
    
    let items: std::vec::Vec<String> = std::vec::Vec::new();
    let mutex_and_condvar = std::sync::Arc::new((std::sync::Mutex::new(items), std::sync::Condvar::new()));
    let consumer_mutex_and_condvar = mutex_and_condvar.clone();

    std::thread::spawn(move || {

        let (items_mutex, condvar) = mutex_and_condvar.deref();
        loop {

            let msg = get_application_message();
            {

                let mut target_container = items_mutex.lock().unwrap();
                println!("Adding {} to the items", msg);
                target_container.push(msg);
                condvar.notify_all();
            }

        }

    });

    let consumer = std::thread::spawn(move || {

        let (items_mutex, condvar) = consumer_mutex_and_condvar.deref();
        loop {

            let work_items = wait_job(items_mutex, condvar);
            consume_messages(work_items);
        }
    });

    let _ = consumer.join();
}

fn get_application_message() -> String {

    std::thread::sleep(std::time::Duration::from_millis(5000));
    return "Message".to_owned();
}

fn consume_messages<T> (messages: std::vec::Vec<T>) where T: std::fmt::Display {

    for msg in messages.into_iter() {

        println!("Got {}", msg);
    }
}
