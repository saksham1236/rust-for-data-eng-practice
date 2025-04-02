use std::collections::HashMap;

fn counter(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut hmap = HashMap::new();

    for num in numbers {
        let mut count = hmap.entry(num).or_insert(0);
        *count += 1;
    }

    let mut result = Vec::new();

    for (num, count) in hmap {
        result.push((num, count))
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3, 1, 2, 1, 4, 5, 6, 7, 8, 9];
    let result = counter(numbers);
    //Print the result
    println!("Number Frequency:{:?}", result);
}
