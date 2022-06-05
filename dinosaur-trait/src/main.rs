pub struct Trex { 
    pub eat: String, 
    pub mood: String,
}

pub struct Camarasaurus {
    pub eat: String,
    pub mood: String,
}

trait Dinosaur {
    fn new() -> Self;
    // behevior
    fn eat(&self) -> String;
    // property           
    // fn color(&self) -> String; 
    fn mood(&self) -> String;
}

// Implement the `Dinosaur` trait for `Trex`.
impl Dinosaur for Trex {
    fn new() -> Self {
        Trex { eat: String::from("meat"), mood: String::from("mad") }
    }

    fn eat(&self) -> String {
        println!("T-rex eats meat");
        let food = String::from("Human");
        return food;
    }

    fn mood(&self) -> String {
        println!("T-rex is mad");
        let feeling = String::from("Angry");
        return feeling;
    }
}

impl Dinosaur for Camarasaurus {
    fn new() -> Self {
        Camarasaurus { eat: String::from("vegie"), mood: String::from("kindness") }
    }

    fn eat(&self) -> String {
        println!("Camarasaurus eats vegie");
        let food = String::from("leaf");
        return food;
    }

    fn mood(&self) -> String {
        println!("Camarasaurus is kindness");
        let feeling = String::from("fun");
        return feeling;
    }
}

fn main() {
    let character: Trex = Dinosaur::new();
    println!("So, it can eat {} while its mood is {}\n", character.eat().to_string(), character.mood());

    let character: Camarasaurus = Dinosaur::new();
    println!("So, it can eat {} while its mood is {}", character.eat().to_string(), character.mood());
}
