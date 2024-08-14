fn main() {
    const NUM_THREADS: u32 = 4;
    let (tx, rx) = std::sync::mpsc::channel();

    for i in 0..NUM_THREADS {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let mut list: Vec<String> = vec![];

            for number in (i * 250_000)..(i * 250_000) + 250_001 {
                if number % 7 == 0 || number % 10 == 7 {
                    list.push(String::from("SMAC"));
                } else {
                    list.push(number.to_string());
                }
            }

            tx.send((i, list)).unwrap();
        });
    }

    drop(tx);

    let mut lists: Vec<(u32, Vec<String>)> = vec![];

    while let Ok(list) = rx.recv() {
        lists.push(list);
    }

    lists.sort_by(|a, b| a.0.cmp(&b.0));

    for list in lists {
        print_list(&list.1);
    }
}

fn print_list(list: &[String]) {
    for number in list {
        println!("{number}");
    }
}
