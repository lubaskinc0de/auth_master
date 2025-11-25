use uuid::Uuid;

use crate::entities::shared::ThreadSafe;

pub(crate) trait IdGenerator: ThreadSafe {
    fn generate(&self) -> Uuid;
}
