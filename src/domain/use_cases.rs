use std::error::Error;

use super::entities::Fingerprint;

pub trait FingerprintUseCase {
    type ConcreteType: FingerprintUseCase;

    async fn new(db_url: &str) -> Result<Box<Self::ConcreteType>, Box<dyn Error + Send + Sync>>;
    async fn insert(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn get_all(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
}
