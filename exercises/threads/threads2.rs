// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 用 Arc 和 Mutex 包装 JobStatus 以实现线程安全的共享
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status); // 共享 JobStatus
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250)); // 模拟工作
            // 加锁并更新 jobs_completed
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 打印 jobs_completed 的值
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}

