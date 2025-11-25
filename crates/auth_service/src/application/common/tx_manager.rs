use crate::entities::{errors::base::InfallibleVoid, shared::ThreadSafe};

pub(crate) trait TxManager: ThreadSafe {
    async fn begin(&self) -> InfallibleVoid;
    async fn commit(&self) -> InfallibleVoid;
    async fn rollback(&self) -> InfallibleVoid;
}
