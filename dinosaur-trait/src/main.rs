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
        let character = String::from("carnivorous dinosaurs");
        return character;
    }

    fn mood(&self) -> String {
        println!("T-rex is mad");
        let character = String::from("carnivorous dinosaurs");
        return character;
    }
}

fn main() {
    let Baryonyx: Trex = Dinosaur::new();
    println!("eat = {} \n mood = {}", Baryonyx.eat().to_string(), Baryonyx.mood());
    // Type annotation is necessary in this case.
    // let mut dolly: Sheep = Animal::new("Dolly");
    // // TODO ^ Try removing the type annotations.

    // dolly.talk();
    // dolly.shear();
    // dolly.talk();
}
