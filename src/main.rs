use std::io;

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] < target {
            low = mid + 1;
        } else if arr[mid] > target {
            high = mid - 1;
        } else if mid > 0 && arr[mid - 1] == target {
            high = mid - 1;
        } else {
            return Some(mid);
        }
    }
    None
}

fn main() {
    println!("Enter a sorted array of integers (comma-separated):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Enter the target number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let target: i32 = input.trim().parse().unwrap();

    if let Some(index) = first_occurrence(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} is not present in the array", target);
    }
}
