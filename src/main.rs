pub mod assertions;
mod macros;

#[derive(Debug, Eq, PartialEq)]
enum Vars {
    VarA(u32),
    _VarB,
    _VarC,
    _VarD,
}


fn main() {
    inst_assert_eq!(true, true);

    inst_assert!(true);
    fast_assert!(true);
    moderate_assert!(true);
    slow_assert!(true);

    let t: Option<u32> = None;
    let tt: Result<u32, ()> = Err(());

    assert_some!(Some(1));
    assert_none!(t);
    assert_ok!(Ok::<i32, ()>(1));
    assert_err!(tt);

    let ttt = Vars::VarA(0);
    assert_var!(ttt, Vars::VarA(_));
    inst_assert_eq!(ttt, Vars::VarA(0));
}
