use crate::domain::error::ApiError;

pub trait AbstractUseCase<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}
