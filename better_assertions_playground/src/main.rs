use better_assertions::*;

fn call_me(lvl: &str) -> bool {
    println!("called {lvl}");
    true
}

fn main() {
    println!("IM PLAYGROUND");
    inst_assert!(call_me("inst"), 0);

    basic_store().lock().unwrap().clear();

    for _ in 0..10 {
        inst_assert!(call_me("inst"), 2);
    }

    fast_assert!(call_me("fast"), 3);
    moderate_assert!(call_me("moderate"), 4);
    slow_assert!(call_me("slow"), 5);


    inst_assert_eq!(call_me("inst_2"), call_me("inst_2"), 6);
    fast_assert_eq!(call_me("fast_2"), call_me("fast_2"), 7);
    moderate_assert_eq!(call_me("moderate_2"), call_me("moderate_2"), 8);
    slow_assert_eq!(call_me("slow_2"), call_me("slow_2"), 9);
    println!("IM END");

    let hm = basic_store().lock().unwrap().dump();
    println!("{:?}", hm);

    basic_store().lock().unwrap().print();

    multithreading_test()
}

fn multithreading_test() {
    use std::thread;
    basic_store().lock().unwrap().clear();

    const NUM_THREADS: usize = 8;
    const ASSERTIONS_PER_THREAD: usize = 10_000;
    const SHARED_ASSERT_ID: u32 = 42;

    let mut handles = Vec::with_capacity(NUM_THREADS);

    for i in 0..NUM_THREADS {
        let handle = thread::spawn(move || {
            for _ in 0..ASSERTIONS_PER_THREAD {
                inst_assert!(true, SHARED_ASSERT_ID);
            }
            println!("Thread {i} finished");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let hm = basic_store().lock().unwrap().dump();

    assert_eq!(hm.get(&SHARED_ASSERT_ID).unwrap().number_of_calls(), (NUM_THREADS * ASSERTIONS_PER_THREAD) as u32);
    println!("All threads completed: {:?}", hm);
}
