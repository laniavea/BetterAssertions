mod assertions;
mod macros;

fn main() {
    inst_assert_eq!(true, true);

    inst_assert!(true);
    fast_assert!(true);
    moderate_assert!(true);
    slow_assert!(true);
    println!("Hello, world!");
}
