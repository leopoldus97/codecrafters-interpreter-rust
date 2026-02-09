pub mod error;
pub mod report;

use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn next_id() -> u64 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
