#![warn(clippy::pedantic)]

use rand::prelude::*;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();

    let mut big_array: [i32; 10_000] = [0; 10_000];
    for item in &mut big_array {
        *item = rng.gen();
    }

    // println!("{:?}", big_array);
    let mut cloned = big_array;

    let start = Instant::now();
    insertion_sort(&mut big_array);
    let duration = start.elapsed();

    println!("Sorting time for 10,000 integers: {:?}", duration);
    // println!("{:?}", big_array);

    cloned.sort_unstable();
    assert_eq!(big_array, cloned);
    println!("Sorted correctly!");
}

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
