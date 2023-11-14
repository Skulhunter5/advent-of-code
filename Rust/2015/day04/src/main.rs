use std::{fs, thread, sync::{atomic::{AtomicBool, Ordering, AtomicU64}, Arc, Mutex}};

fn main() {
    let input_raw = fs::read_to_string("../inputs/04-input.txt").unwrap();
    let input = input_raw.lines().next().unwrap().to_string();

    let solution = solve_fast(input);
    println!("Part 1: {}", solution.0);
    println!("Part 2: {}", solution.1);
}

fn solve(input: String) -> (u64, u64) {
    let mut i = 0;
    let mut result1 = None;
    let mut result2 = None;
    while result2.is_none() {
        let digest = md5::compute(format!("{}{}", input, i));
        if result1.is_none() && format!("{:x}", digest).starts_with("00000") {
            result1 = Some(i);
        }
        if format!("{:x}", digest).starts_with("000000") {
            result2 = Some(i);
        }
        i += 1;
    }
    (result1.unwrap(), result2.unwrap())
}

static DONE: AtomicBool = AtomicBool::new(false);
static NEXT_NUMBER: AtomicU64 = AtomicU64::new(0);

const WORKLOAD_WINDOWS: u64 = 100;

fn solve_fast(input: String) -> (u64, u64) {
    let mut thread_pool = Vec::new();
    let results = Arc::new(Mutex::new(Vec::new()));
    for _ in 0..6 {
        let results_clone = results.clone();
        let input = input.clone();
        thread_pool.push(thread::spawn(move || {
            let result = solver_thread(input);
            let mut results = results_clone.lock().unwrap();
            results.push(result);
        }));
    }
    while !thread_pool.is_empty() {
        thread_pool.pop().unwrap().join().unwrap();
    }
    let results = results.lock().unwrap();
    let solution1 = results.iter()
        .map(|x| x.0)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .min().unwrap();
    let solution2 = results.iter()
        .map(|x| x.1)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .min().unwrap();

    (solution1, solution2)
}

fn solver_thread(input: String) -> (Option<u64>, Option<u64>) {
    let mut result = (None, None);
    'outer:while !DONE.load(Ordering::Relaxed) {
        let i = NEXT_NUMBER.fetch_add(WORKLOAD_WINDOWS, Ordering::Relaxed);
        for i in i..(i+WORKLOAD_WINDOWS) {
            let digest = md5::compute(format!("{}{}", input, i));
            if result.0.is_none() && format!("{:x}", digest).starts_with("00000") {
                result.0 = Some(i);
            }
            if format!("{:x}", digest).starts_with("000000") {
                result.1 = Some(i);
                DONE.store(true, Ordering::Relaxed);
                break 'outer;
            }
        }
    }
    result
}
