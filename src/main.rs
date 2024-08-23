use algo_and_rust::sorting::bubble_sort::bubble_sort;
use algo_and_rust::utils::print_array;

fn main() {
    println!("💖💖💖 Hello, Rust 💖💖💖");

    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("Before sorting:");
    print_array(&data);

    bubble_sort(&mut data);
    println!("After sorting:");
    print_array(&data);
}