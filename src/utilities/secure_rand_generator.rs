use std::sync::{Arc, Mutex};
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};
use crate::server_errors::ServerError;
use crate::utilities::traits::RandomNumberGenerator;

#[derive(Clone)]
pub struct ChaCha20 {
    generator: Arc<Mutex<ChaCha20Rng>>
}

impl ChaCha20 {
    pub fn new() -> Self {
        Self {
            generator: Arc::new(Mutex::new(ChaCha20Rng::from_entropy()))
        }
    }
}

impl RandomNumberGenerator for ChaCha20 {
    fn generate(&self) -> Result<u64, ServerError> {
        self.generator
            .lock()
            .map(|mut generator| generator.next_u64())
            .map_err(|e| ServerError::InternalError(Arc::new(anyhow::Error::msg(e.to_string()))))
    }
}