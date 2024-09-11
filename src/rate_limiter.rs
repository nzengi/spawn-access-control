#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub max_requests: u32,
    pub window: u64, // Time window in seconds
    pub requests: u32,
    pub start_time: u64,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: u64) -> Self {
        RateLimiter {
            max_requests,
            window,
            requests: 0,
            start_time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    // Check if the user is within the rate limit
    pub fn is_within_limit(&mut self) -> bool {
        let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        if current_time - self.start_time > self.window {
            // Reset the window and request count
            self.start_time = current_time;
            self.requests = 0;
        }

        if self.requests < self.max_requests {
            self.requests += 1;
            true
        } else {
            false
        }
    }
}
