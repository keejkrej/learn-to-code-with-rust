use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    println!("{}", coffee_pairings.len());
    println!("{coffee_pairings:?}");

    if let Some(&value) = coffee_pairings.get("Espresso") {
        println!("{}", value);
    } else {
        println!("No value found");
    }

    coffee_pairings.insert("Latte", "Pistachio Milk");
    println!("{coffee_pairings:?}");

    coffee_pairings
        .entry("Flat White")
        .or_insert("Pistachio Milk");
    println!("{coffee_pairings:?}");

    coffee_pairings.entry("Espresso").or_insert("Oat Milk");
    println!("{coffee_pairings:?}");
}
