use std::sync::{Mutex, Arc};
// use std::thread::{spawn};
use rayon::prelude::*;

fn main() {
    println!("Hello, world! {}, {}", 1, "test");

    let max = find_max(vec![1, 2, 3, 4, 5, 10]);
    println!("max: {}", max);

    let my_int = Arc::new(Mutex::new(0i32));

    // Create an array to store thread handles
    // let mut handles = Vec::new();

    // Use rayon's parallel iterator to parallelize the loop
    (0..10_000_000).into_par_iter().for_each(|_| {
        modify_value(my_int.clone());
    });

    // Wait for all threads to finish
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // Access the modified string after the threads have finished
    println!("new str: {}",  my_int.lock().unwrap());
}

fn find_max(nums: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    for n in nums {
        if n > max {
            max = n;
        }
    }
    max
}

fn modify_value(inv: Arc<Mutex<i32>>) {
    let mut locked = inv.lock().unwrap();
    *locked += 1;
}
