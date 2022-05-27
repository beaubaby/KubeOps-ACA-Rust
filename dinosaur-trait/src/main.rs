pub struct Trex { 
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

fn main() {
    let Character: Trex = Dinosaur::new();
    println!("So, it can eat {} while its mood is {}", Character.eat().to_string(), Character.mood());
}
