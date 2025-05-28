use std::collections::HashMap;

trait Description {
    fn get_description(&self) -> String;
}

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl Description for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

fn mix_and_match(
    first: &mut (impl Accommodation + Description),
    second: &mut (impl Accommodation + Description),
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    book_for_one_night(&mut hotel, "Piers");
    println!("{:?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    book_for_one_night(&mut airbnb, "Piers");
    println!("{:?}", airbnb);

    mix_and_match(&mut hotel, &mut airbnb, "John");
    println!("{:?}\n{:?}", hotel, airbnb);
}
