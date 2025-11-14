use uuid::Uuid;

pub(crate) trait IdGenerator: Send + Sync + 'static {
    fn generate(&self) -> Uuid;
}
