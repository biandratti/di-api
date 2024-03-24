use crate::adapters::spi::db::fingerprint_repository::MongoFingerprintRepository;
use crate::application::repositories::fingerprint_repository_abstract::FingerprintRepositoryAbstract;
use crate::application::utils::error_handling_utils::ErrorHandlingUtils;
use crate::domain::error::ApiError;
use crate::domain::fingerprint_entity::FingerprintEntity;
use std::future::Future;

pub trait FingerprintUseCase: Send + Sync + 'static {
    fn create_fingerprint(
        &self,
        fingerprint: &mut FingerprintEntity,
    ) -> impl Future<Output = Result<(), ApiError>> + Send;

    fn get_all_fingerprints(
        &self,
    ) -> impl Future<Output = Result<Vec<FingerprintEntity>, ApiError>> + Send;
}

impl FingerprintUseCase for MongoFingerprintRepository {
    async fn create_fingerprint(
        &self,
        fingerprint: &mut FingerprintEntity,
    ) -> Result<(), ApiError> {
        fingerprint.ip = Some("maxi_ip".parse().unwrap()); //WIP...
        let result = self.insert(fingerprint).await;
        match result {
            Ok(fingerprints) => Ok(fingerprints),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Cannot create a fingerprint",
                e,
            )),
        }
    }

    async fn get_all_fingerprints(&self) -> Result<Vec<FingerprintEntity>, ApiError> {
        let result = self.get_all().await;
        match result {
            Ok(fingerprints) => Ok(fingerprints),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Cannot get all fingerprints",
                e,
            )),
        }
    }
}
