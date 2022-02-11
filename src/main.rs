use parking_lot::{Condvar, Mutex};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = dhat::Profiler::new_heap();
    let mutex = Arc::new(Mutex::new(()));
    let condvar = Arc::new(Condvar::new());
    let mut handles = Vec::new();
    for _ in 0..32 {
        let condvar = condvar.clone();
        let mutex = mutex.clone();
        handles.push(std::thread::spawn(move || {
            let mut guard = mutex.lock();
            condvar.wait(&mut guard);
            condvar.notify_one();
        }));
    }
    sleep(Duration::from_millis(1000));
    {
        let _guard = mutex.lock();
        condvar.notify_one();
    }
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
