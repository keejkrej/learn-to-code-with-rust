use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot Dogs", "Burgers", "Pretzels"]);

    if let Some(removed) = sauces_to_meals.remove("Mayonnaise") {
        println!("Removed Mayonnaise: {:?}", removed);
    } else {
        println!("Mayonnaise not found.");
    }

    if let Some(meals) = sauces_to_meals.get("Mustard") {
        println!("Meals with Mustard: {:?}", meals);
    } else {
        println!("Mustard not found.");
    }

    sauces_to_meals
        .entry("Soy Saunce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("Sauces to Meals: {:?}", sauces_to_meals);
}
