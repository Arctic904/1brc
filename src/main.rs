pub mod types;

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::{mpsc, Arc, Mutex, RwLock},
    thread::{self, sleep},
    time,
};

use types::parsing::{parse_entry, Stations};

const NUM_THREADS: usize = 5; // Number of worker threads
const LINES_PER_BATCH: usize = 1000; // Tune this for performance

fn worker(
    id: usize,
    queue: Arc<Mutex<VecDeque<Vec<String>>>>,
    finished: Arc<RwLock<bool>>,
    station_list: Arc<RwLock<Stations>>,
) {
    // sleep(time::Duration::from_millis(50));
    loop {
        let batch = {
            // println!("Batch check {}", id);
            let mut queue_lock = queue.lock().expect("Unable to unlock queue");
            // println!("{}", queue_lock.len());

            let var: Option<Vec<String>>;
            if queue_lock.is_empty() {
                drop(queue_lock);
                sleep(time::Duration::from_millis(50));
                var = Option::None;
                let fin = finished.read().unwrap();
                if *fin {
                    println!("Thread returned: {}", id);
                    break;
                }
            } else {
                var = queue_lock.pop_front();
            };
            var
        };

        // Process batch if available
        if let Some(lines) = batch {
            for contents in lines {
                parse_entry(&contents, contents.len(), Arc::clone(&station_list));
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("measurements.txt").expect("file should be readable");
    let buf_reader = BufReader::new(file);
    let station_list = Arc::new(RwLock::new(Stations(HashMap::new())));
    let finished = Arc::new(RwLock::new(false));

    let queue = Arc::new(Mutex::new(VecDeque::new()));

    // Spawn worker threads
    let mut handles = vec![];
    for i in 0..NUM_THREADS {
        let clone_list = Arc::clone(&station_list);
        let finished = Arc::clone(&finished);
        let queue_clone = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            worker(i, queue_clone, finished, clone_list)
        }));
    }

    // Read file in batches of lines and add to queue
    let mut batch = Vec::with_capacity(LINES_PER_BATCH);
    let mut lines_to_proc = 1_000_000_000;
    for line in buf_reader.lines() {
        if let Ok(line) = line {
            lines_to_proc -= 1;
            if lines_to_proc % 100_000_000 == 0 {
                println!("{}", lines_to_proc);
            }
            if line.len() > 4 {
                batch.push(line);
                if batch.len() >= LINES_PER_BATCH {
                    queue.lock().unwrap().push_back(batch);
                    batch = Vec::with_capacity(LINES_PER_BATCH); // Reset batch
                }
            }
        }
    }

    if !batch.is_empty() {
        queue.lock().unwrap().push_back(batch);
    }

    loop {
        if queue.lock().unwrap().is_empty() {
            println!("Finished");
            let mut fin = finished.write().unwrap();
            *fin = true;
            break;
        }
    }

    drop(queue);

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finished processing");
    let file = std::fs::File::create("baseline_output.txt").expect("opening output file for write");
    let mut buf = std::io::BufWriter::new(file);
    write!(buf, "{}", station_list.read().unwrap())?;
    Ok(())
}
