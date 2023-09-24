use std::sync::atomic::{AtomicU64, Ordering};
use crate::server_errors::ServerError;
use crate::utilities::secure_rand_generator::RandomNumberGenerator;

pub struct FakeRandomGenerator {
    state: AtomicU64
}

impl FakeRandomGenerator {
    pub fn new() -> Self {
        Self {
            state: AtomicU64::new(0),
        }
    }
}

impl RandomNumberGenerator for FakeRandomGenerator {
    fn generate(&self) -> Result<u64, ServerError> {
        Ok(self.state.fetch_add(1, Ordering::SeqCst))
    }
}