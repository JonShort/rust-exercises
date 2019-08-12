extern crate rand;

use std::collections::HashMap;
use rand::Rng;

use std::time::{Instant};

fn main() {
    let now = Instant::now();
    let mut rng = rand::thread_rng();

    // populate a Vec with random numbers
    let amount_of_numbers: i32 = rng.gen_range(25, 50);
    let numbers: Vec<i32> = (0..amount_of_numbers).map(|_| {
        rng.gen_range(0, 10)
    }).collect();

    println!("amount_of_numbers: {:#?}", amount_of_numbers);
    println!("numbers: {:#?}", numbers);

    // calculate the mean (sum / count)
    let sum: i32 = numbers.iter().sum();
    let mean: f32 = sum as f32 / amount_of_numbers as f32;

    // calculate the median ('middle' of sorted list)
    let median = calculate_median(numbers.to_vec());

    // calculate the mode (num which occurs the most)
    let mode = calculate_mode(numbers.to_vec());

    println!("mean: {:#?}", mean);
    println!("median: {:#?}", median);
    println!("mode: {:#?}", mode);
    println!("program finished: {}s", now.elapsed().as_secs());
}

fn calculate_median(mut numbers: Vec<i32>) -> f32 {
    numbers.sort();
    let vec_length = numbers.iter().count();
    let has_even_length = vec_length % 2 == 0;
    let center_index = vec_length / 2;

    if has_even_length {
        // length / 2 will resolve to x.5, which is rounded-up as i32, so we -1 for the left side
        let left_side = &numbers[center_index - 1];
        let right_side = &numbers[center_index];

        (*left_side as  f32 + *right_side as f32) / 2 as f32
    } else {
        numbers[center_index] as f32
    }
}

fn calculate_mode(numbers: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    numbers.iter().for_each(|num| {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    });

    let most_frequent_number = map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value)
        .unwrap();

    most_frequent_number
}
