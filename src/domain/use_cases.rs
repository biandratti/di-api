use crate::domain::entities::Fingerprint;
use crate::domain::repositories::FingerprintRepository;
use crate::infrastructure::mongo::MongoFingerprintRepository;
use std::error::Error;

pub trait FingerprintUseCase: Send + Sync + 'static {
    async fn create_fingerprint(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

impl FingerprintUseCase for MongoFingerprintRepository {
    async fn create_fingerprint(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        //fingerprint.created = Some(bson::DateTime::now());

        // Generate UUID for fingerprint
        fingerprint.id = bson::Uuid::new();
        fingerprint.ip = Some("maxi_ip".parse().unwrap());

        // Call the insert method of the repository
        self.insert(fingerprint).await
    }
}
