#[macro_export]
macro_rules! __simple_assert {
    ($cond:expr, $lvl:expr) => {
        let __lvl = $lvl;

        if __lvl <= $crate::assertions::STATIC_MAX_LEVEL {
            if !$cond {
                panic!(
                    "Assertion failed"
                );
            }
        }
    };
}

#[macro_export]
macro_rules! __simple_assert_eq {
    ($cond:expr, $sec_cond:expr, $lvl:expr) => {
        let __lvl = $lvl;

        if __lvl <= $crate::assertions::STATIC_MAX_LEVEL {
            if $cond != $sec_cond {
                panic!(
                    "Assertion failed"
                );
            }
        }
    };
}


#[macro_export]
macro_rules! inst_assert {
    ($cond:expr) => {
        $crate::__simple_assert!($cond, $crate::assertions::AssertionLevel::Instant)
    };
}

#[macro_export]
macro_rules! fast_assert {
    ($cond:expr) => {
        $crate::__simple_assert!($cond, $crate::assertions::AssertionLevel::Fast)
    };
}

#[macro_export]
macro_rules! moderate_assert {
    ($cond:expr) => {
        $crate::__simple_assert!($cond, $crate::assertions::AssertionLevel::Moderate)
    };
}

#[macro_export]
macro_rules! slow_assert {
    ($cond:expr) => {
        $crate::__simple_assert!($cond, $crate::assertions::AssertionLevel::Slow)
    };
}

#[macro_export]
macro_rules! inst_assert_eq {
    ($cond:expr, $other_cond:expr) => {
        $crate::__simple_assert_eq!($cond, $other_cond, $crate::assertions::AssertionLevel::Instant)
    };
}

#[macro_export]
macro_rules! fast_assert_eq {
    ($cond:expr, $other_cond:expr) => {
        $crate::__simple_assert_eq!($cond, $other_cond, $crate::assertions::AssertionLevel::Fast)
    };
}

#[macro_export]
macro_rules! moderate_assert_eq {
    ($cond:expr, $other_cond:expr) => {
        $crate::__simple_assert_eq!($cond, $other_cond, $crate::assertions::AssertionLevel::Moderate)
    };
}

#[macro_export]
macro_rules! slow_assert_eq {
    ($cond:expr, $other_cond:expr) => {
        $crate::__simple_assert_eq!($cond, $other_cond, $crate::assertions::AssertionLevel::Slow)
    };
}
