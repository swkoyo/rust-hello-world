use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

fn increment_counter(thread_name: char, m: Arc<Mutex<i32>>, reps: i32) {
    for _ in 0..reps {
        {
            let mut val = m.lock().unwrap();
            *val += 1;
            if *val % 100 == 0 {
                println!("Thread {} val {}", thread_name, *val);
            }
        }
    }
}

fn main() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let iterations = 1000;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    {
        let x = Arc::clone(&counter);
        let handle = thread::spawn(move || increment_counter('a', x, iterations));
        handles.push(handle);
    }
    {
        let y = Arc::clone(&counter);
        let handle = thread::spawn(move || increment_counter('b', y, iterations));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
