use better_assertions::*;

fn call_me(lvl: &str) -> bool {
    println!("called {lvl}");
    true
}

fn main() {
    println!("IM PLAYGROUND");
    inst_assert!(call_me("inst"));
    fast_assert!(call_me("fast"));
    moderate_assert!(call_me("moderate"));
    slow_assert!(call_me("slow"));

    inst_assert_eq!(call_me("inst_2"), call_me("inst_2"));
    fast_assert_eq!(call_me("fast_2"), call_me("fast_2"));
    moderate_assert_eq!(call_me("moderate_2"), call_me("moderate_2"));
    slow_assert_eq!(call_me("slow_2"), call_me("slow_2"));
    println!("IM END");
}
