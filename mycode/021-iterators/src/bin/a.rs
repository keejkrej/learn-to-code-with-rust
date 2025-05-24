fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut current_idx = 0;
    let final_idx = numbers.len() - 1;

    // loop {
    //     if current_idx > final_idx {
    //         break;
    //     }

    //     println!("Current number: {}", numbers[current_idx]);

    //     current_idx += 1;
    // }

    while current_idx < final_idx {
        println!("Found 3 at index {}", current_idx);
        current_idx += 1;
    }

    for number in &numbers {
        println!("{}", number);
    }
}
