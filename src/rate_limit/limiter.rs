use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};

pub struct RateLimiter {
    window: Arc<Mutex<SlidingWindow>>,
    semaphore: Arc<Semaphore>,
}

struct SlidingWindow {
    timestamps: VecDeque<Instant>,
    max_per_minute: u32,
}

impl RateLimiter {
    pub fn new(calls_per_minute: u32, max_concurrent: u32) -> Self {
        Self {
            window: Arc::new(Mutex::new(SlidingWindow {
                timestamps: VecDeque::new(),
                max_per_minute: calls_per_minute,
            })),
            semaphore: Arc::new(Semaphore::new(max_concurrent as usize)),
        }
    }

    pub async fn acquire(&self) -> RateLimitGuard {
        // Acquire concurrency permit
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore closed");

        // Wait for sliding window slot
        loop {
            let wait_time = {
                let mut window = self.window.lock().await;
                let now = Instant::now();
                let one_minute_ago = now - Duration::from_secs(60);

                // Remove old entries
                while window
                    .timestamps
                    .front()
                    .is_some_and(|&t| t < one_minute_ago)
                {
                    window.timestamps.pop_front();
                }

                if (window.timestamps.len() as u32) < window.max_per_minute {
                    window.timestamps.push_back(now);
                    None
                } else {
                    // Wait until the oldest entry expires
                    let oldest = window.timestamps.front().unwrap();
                    Some(Duration::from_secs(60) - (now - *oldest) + Duration::from_millis(10))
                }
            };

            match wait_time {
                None => break,
                Some(duration) => {
                    tracing::debug!("Rate limit reached, waiting {duration:?}");
                    tokio::time::sleep(duration).await;
                }
            }
        }

        RateLimitGuard { _permit: permit }
    }
}

pub struct RateLimitGuard {
    _permit: tokio::sync::OwnedSemaphorePermit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn acquire_within_limit() {
        let limiter = RateLimiter::new(60, 5);
        let _guard = limiter.acquire().await;
        // Should not block
    }

    #[tokio::test]
    async fn concurrent_limit_respected() {
        let limiter = Arc::new(RateLimiter::new(100, 2));
        let mut handles = vec![];

        for _ in 0..2 {
            let l = limiter.clone();
            handles.push(tokio::spawn(async move {
                let _guard = l.acquire().await;
                tokio::time::sleep(Duration::from_millis(50)).await;
            }));
        }

        for h in handles {
            h.await.unwrap();
        }
    }
}
