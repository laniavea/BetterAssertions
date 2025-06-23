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
}
