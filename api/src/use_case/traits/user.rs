use async_trait::async_trait;

#[async_trait]
pub trait RegisterUserUseCase: Send + Sync + 'static {
}
