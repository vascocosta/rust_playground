use std::{
    sync::{Arc, Mutex},
    thread::{sleep, JoinHandle},
    time::Duration,
};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Self {
        Self {
            name: name.to_string(),
            left,
            right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        //sleep(Duration::from_millis(1)); // Delay locking to allow deadlock.
        let _right = table.forks[self.right].lock().unwrap();

        println!("Philosopher {} is eating...", self.name);
        sleep(Duration::from_millis(1000));
        println!("Philosopher {} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Aristotle", 0, 1),
        Philosopher::new("Socrates", 1, 2),
        Philosopher::new("Plato", 2, 3),
        Philosopher::new("Kant", 3, 4),
        Philosopher::new("Locke", 0, 4),
    ];

    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let handles: Vec<JoinHandle<()>> = philosophers
        .into_iter()
        .map(|p| {
            let table = Arc::clone(&table);

            std::thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
