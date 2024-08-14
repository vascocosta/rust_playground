use std::sync::{mpsc::Sender, Arc};

trait Number {
    fn to_number(&self) -> isize;
}

impl Number for String {
    fn to_number(&self) -> isize {
        self.parse().unwrap_or_default()
    }
}

impl Number for &str {
    fn to_number(&self) -> isize {
        self.parse().unwrap_or_default()
    }
}

impl Number for isize {
    fn to_number(&self) -> isize {
        *self
    }
}

fn factorial<T: Number>(number: &T) -> isize {
    if number.to_number() == 0 {
        1
    } else {
        number.to_number() * factorial(&(number.to_number() - 1))
    }
}

fn factorial_mt<T>(numbers: Arc<Vec<T>>, channel: Sender<isize>)
where
    T: Number + Send + Sync + 'static,
{
    let numbers1 = Arc::clone(&numbers);
    let channel1 = channel.clone();

    std::thread::spawn(move || {
        for n in &numbers1[0..numbers1.len() / 2] {
            channel1.send(factorial(n)).unwrap();
        }
    });

    let numbers2 = Arc::clone(&numbers);
    let channel2 = channel.clone();

    std::thread::spawn(move || {
        for n in &numbers2[numbers.len() / 2..numbers.len()] {
            channel2.send(factorial(n)).unwrap();
        }
    });
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    //let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let numbers = vec!["1"];
    // let numbers = vec![String::from("1"), String::from("2")];
    factorial_mt(Arc::new(numbers), tx);

    while let Ok(number) = rx.recv() {
        println!("{number}");
    }
}
