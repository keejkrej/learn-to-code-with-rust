use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== HashMap Creation and Basic Operations ===");
    
    // Example 1: Creating HashMap with insert method
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Burger"), 14.99);
    println!("Restaurant menu: {menu:?}");

    // Example 2: HashMap with string slices
    let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");
    println!("Country capitals: {country_capitals:?}");

    println!("\n=== HashMap Creation from Array ===");
    
    // Example 3: Creating HashMap from array data
    let data = [
        ("Bobby", 7),
        ("Grant", 4),
        ("Ben", 6)
    ];
    let mut years_at_company = HashMap::from(data);
    println!("Years at company: {years_at_company:?}");

    // Example 4: Removing items (note: intentional typo to show error handling)
    if let Some(grant_years) = years_at_company.remove("Gran") {
        println!("Gran is fired and has been at the company for {grant_years} years");
    } else {
        println!("Employee 'Gran' not found - perhaps you meant 'Grant'?");
    }

    println!("\n=== Advanced HashMap Operations ===");
    
    // Example 5: References, get method, and entry API
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    println!("Coffee pairings count: {}", coffee_pairings.len());
    println!("Coffee pairings: {coffee_pairings:?}");

    // Using get method
    if let Some(&value) = coffee_pairings.get("Espresso") {
        println!("Espresso pairs with: {}", value);
    } else {
        println!("No pairing found for Espresso");
    }

    // Updating existing values
    coffee_pairings.insert("Latte", "Pistachio Milk");
    println!("After updating Latte: {coffee_pairings:?}");

    // Using entry API - or_insert for existing key
    coffee_pairings
        .entry("Flat White")
        .or_insert("Pistachio Milk");
    println!("After entry or_insert for existing Flat White: {coffee_pairings:?}");

    // Using entry API - or_insert for new key
    coffee_pairings.entry("Espresso").or_insert("Oat Milk");
    println!("After entry or_insert for new Espresso: {coffee_pairings:?}");

    println!("\n=== HashSet Basic Operations ===");
    
    // Example 6: HashSet creation and basic operations
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("Empty concert queue: {concert_queue:?}");

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("Concert queue after inserts: {concert_queue:?}");
    println!("Concert queue length: {}", concert_queue.len());

    // HashSet prevents duplicates
    concert_queue.insert("Molly");
    println!("After inserting duplicate 'Molly': {concert_queue:?}");

    // Removing elements
    println!("Removed Megan: {}", concert_queue.remove("Megan"));
    println!("Tried to remove Franny: {}", concert_queue.remove("Franny"));
    println!("Concert queue after removals: {concert_queue:?}");

    // Checking for existence
    println!("Contains Molly: {}", concert_queue.contains("Molly"));
    println!("Contains Fred: {}", concert_queue.contains("Fred"));

    // Using get method
    println!("Get Molly: {:?}", concert_queue.get("Molly"));
    println!("Get Joe: {:?}", concert_queue.get("Joe"));

    println!("\n=== HashSet Set Operations ===");
    
    // Example 7: Set operations between HashSets
    let mut concert_queue_new: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue_new.insert("Boris");
    concert_queue_new.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    // Union operations
    println!("Concert ∪ Movie: {:?}", concert_queue_new.union(&movie_queue).collect::<Vec<_>>());
    println!("Movie ∪ Concert: {:?}", movie_queue.union(&concert_queue_new).collect::<Vec<_>>());

    // Difference operations
    println!("Concert - Movie: {:?}", concert_queue_new.difference(&movie_queue).collect::<Vec<_>>());
    println!("Movie - Concert: {:?}", movie_queue.difference(&concert_queue_new).collect::<Vec<_>>());

    // Symmetric difference operations
    println!("Concert ⊕ Movie: {:?}", concert_queue_new.symmetric_difference(&movie_queue).collect::<Vec<_>>());
    println!("Movie ⊕ Concert: {:?}", movie_queue.symmetric_difference(&concert_queue_new).collect::<Vec<_>>());

    // Set relationship checks
    println!("Concert disjoint from Movie: {:?}", concert_queue_new.is_disjoint(&movie_queue));
    println!("Movie disjoint from Concert: {:?}", movie_queue.is_disjoint(&concert_queue_new));

    let mut attendees: HashSet<&str> = HashSet::new();
    attendees.insert("Boris");
    println!("Concert subset of Movie: {:?}", concert_queue_new.is_subset(&movie_queue));
    println!("Attendees subset of Concert: {:?}", attendees.is_subset(&concert_queue_new));
    println!("Movie superset of Attendees: {:?}", movie_queue.is_superset(&attendees));

    println!("\n=== Summary ===");
    println!("HashMap: Key-value pairs, allows duplicate values but unique keys");
    println!("HashSet: Unique values only, no duplicates, supports set operations");
}