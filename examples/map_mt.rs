use std::{
    cmp,
    sync::{mpsc::Sender, Arc},
    thread,
};

const DEFAULT_NUM_THREADS: usize = 4;
const THREAD_MUL: usize = 4;

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

fn map_mt<T>(
    num_threads: usize,
    elements: Arc<Vec<T>>,
    transform: fn(T) -> T,
    tx: Sender<T>,
) -> Result<(), &'static str>
where
    T: Copy + Send + Sync + 'static,
{
    let elements_len = elements.len();
    for n in 0..cmp::min(num_threads, elements_len) {
        let elements_clone = Arc::clone(&elements);
        let tx_clone = tx.clone();
        let window = elements_len.div_ceil(num_threads);
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

fn main() -> Result<(), &'static str> {
    let (tx, rx) = std::sync::mpsc::channel();

    let num_threads = match thread::available_parallelism() {
        Ok(num_threads) => num_threads.get(),
        Err(_) => DEFAULT_NUM_THREADS,
    };
    let elements = (1..=5000).collect();
    map_mt(
        num_threads * THREAD_MUL,
        Arc::new(elements),
        sum_of_divisors,
        tx,
    )?;

    while let Ok(n) = rx.recv() {
        println!("{n}");
    }

    Ok(())
}
