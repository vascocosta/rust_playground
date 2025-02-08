use std::{sync::Arc, time::Duration};

use tokio::{
    fs::{read_to_string, File},
    io::AsyncWriteExt,
    spawn,
    sync::RwLock,
    task::JoinHandle,
    time::sleep,
};

#[tokio::main]
async fn main() {
    let file_path = Arc::new(RwLock::new(String::from("db.txt")));

    let readers: Vec<JoinHandle<_>> = (0..2)
        .map(|id| {
            let file_path = Arc::clone(&file_path);
            spawn(reader_task(id, file_path))
        })
        .collect();

    let writers: Vec<JoinHandle<_>> = (0..1)
        .map(|id| {
            let file_path = Arc::clone(&file_path);
            spawn(writer_task(id, file_path))
        })
        .collect();

    for reader in readers {
        reader.await.expect("Could not await reader");
    }

    for writer in writers {
        writer.await.expect("Could not await writer");
    }
}

async fn reader_task(id: u8, file_path: Arc<RwLock<String>>) {
    loop {
        {
            let file_path = file_path.read().await;
            let contents = read_to_string(&*file_path)
                .await
                .expect("Could not read from file");
            println!("Reader {} sees:\n{}", id, contents);
        }

        sleep(Duration::from_secs(10)).await;
    }
}

async fn writer_task(id: u8, file_path: Arc<RwLock<String>>) {
    loop {
        {
            let file_path = file_path.write().await;
            let mut file = File::options()
                .append(true)
                .open(&*file_path)
                .await
                .expect("Could not open file");
            file.write_all(b"HELLO\n")
                .await
                .expect("Could not write to file");
            println!("Writer {} wrote: {}", id, "HELLO");
        }

        sleep(Duration::from_secs(30)).await;
    }
}
