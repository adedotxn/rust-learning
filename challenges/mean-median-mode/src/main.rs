use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 5, 6, 6, 7];
    v.push(7);
    v.push(7);

    let (mean, median, mode) = mean_median_mode(&v);

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    match mode {
        Some(value) => println!("Mode: {}", value),
        None => println!("No unique mode found"),
    }
}

fn mean_median_mode(v: &Vec<i32>) -> (f64, f64, Option<&i32>) {
    // Mean
    let sum: i32 = v.iter().sum();
    let mean = sum as f64 / v.len() as f64;
    // println!("mean: {mean}");

    // Median
    let mut vector_clone = v.clone();
    vector_clone.sort();
    let len = vector_clone.len();
    let median = if len % 2 == 0 {
        (vector_clone[len / 2 - 1] + vector_clone[len / 2]) as f64 / 2.0
    } else {
        vector_clone[len / 2] as f64
    };
    // println!("median: {median}");

    // Mode
    let mut hashmap = HashMap::new();
    for i in v {
        *hashmap.entry(i).or_insert(0) += 1;
    }

    let mut mode = None;
    let mut max_count = 0;

    for (&num, &count) in &hashmap {
        if count > max_count {
            mode = Some(num);
            max_count = count;
        }
    }

    (mean, median, mode)
}
