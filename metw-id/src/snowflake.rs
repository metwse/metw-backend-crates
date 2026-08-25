//! An ID generator inspired by Twitter's Snowflake format.
//!
//! | Field | Bits | Description |
//! | -- | -- | -- |
//! | Timestamp | 22 to 63 | Milliseconds since the metw.cc [`EPOCH`]. |
//! | Constant | 12 to 21 | User-specified constant (defaults to 0). |
//! | Increment | 0 to 11 | Increments with every generated ID. |

use crate::checked_now;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::{
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicI64, Ordering},
    },
    time::Duration,
};

/// Base timestamp for metw.cc.
///
/// 2022 Aug 12, 00:00:00 (`1660262400000`)
pub static EPOCH: LazyLock<u64> = LazyLock::new(|| {
    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 8, 12).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    )
    .and_utc()
    .timestamp_millis() as u64
});

#[cfg(test)]
static INCREMENT_MAX: i64 = 2i64.pow(4);
#[cfg(not(test))]
static INCREMENT_MAX: i64 = 2i64.pow(12);

struct SnowflakeState {
    last_overflow: i64,
    increment: i64,
}

static GLOBAL_STATE: Mutex<SnowflakeState> = Mutex::new(SnowflakeState {
    last_overflow: 0,
    increment: 0,
});

static SNOWFLAKE_CONSTANT: AtomicI64 = AtomicI64::new(0);

/// Returns the next Snowflake ID.
pub fn next() -> i64 {
    let timestamp = checked_now().timestamp_millis() - *EPOCH as i64;

    // Ensure the time is not yet May 15 2109 07:35:11
    assert!(timestamp < 2i64.pow(42) - 1);

    let mut state = GLOBAL_STATE.lock().unwrap();

    if state.last_overflow == timestamp {
        drop(state);

        std::thread::sleep(Duration::from_millis(1));
        return next();
    }

    if state.increment == 0 {
        state.last_overflow = timestamp;
    }

    state.increment += 1;

    if state.increment == INCREMENT_MAX {
        state.increment = 0;
    }

    let snowflake_constant = SNOWFLAKE_CONSTANT.load(Ordering::Relaxed);

    (timestamp << 22) | (snowflake_constant << 12) | state.increment
}

/// Sets the Snowflake constant.
///
/// Note: The constant cannot be greater than 2¹⁰-1, or less than 0
pub fn load_constant(snowflake_constant: i64) {
    assert!(
        (0..=1023).contains(&snowflake_constant),
        "snowflake constant must be between 0 and 1023"
    );

    SNOWFLAKE_CONSTANT.store(snowflake_constant, Ordering::Relaxed);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stress_snowflake() {
        use std::collections::HashSet;

        let cap = 2usize.pow(12);
        let mut snowflakes = HashSet::with_capacity(cap);

        for _ in 0..cap {
            let snowflake = next();

            assert!(!snowflakes.contains(&snowflake));

            snowflakes.insert(snowflake);
        }
    }

    #[test]
    #[should_panic]
    fn large_constant() {
        load_constant(2i64.pow(10));
    }

    #[test]
    #[should_panic]
    fn negative_constant() {
        load_constant(-1);
    }

    #[test]
    fn constant() {
        let constant = 2i64.pow(10 - 1);

        load_constant(constant);

        let snowflake = next();

        assert_eq!((snowflake >> 12) & (2i64.pow(10) - 1), constant);

        load_constant(0);
    }
}
