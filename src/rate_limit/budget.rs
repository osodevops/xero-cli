use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub struct DailyBudget {
    limit: u64,
    used: AtomicU64,
    last_reset: Mutex<chrono::NaiveDate>,
}

impl DailyBudget {
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            used: AtomicU64::new(0),
            last_reset: Mutex::new(chrono::Utc::now().date_naive()),
        }
    }

    pub fn check_and_increment(&self) -> Result<u64, (u64, u64)> {
        self.maybe_reset();

        let current = self.used.fetch_add(1, Ordering::SeqCst);
        if current >= self.limit {
            self.used.fetch_sub(1, Ordering::SeqCst);
            Err((current, self.limit))
        } else {
            Ok(self.limit - current - 1)
        }
    }

    pub fn update_from_header(&self, remaining: u64) {
        let used = self.limit.saturating_sub(remaining);
        self.used.store(used, Ordering::SeqCst);
    }

    pub fn remaining(&self) -> u64 {
        self.maybe_reset();
        self.limit.saturating_sub(self.used.load(Ordering::SeqCst))
    }

    pub fn used(&self) -> u64 {
        self.maybe_reset();
        self.used.load(Ordering::SeqCst)
    }

    fn maybe_reset(&self) {
        let today = chrono::Utc::now().date_naive();
        let mut last = self.last_reset.lock().unwrap();
        if *last < today {
            self.used.store(0, Ordering::SeqCst);
            *last = today;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_starts_at_zero() {
        let budget = DailyBudget::new(5000);
        assert_eq!(budget.used(), 0);
        assert_eq!(budget.remaining(), 5000);
    }

    #[test]
    fn increment_tracks_usage() {
        let budget = DailyBudget::new(100);
        let remaining = budget.check_and_increment().unwrap();
        assert_eq!(remaining, 99);
        assert_eq!(budget.used(), 1);
    }

    #[test]
    fn budget_exhaustion() {
        let budget = DailyBudget::new(2);
        budget.check_and_increment().unwrap();
        budget.check_and_increment().unwrap();
        assert!(budget.check_and_increment().is_err());
    }

    #[test]
    fn update_from_header_syncs() {
        let budget = DailyBudget::new(5000);
        budget.update_from_header(4500);
        assert_eq!(budget.used(), 500);
        assert_eq!(budget.remaining(), 4500);
    }
}
