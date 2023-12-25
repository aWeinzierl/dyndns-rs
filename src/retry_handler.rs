use std::time::Duration;

pub struct RetryHandler {
    max_attempts: usize,
    base_delay: u64,
}

impl RetryHandler {
    pub fn new(max_exponential_backoffs: usize, base_delay: u64) -> Self {
        RetryHandler {
            max_attempts: max_exponential_backoffs,
            base_delay,
        }
    }
}

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
fn from(u: usize) -> u32 {
    u as u32
}

#[cfg(target_pointer_width = "64")]
fn from(u: usize) -> u32 {
    if u > u32::MAX as usize {
        panic!("Cannot convert variable of type usize to u32, because it does not fit into u32")
    } else {
        u as u32
    }
}

impl futures_retry::ErrorHandler<()> for RetryHandler
{
    type OutError = ();
    fn handle(&mut self, failed_attempt: usize, e: ()) -> futures_retry::RetryPolicy<()> {
        if failed_attempt == self.max_attempts {
            return futures_retry::RetryPolicy::ForwardError(e);
        }
        futures_retry::RetryPolicy::WaitRetry(Duration::from_millis(
            self.base_delay.pow(from(failed_attempt))))
    }
}
