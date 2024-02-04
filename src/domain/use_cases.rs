use std::error::Error;

use super::entities::Fingerprint;

pub trait FingerprintUseCase {
    type ConcreteType: FingerprintUseCase;

    async fn new(db_url: &str) -> Result<Box<Self::ConcreteType>, Box<dyn Error + Send + Sync>>;
    async fn create_fingerprint(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn get_all_fingerprints(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
}
