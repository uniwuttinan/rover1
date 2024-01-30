
use std::sync::{Arc, Mutex};
use tokio::time::sleep;
use std::time::Duration;
use futures::future;

#[tokio::main]
async fn main() {
    let my_int = Arc::new(Mutex::new(0));

    // variable to store start time
    let start = std::time::Instant::now();

    let mut handles = Vec::new();

    for _ in 0..10 {
        let cloned = my_int.clone();
        let handle = tokio::task::spawn(async move {
            modify_value(cloned).await;
        });
        handles.push(handle);
    }

    future::join_all(handles).await;

    println!("Final value = {}", *my_int.lock().unwrap());

    // variable to store end time
    let end = std::time::Instant::now();

    // print time difference in milliseconds
    println!("Time used: {}ms", (end - start).as_millis());

}

async fn modify_value(inv: Arc<Mutex<i32>>) {
    {
        let mut locked = inv.lock().unwrap();
        *locked += 1;
    }
    sleep(Duration::from_secs(1)).await;
}
