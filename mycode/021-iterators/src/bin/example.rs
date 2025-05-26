use std::collections::HashMap;

fn main() {
    println!("=== Basic Iteration Methods ===");
    
    // Example 1: Traditional loop vs for loop
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Manual indexing approach (commented out for comparison)
    /*
    let mut current_idx = 0;
    let final_idx = numbers.len() - 1;
    
    loop {
        if current_idx > final_idx {
            break;
        }
        println!("Current number: {}", numbers[current_idx]);
        current_idx += 1;
    }
    */
    
    // While loop approach
    let mut current_idx = 0;
    let final_idx = numbers.len();
    while current_idx < final_idx {
        println!("Number at index {}: {}", current_idx, numbers[current_idx]);
        current_idx += 1;
    }
    
    // For loop approach (most idiomatic)
    println!("Using for loop with references:");
    for number in &numbers {
        println!("{}", number);
    }
    
    println!("\n=== Creating Iterators ===");
    
    // Example 2: Creating iterators from different types
    let my_vector_int = vec![1, 2, 3, 4, 5];
    let my_iterator_int = &my_vector_int.into_iter();
    println!("Integer Iterator: {:?}", my_iterator_int);

    let my_vector_bool = vec![true, false, true];
    let my_iterator_bool = &my_vector_bool.into_iter();
    println!("Boolean Iterator: {:?}", my_iterator_bool);

    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator_hashmap = &my_hashmap.into_iter();
    println!("HashMap Iterator: {:?}", my_iterator_hashmap);
    
    println!("\n=== Manual Iterator Consumption ===");
    
    // Example 3: Using next() method manually
    let my_vector = vec![1, 2, 3, 4, 5];
    let mut my_iterator = my_vector.into_iter();
    println!("Initial iterator: {:?}", my_iterator);
    println!("First next(): {:?}", my_iterator.next());
    println!("Second next(): {:?}", my_iterator.next());
    println!("Third next(): {:?}", my_iterator.next());
    println!("Fourth next(): {:?}", my_iterator.next());
    println!("Fifth next(): {:?}", my_iterator.next());
    println!("Sixth next() (exhausted): {:?}", my_iterator.next());
    println!("Seventh next() (still exhausted): {:?}", my_iterator.next());
    println!("Iterator after exhaustion: {:?}", my_iterator);
    
    println!("\n=== Iterator Ownership with into_iter() ===");
    
    // Example 4: into_iter() takes ownership
    let my_vector = vec![1, 2, 3, 4, 5];
    let my_iterator = my_vector.into_iter();
    // For loop takes ownership and makes iterator mutable internally
    for number in my_iterator {
        println!("{}", number);
    }
    // println!("{:?}", my_vector); // This would error - vector was moved
    // println!("{:?}", my_iterator); // This would error - iterator was exhausted
    
    println!("\n=== Iterator Borrowing with iter() ===");
    
    // Example 5: iter() borrows immutably
    let my_vector = vec![1, 2, 3, 4, 5];
    let my_iterator = my_vector.iter();

    for number in my_iterator {
        println!("{}", number);
    }

    println!("Vector still available: {:?}", my_vector);
    // println!("{:?}", my_iterator); // Iterator is exhausted but vector is fine

    // Alternative: iterate directly over reference
    println!("Direct iteration over reference:");
    for number in &my_vector {
        println!("{}", number);
    }

    println!("Vector still available: {:?}", my_vector);

    // Taking ownership with for loop
    println!("Taking ownership in for loop:");
    for number in my_vector {
        println!("{}", number);
    }

    // println!("{:?}", my_vector); // This would error - vector was moved
    
    println!("\n=== Mutable Iteration with iter_mut() ===");
    
    // Example 6: iter_mut() for modifying elements
    let mut flavors = [
        String::from("Vanilla"),
        String::from("Chocolate"),
        String::from("Strawberry"),
    ];
    println!("Original flavors: {:?}", flavors);
    
    let flavors_iter = flavors.iter_mut();
    for flavor in flavors_iter {
        flavor.push_str(" Ice Cream");
    }
    println!("After adding ' Ice Cream': {:?}", flavors);
    
    // Direct mutable iteration
    for flavor in &mut flavors {
        flavor.push_str(" is delicious!");
    }
    println!("After adding ' is delicious!': {:?}", flavors);

    // Mutable iteration with primitive types
    let mut school_grades = [88, 90, 73, 99];
    println!("Original grades: {:?}", school_grades);
    for grade in &mut school_grades {
        *grade -= 5; // Dereference to modify the value
    }
    println!("Grades after 5-point deduction: {:?}", school_grades);
    
    println!("\n=== Summary ===");
    println!("iter()      - Borrows immutably, returns &T");
    println!("iter_mut()  - Borrows mutably, returns &mut T");
    println!("into_iter() - Takes ownership, returns T");
    println!("for loop    - Can use any of the above automatically");
}