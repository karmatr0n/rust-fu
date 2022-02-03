use std::collections::HashMap;

fn mean(v: &mut Vec<i32>) -> f32 {
    // let mut sum: i32 = 0;
    // let l = v.len();
    // for &mut n in v {
    //     sum += n;
    // }
    // sum as f32 / l as f32
    let sum: i32 = v.iter().sum();
    sum as f32 / v.len() as f32
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    let mid = v.len() / 2;
    if v.len() % 2 == 0 {
        mean(&mut vec![v[mid - 1], v[mid]]) as i32
    } else {
        v[mid]
    }
}
fn mode(v: &mut Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for &mut n in v {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }
    let max_value = map.values().cloned().max().unwrap_or(0);
    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(k, _)| k)
        .collect()
}

fn main() {
    let mut v = vec![10, 5, 14, 18, 19, 17, 44, 50, 10];
    let m1 = mean(&mut v);
    println!("Mean: {}", m1);

    let m2 = median(&mut v);
    println!("Median: {}", m2);

    let mut v2 = vec![1, 2, 2, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 7, 7, 7, 8, 9, 10];
    let m3 = mode(&mut v2);

    println!("Mode: {:?}", m3);
}
