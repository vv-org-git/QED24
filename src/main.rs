mod sim;
use std::collections::HashMap;
use std::iter::Enumerate;
use std::sync::{Arc, Mutex};
use std::thread;
use sim::Sim;
use plotly::{Bar, Plot};
fn main() {
    let trials = 100000;
    let threads = 10;
    let n = 100;
    let mut freq: Arc<Mutex<HashMap<i32, i32>>> = Arc::new(Mutex::new(HashMap::new())); 
    let mut handles = vec![];

    for _ in 0..threads {
        let data_clone = Arc::clone(&freq);
        let handle = thread::spawn(move || {
            let mut freq = data_clone.lock().unwrap();

            for _ in 0..trials{
                let mut s = Sim::new(n);
                let r =  s.run();
                freq.entry(r).and_modify(|e| *e += 1).or_insert(1);
    
            }
           
            
             // Safely increment the shared data
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let keys: Vec<_> = (*freq.lock().unwrap()).keys().cloned().collect(); // Cloning keys
    let values: Vec<_> = (*freq.lock().unwrap()).values().cloned().collect(); // Cloning values

    for (i, vals) in values.iter().enumerate() {
        println!("{i}: {vals}");
    }
    let t = Bar::new(keys, values);
    let mut plot = Plot::new();
    plot.add_trace(t);

    plot.show();
}
