trait Censor {
    fn censor(&self, input: &str) -> String;
}

struct Swearing {
    list: Vec<String>,
}

impl Swearing {
    fn new() -> Self {
        Swearing::default()
    }

    fn list(&self) -> Vec<String> {
        self.list.to_owned()
    }
}

impl Default for Swearing {
    fn default() -> Self {
        Self {
            list: vec![
                String::from("cunt"),
                String::from("cock"),
                String::from("fuck"),
                String::from("shit"),
            ],
        }
    }
}

impl Censor for Swearing {
    fn censor(&self, input: &str) -> String {
        let list = self.list();

        input
            .split(" ")
            .map(|w| {
                if list.contains(&String::from(w)) {
                    ""
                } else {
                    w
                }
            })
            .collect()
    }
}

struct NoCensor;

impl Censor for NoCensor {
    fn censor(&self, input: &str) -> String {
        input.to_owned()
    }
}

fn static_censorship<C: Censor>(input: &str, censor: C) -> String {
    censor.censor(input)
}

fn dynamic_censorship(input: &str, censor: &dyn Censor) -> String {
    censor.censor(input)
}

fn main() {
    //println!("{}", static_censorship("fuck this shit", Swearing::new()));
    println!("{}", dynamic_censorship("fuck this shit", &Swearing::new()));
}
