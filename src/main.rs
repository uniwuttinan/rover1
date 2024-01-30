mod snippets;
mod same_level;

use std::sync::{Arc, Mutex};
use tokio::time::sleep;
use std::time::Duration;
use futures::future;
use same_level::same_level_mod;

#[tokio::main]
async fn main() {
    let my_int = Arc::new(Mutex::new(0));

    // variable to store start time
    let start = std::time::Instant::now();

    let mut handles = Vec::new();

    // print number of available cpu cores
    println!("Number of available cores = {}", num_cpus::get());

    for _ in 0..1_000 {
        let cloned = my_int.clone();
        let handle = tokio::task::spawn(async move {
            modify_value(cloned, false).await;
        });
        handles.push(handle);
    }

    future::join_all(handles).await;

    println!("Final value = {}", *my_int.lock().unwrap());

    // variable to store end time
    let end = std::time::Instant::now();

    // print time difference in milliseconds
    println!("Time used: {}ms", (end - start).as_millis());

    snippets::hello_mod();
    snippets::hello_snippets();

    same_level_mod::hello_same_level();
}

async fn modify_value(v: Arc<Mutex<i32>>, debug_msg: bool){
    let thread_id = std::thread::current().id();
    if debug_msg {
        println!("Thread id started = {:?}", thread_id);
    }
    {
        let mut locked = v.lock().unwrap();
        *locked += 1;
    }
    sleep(Duration::from_secs(1)).await;
    if debug_msg {
        println!("Thread id ended = {:?}", thread_id);
    }
}
