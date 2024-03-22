use crate::domain::entities::Fingerprint;
use crate::application::repositories::fingerprint_repository_abstract::FingerprintRepositoryAbstract;
use crate::infrastructure::repository::fingerprint_repository::MongoFingerprintRepository;
use std::error::Error;

pub trait FingerprintUseCase: Send + Sync + 'static {
    async fn create_fingerprint(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    async fn get_all_fingerprints(&self) -> Result<Vec<Fingerprint>, Box<dyn Error + Send + Sync>>;
}

impl FingerprintUseCase for MongoFingerprintRepository {
    async fn create_fingerprint(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        fingerprint.ip = Some("maxi_ip".parse().unwrap()); //WIP...
        self.insert(fingerprint).await
    }

    async fn get_all_fingerprints(&self) -> Result<Vec<Fingerprint>, Box<dyn Error + Send + Sync>> {
        self.get_all().await
    }
}
