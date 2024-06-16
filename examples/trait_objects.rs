// Super trait Survive that depends on Eat + Drink
trait Survive: Eat + Drink {}

trait Eat {
    fn eat(&self) -> String;
}

trait Drink {
    fn drink(&self) -> String;
}

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

fn main() {
    let animals: Vec<Box<dyn Survive>> = vec![Box::new(Cat {}), Box::new(Dog {})];

    for animal in animals {
        println!("{}", animal.eat());
    }
}
