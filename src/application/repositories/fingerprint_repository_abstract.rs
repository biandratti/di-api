use std::error::Error;

use crate::domain::entities::Fingerprint;

pub trait FingerprintRepositoryAbstract {
    async fn insert(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    async fn get_all(&self) -> Result<Vec<Fingerprint>, Box<dyn Error + Send + Sync>>;
}
