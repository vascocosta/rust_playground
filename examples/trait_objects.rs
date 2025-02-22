use std::time::Duration;

// Super trait Survive that depends on Eat + Drink
trait Survive: Eat + Drink + std::fmt::Debug {}

trait Eat {
    fn eat(&self) -> String;
}

trait Drink {
    fn drink(&self) -> String;
}

#[derive(Debug)]
struct Cat;

impl Eat for Cat {
    fn eat(&self) -> String {
        String::from("I eat like a cat!")
    }
}

impl Drink for Cat {
    fn drink(&self) -> String {
        String::from("I drink like a cat!")
    }
}

impl Survive for Cat {}

#[derive(Debug)]
struct Dog;

impl Eat for Dog {
    fn eat(&self) -> String {
        String::from("I eat like a dog!")
    }
}

impl Drink for Dog {
    fn drink(&self) -> String {
        String::from("I drink like a dog!")
    }
}

impl Survive for Dog {}

fn surviving_loop(animals: &[Box<dyn Survive>]) {
    loop {
        std::thread::sleep(Duration::from_secs(1));
        for animal in animals {
            println!("{}", animal.eat());
            println!("{}", animal.drink());
        }
    }
}

fn main() {
    let animals: Vec<Box<dyn Survive>> = vec![Box::new(Cat {}), Box::new(Dog {})];
    surviving_loop(&animals);

    println!("{:?}", animals);
}
