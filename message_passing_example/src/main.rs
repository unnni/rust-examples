use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time::Duration};

struct Task {
    id: u32,
    payload: String
}

struct Worker {
    id: u32
}

impl Worker{
    fn process_task(&self, task: Task) -> String {
        format!("Worker {} processed task {}: {}", self.id, task.id, task.payload)
    }
}

fn create_worker(id: u32) -> Worker {
    Worker{ id: id }
}

fn create_task(id: u32, payload: &str) -> Task {
    Task { id: id , payload: payload.to_string() }
}

fn main() {

    let mut tasks: Vec<Task> = vec![
        create_task(1, "Task 1"),
        create_task(2, "Task 2"),
        create_task(3, "Task 3"),
        create_task(4, "Task 4"),
        create_task(5, "Task 5"),
        create_task(6, "Task 6"),
        create_task(7, "Task 7"),
        create_task(8, "Task 8"),
        create_task(9, "Task 9"),
        create_task(10, "Task 10"),
        create_task(11, "Task 11"),
        create_task(12, "Task 12"),
        create_task(13, "Task 13"),
        ];
    let worker_threads: Vec<Worker> = vec![create_worker(1), create_worker(2),create_worker(3), create_worker(4)];

    let (tx, rx) = mpsc::channel();
    let (tx_results, rx_results) = mpsc::channel();
    for task in tasks {
        tx.send(task).unwrap();
    };

    let mut handles = Vec::new();
    let rx = Arc::new(Mutex::new(rx));
    for worker in worker_threads {
        let rx = Arc::clone(&rx);
        let tx_results = tx_results.clone();
        let handle = thread::spawn(move ||{
            while let Ok(task) = rx.lock().unwrap().recv() {
                let result = worker.process_task(task);
                tx_results.send(result).unwrap();
            }
        });
        handles.push(handle);
    }

    drop(tx);
    for result in rx_results {
        println!("Recieved {} in results channel", result);
    }

    for handle in handles{
        handle.join().unwrap();
    }
}
