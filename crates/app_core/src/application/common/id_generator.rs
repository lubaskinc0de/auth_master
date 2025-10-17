use uuid::Uuid;

pub(crate) trait IdGenerator {
    fn generate(&self) -> Uuid;
}
