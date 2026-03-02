use std::time::Duration;

const INITIAL_BACKOFF_MS: u64 = 1000;
const MAX_BACKOFF_MS: u64 = 60_000;
const MAX_RETRIES: u32 = 5;

pub struct BackoffStrategy {
    pub initial: Duration,
    pub max: Duration,
    pub max_retries: u32,
}

impl Default for BackoffStrategy {
    fn default() -> Self {
        Self {
            initial: Duration::from_millis(INITIAL_BACKOFF_MS),
            max: Duration::from_millis(MAX_BACKOFF_MS),
            max_retries: MAX_RETRIES,
        }
    }
}

impl BackoffStrategy {
    pub fn delay_for_attempt(&self, attempt: u32) -> Option<Duration> {
        if attempt >= self.max_retries {
            return None;
        }

        let base_ms = self.initial.as_millis() as u64 * 2u64.pow(attempt);
        let capped_ms = base_ms.min(self.max.as_millis() as u64);

        // Add jitter: random value between 0 and capped_ms/2
        let jitter = (capped_ms / 2).wrapping_mul(simple_random() % 100) / 100;
        let total_ms = capped_ms + jitter;

        Some(Duration::from_millis(
            total_ms.min(self.max.as_millis() as u64),
        ))
    }

    pub fn delay_with_retry_after(&self, retry_after_secs: u64, attempt: u32) -> Option<Duration> {
        if attempt >= self.max_retries {
            return None;
        }
        Some(Duration::from_secs(retry_after_secs))
    }
}

// Simple pseudo-random for jitter without pulling in rand
fn simple_random() -> u64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_attempt_returns_delay() {
        let strategy = BackoffStrategy::default();
        let delay = strategy.delay_for_attempt(0);
        assert!(delay.is_some());
        assert!(delay.unwrap() >= Duration::from_millis(1000));
    }

    #[test]
    fn delay_increases_with_attempts() {
        let strategy = BackoffStrategy {
            initial: Duration::from_millis(1000),
            max: Duration::from_secs(120),
            max_retries: 5,
        };
        // Base doubles each attempt: 1000, 2000, 4000, 8000
        let d0 = strategy.delay_for_attempt(0).unwrap();
        let d2 = strategy.delay_for_attempt(2).unwrap();
        // d2 base (4000ms) should be greater than d0 base (1000ms), even with jitter
        assert!(d2 >= d0);
    }

    #[test]
    fn max_retries_returns_none() {
        let strategy = BackoffStrategy::default();
        assert!(strategy.delay_for_attempt(5).is_none());
    }

    #[test]
    fn delay_capped_at_max() {
        let strategy = BackoffStrategy {
            initial: Duration::from_secs(30),
            max: Duration::from_secs(60),
            max_retries: 10,
        };
        let delay = strategy.delay_for_attempt(4);
        assert!(delay.is_some());
        assert!(delay.unwrap() <= Duration::from_secs(60));
    }

    #[test]
    fn retry_after_respected() {
        let strategy = BackoffStrategy::default();
        let delay = strategy.delay_with_retry_after(45, 0);
        assert_eq!(delay, Some(Duration::from_secs(45)));
    }
}
