use std::error::Error;

use crate::domain::fingerprint_entity::FingerprintEntity;

pub trait FingerprintRepositoryAbstract {
    async fn insert(
        &self,
        fingerprint: &mut FingerprintEntity,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    async fn get_all(&self) -> Result<Vec<FingerprintEntity>, Box<dyn Error + Send + Sync>>;
}
