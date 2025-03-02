mod parallel_map {
    use std::{
        cmp,
        sync::{mpsc::Sender, Arc},
        thread,
    };

    const DEFAULT_NUM_THREADS: usize = 4;
    const THREAD_MUL: usize = 4;

    pub trait ParallelMap<T, F>
    where
        T: Copy + Send + Sync + 'static,
        F: Fn(T) -> T + Copy + Send + Sync + 'static,
    {
        fn map(
            self,
            num_threads: Option<usize>,
            transform: F,
            tx: Sender<T>,
        ) -> Result<(), &'static str>;
    }

    impl<T, F> ParallelMap<T, F> for Vec<T>
    where
        T: Copy + Send + Sync + 'static,
        F: Fn(T) -> T + Copy + Send + Sync + 'static,
    {
        fn map(
            self,
            num_threads: Option<usize>,
            transform: F,
            tx: Sender<T>,
        ) -> Result<(), &'static str> {
            let num_threads = match num_threads {
                Some(num_threads) => num_threads,
                None => match thread::available_parallelism() {
                    Ok(num_threads) => num_threads.get(),
                    Err(_) => DEFAULT_NUM_THREADS,
                },
            };

            let elements = Arc::new(self);
            let elements_len = elements.len();

            for n in 0..cmp::min(num_threads * THREAD_MUL, elements_len) {
                let elements_clone = Arc::clone(&elements);
                let tx_clone = tx.clone();

                let window = elements_len.div_ceil(num_threads * THREAD_MUL);

                thread::spawn(move || -> Result<(), &'static str> {
                    let start = n * window;
                    let end = cmp::min((n * window) + window, elements_len);

                    for element in &elements_clone[start..end] {
                        tx_clone
                            .send(transform(*element))
                            .map_err(|_| "Could not send to channel")?;
                    }

                    Ok(())
                });
            }

            Ok(())
        }
    }
}

use parallel_map::ParallelMap;

fn sum_of_divisors(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..=n {
        let mut count = 0;
        for j in 1..=(i as f64).sqrt() as usize {
            if i % j == 0 {
                count += 1;
                if j != i / j {
                    count += 1;
                }
            }
        }
        sum += count;
    }
    sum
}

fn main() -> Result<(), &'static str> {
    let (tx, rx) = std::sync::mpsc::channel();
    let elements: Vec<usize> = (1..=5000).collect();

    elements.map(None, sum_of_divisors, tx).unwrap();

    while let Ok(value) = rx.recv() {
        println!("{}", value);
    }

    Ok(())
}
