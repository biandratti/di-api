use crate::adapters::api::fingerprint::fingerprint_payload::FingerprintPayload;
use crate::adapters::api::fingerprint::fingerprint_presenters::FingerprintPresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::fingerprint_entity::FingerprintEntity;

pub struct FingerprintMapper {}

impl ApiMapper<FingerprintEntity, FingerprintPresenter, FingerprintPayload> for FingerprintMapper {
    fn to_api(entity: FingerprintEntity) -> FingerprintPresenter {
        FingerprintPresenter {
            id: entity.id,
            trace_id: entity.trace_id,
            ip: entity.ip,
            created: entity.created,
        }
    }

    fn to_entity(payload: FingerprintPayload) -> FingerprintEntity {
        FingerprintEntity {
            id: None,
            trace_id: payload.trace_id,
            ip: payload.ip,
            created: None,
        }
    }
}
