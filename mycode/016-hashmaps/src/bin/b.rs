use std::collections::HashMap;

fn main() {
    let data = [
        ("Bobby", 7),
        ("Grant", 4),
        ("Ben", 6)
    ];

    let mut years_at_company = HashMap::from(data);

    println!("{years_at_company:?}");

    if let Some(grant_years) = years_at_company.remove("Gran") {
        println!("Gran is fired and has been at the company for {grant_years} years");
    }
}