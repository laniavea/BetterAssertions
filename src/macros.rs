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
                    "Assertion failed! Got: {:?}, Expected: {:?}", $cond, $sec_cond
                );
            }
        }
    };
}


/// Do standart assertion with level: Instant
#[macro_export]
macro_rules! inst_assert {
    ($cond:expr) => {
        $crate::__simple_assert!($cond, $crate::assertions::AssertionLevel::Instant)
    };
    ($cond:expr, $assert_id:expr) => {
        let this_level = $crate::assertions::AssertionLevel::Instant;
        $crate::__simple_assert!($cond, this_level);
        if $crate::assertions::COLLECT_STATS_LVL <= this_level {
            let mut st = $crate::basic_store().lock().unwrap();
            st.store($assert_id, this_level);
        }
    };
}

/// Do standart assertion with level: Fast
#[macro_export]
macro_rules! fast_assert {
    ($cond:expr) => {
        $crate::__simple_assert!($cond, $crate::assertions::AssertionLevel::Fast)
    };
    ($cond:expr, $assert_id:expr) => {
        let this_level = $crate::assertions::AssertionLevel::Fast;
        $crate::__simple_assert!($cond, this_level);
        if $crate::assertions::COLLECT_STATS_LVL <= this_level {
            let mut st = $crate::basic_store().lock().unwrap();
            st.store($assert_id, this_level);
        }
    };
}

/// Do standart assertion with level: Moderate
#[macro_export]
macro_rules! moderate_assert {
    ($cond:expr) => {
        $crate::__simple_assert!($cond, $crate::assertions::AssertionLevel::Moderate)
    };
    ($cond:expr, $assert_id:expr) => {
        let this_level = $crate::assertions::AssertionLevel::Moderate;
        $crate::__simple_assert!($cond, this_level);
        if $crate::assertions::COLLECT_STATS_LVL <= this_level {
            let mut st = $crate::basic_store().lock().unwrap();
            st.store($assert_id, this_level);
        }
    };
}

/// Do standart assertion with level: Slow
#[macro_export]
macro_rules! slow_assert {
    ($cond:expr) => {
        $crate::__simple_assert!($cond, $crate::assertions::AssertionLevel::Slow)
    };
    ($cond:expr, $assert_id:expr) => {
        let this_level = $crate::assertions::AssertionLevel::Slow;
        $crate::__simple_assert!($cond, this_level);
        if $crate::assertions::COLLECT_STATS_LVL <= this_level {
            let mut st = $crate::basic_store().lock().unwrap();
            st.store($assert_id, this_level);
        }
    };
}

/// Do equality assertion with level: Instant
#[macro_export]
macro_rules! inst_assert_eq {
    ($cond:expr, $other_cond:expr) => {
        $crate::__simple_assert_eq!($cond, $other_cond, $crate::assertions::AssertionLevel::Instant)
    };
    ($cond:expr, $other_cond:expr, $assert_id:expr) => {
        let this_level = $crate::assertions::AssertionLevel::Instant;
        $crate::__simple_assert_eq!($cond, $other_cond, this_level);
        if $crate::assertions::COLLECT_STATS_LVL <= this_level {
            let mut st = $crate::basic_store().lock().unwrap();
            st.store($assert_id, this_level);
        }
    };
}

/// Do equality assertion with level: Fast
#[macro_export]
macro_rules! fast_assert_eq {
    ($cond:expr, $other_cond:expr) => {
        $crate::__simple_assert_eq!($cond, $other_cond, $crate::assertions::AssertionLevel::Fast)
    };
    ($cond:expr, $other_cond:expr, $assert_id:expr) => {
        let this_level = $crate::assertions::AssertionLevel::Fast;
        $crate::__simple_assert_eq!($cond, $other_cond, this_level);
        if $crate::assertions::COLLECT_STATS_LVL <= this_level {
            let mut st = $crate::basic_store().lock().unwrap();
            st.store($assert_id, this_level);
        }
    };
}

/// Do equality assertion with level: Moderate
#[macro_export]
macro_rules! moderate_assert_eq {
    ($cond:expr, $other_cond:expr) => {
        $crate::__simple_assert_eq!($cond, $other_cond, $crate::assertions::AssertionLevel::Moderate)
    };
    ($cond:expr, $other_cond:expr, $assert_id:expr) => {
        let this_level = $crate::assertions::AssertionLevel::Moderate;
        $crate::__simple_assert_eq!($cond, $other_cond, this_level);
        if $crate::assertions::COLLECT_STATS_LVL <= this_level {
            let mut st = $crate::basic_store().lock().unwrap();
            st.store($assert_id, this_level);
        }
    };
}

/// Do equality assertion with level: Slow
#[macro_export]
macro_rules! slow_assert_eq {
    ($cond:expr, $other_cond:expr) => {
        $crate::__simple_assert_eq!($cond, $other_cond, $crate::assertions::AssertionLevel::Slow)
    };
    ($cond:expr, $other_cond:expr, $assert_id:expr) => {
        let this_level = $crate::assertions::AssertionLevel::Slow;
        $crate::__simple_assert_eq!($cond, $other_cond, this_level);
        if $crate::assertions::COLLECT_STATS_LVL <= this_level {
            let mut st = $crate::basic_store().lock().unwrap();
            st.store($assert_id, this_level);
        }
    };
}

/// Do Some(T) assertion at level Instant
#[macro_export]
macro_rules! assert_some {
    ($cond:expr) => {
        $crate::__simple_assert!($cond.is_some(), $crate::assertions::AssertionLevel::Instant)
    };
}

/// Do None assertion at level Instant
#[macro_export]
macro_rules! assert_none {
    ($cond:expr) => {
        $crate::__simple_assert!($cond.is_none(), $crate::assertions::AssertionLevel::Instant)
    };
}

/// Do Ok(T) assertion at level Instant
#[macro_export]
macro_rules! assert_ok {
    ($cond:expr) => {
        $crate::__simple_assert!($cond.is_ok(), $crate::assertions::AssertionLevel::Instant)
    };
}

/// Do Err(T) assertion at level Instant
#[macro_export]
macro_rules! assert_err {
    ($cond:expr) => {
        $crate::__simple_assert!($cond.is_err(), $crate::assertions::AssertionLevel::Instant)
    };
}

/// Do Enum(T) assertion at level Instant
#[macro_export]
macro_rules! assert_var {
    ($cond:expr, $var:pat) => {
        $crate::__simple_assert!(matches!($cond, $var), $crate::assertions::AssertionLevel::Instant)
    };
}
