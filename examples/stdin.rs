use std::{error::Error, io::stdin, str::FromStr};

#[derive(Debug)]
enum Sex {
    Female,
    Male,
    Other,
}

impl FromStr for Sex {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "female" {
            Ok(Sex::Female)
        } else if s.to_lowercase() == "male" {
            Ok(Sex::Male)
        } else {
            Ok(Sex::Other)
        }
    }
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: i32,
    height: u8,
    weight: u8,
    sex: Sex,
}

impl Person {
    fn new(
        first_name: String,
        last_name: String,
        age: i32,
        height: u8,
        weight: u8,
        sex: Sex,
    ) -> Self {
        Self {
            first_name,
            last_name,
            age,
            height,
            weight,
            sex,
        }
    }
}

fn read_person() -> Result<Person, Box<dyn Error>> {
    let mut input = String::new();

    println!("First Name?");
    stdin().read_line(&mut input)?;
    let first_name: String = input.trim().parse()?;
    input.clear();

    println!("Last Name?");
    stdin().read_line(&mut input)?;
    let last_name: String = input.trim().parse()?;
    input.clear();

    println!("Age?");
    std::io::stdin().read_line(&mut input)?;
    let age: i32 = input.trim().parse::<i32>()?;
    input.clear();

    println!("Height?");
    std::io::stdin().read_line(&mut input)?;
    let height: u8 = input.trim().parse()?;
    input.clear();

    println!("Weight?");
    std::io::stdin().read_line(&mut input)?;
    let weight: u8 = input.trim().parse()?;
    input.clear();

    println!("Sex?");
    std::io::stdin().read_line(&mut input)?;
    let sex: Sex = input.trim().parse()?;

    Ok(Person::new(first_name, last_name, age, height, weight, sex))
}

fn main() {
    let p1 = Person::new(
        String::from("Vasco"),
        String::from("Costa"),
        42,
        173,
        60,
        Sex::Male,
    );
    let p2 = Person::new(
        String::from("Diana"),
        String::from("Almeida"),
        41,
        162,
        50,
        Sex::Female,
    );

    loop {
        match read_person() {
            Ok(p3) => {
                println!("{:?}\n{:?}\n{:?}", p1, p2, p3);
                break;
            }
            Err(_) => continue,
        }
    }
}
