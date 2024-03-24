use crate::domain::fingerprint_entity::FingerprintEntity;
use std::error::Error;
use std::future::Future;

pub trait FingerprintRepositoryAbstract {
    fn insert(
        &self,
        fingerprint: &mut FingerprintEntity,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send;

    fn get_all(
        &self,
    ) -> impl Future<Output = Result<Vec<FingerprintEntity>, Box<dyn Error + Send + Sync>>> + Send;
}
