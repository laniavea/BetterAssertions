use std::cmp;

static ASSERTION_LEVEL_NAMES: [&str; 5] = ["OFF", "INSTANT", "FAST", "MODERATE", "SLOW"];

/// An enum representing the available levels of assertions difficulty
#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AssertionLevel {
    /// The "instant" level.
    ///
    /// Level created to for O(1) checks, for example checks for primitive types such as Booleans,
    /// Ints, Enums Varians and so on.
    Instant = 1,
    /// The "fast" level.
    ///
    /// Level designed for checks that can take microseconds to complete.
    Fast,
    /// The "moderate" level.
    ///
    /// Level designed for checks that can take millyseconds to complete. Should be used when data
    /// size can be large, but not supposed to be
    Moderate,
    /// The "slow" level.
    ///
    /// Level designed for checks that can strongly slow down your program. May be used to validate
    /// smth for large amount of data
    Slow,
}

impl PartialEq<AssertionLevelFilter> for AssertionLevel {
    fn eq(&self, other: &AssertionLevelFilter) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd<AssertionLevelFilter> for AssertionLevel {
    fn partial_cmp(&self, other: &AssertionLevelFilter) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }
}

/// An enum representing the availbale filters levels to do assertions for
#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AssertionLevelFilter {
    /// A level that will turn all assertions off
    Off = 0,
    /// A level to turn on 'Instant' level assertions only
    Instant,
    /// A level to turn on 'Fast' level assertions and lower
    Fast,
    /// A level to turn on 'Moderate' level assertions and lower
    Moderate,
    /// A level to turn on 'Slow' level assertions and lower
    Slow,
}

impl PartialEq<AssertionLevel> for AssertionLevelFilter {
    fn eq(&self, other: &AssertionLevel) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd<AssertionLevel> for AssertionLevelFilter {
    fn partial_cmp(&self, other: &AssertionLevel) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }
}

pub const STATIC_MAX_LEVEL: AssertionLevelFilter = match cfg!(debug_assertions) {
    true if cfg!(feature = "debug_max_level_off") => AssertionLevelFilter::Off,
    true if cfg!(feature = "debug_max_level_instant") => AssertionLevelFilter::Instant,
    true if cfg!(feature = "debug_max_level_fast") => AssertionLevelFilter::Fast,
    true if cfg!(feature = "debug_max_level_moderate") => AssertionLevelFilter::Moderate,
    true if cfg!(feature = "debug_max_level_slow") => AssertionLevelFilter::Slow,
    false if cfg!(feature = "max_level_off") => AssertionLevelFilter::Off,
    false if cfg!(feature = "max_level_instant") => AssertionLevelFilter::Instant,
    false if cfg!(feature = "max_level_fast") => AssertionLevelFilter::Fast,
    false if cfg!(feature = "max_level_moderate") => AssertionLevelFilter::Moderate,
    false if cfg!(feature = "max_level_slow") => AssertionLevelFilter::Slow,
    _ => AssertionLevelFilter::Slow
};

pub const COLLECT_STATS: bool = match cfg!(debug_assertions) {
    true => cfg!(feature = "debug_collect_stats"),
    false => cfg!(feature = "collect_stats"),
};
