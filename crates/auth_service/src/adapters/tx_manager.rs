use deadpool_postgres::Object;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{
    application::common::tx_manager::TxManager,
    entities::{errors::base::InfallibleVoid, shared::ThreadSafe},
    unexpected_err,
};

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum InvalidTransactionStateError {
    #[error("Transaction already commited.")]
    AlreadyCommited,
    #[error("Transaction already begun.")]
    AlreadyBegun,
    #[error("Trasaction already rollbacked")]
    AlreadyRollbacked,
}

pub(crate) struct PgTxManager {
    conn: Arc<Object>,
    is_begun: AtomicBool,
    is_commited: AtomicBool,
    is_rollbacked: AtomicBool,
}

impl PgTxManager {
    pub(crate) fn new(conn: Arc<Object>) -> Self {
        Self {
            conn,
            is_begun: false.into(),
            is_commited: false.into(),
            is_rollbacked: false.into(),
        }
    }
}

impl TxManager for PgTxManager {
    async fn begin(&self) -> InfallibleVoid {
        if self.is_begun.load(Ordering::Relaxed) {
            return Err(crate::entities::errors::base::ErrorKind::Unexpected(
                InvalidTransactionStateError::AlreadyBegun.into(),
            ));
        }
        unexpected_err!(self.conn.execute("BEGIN", &[]).await);
        self.is_begun.store(true, Ordering::Relaxed);
        tracing::debug!("Transaction begun");
        Ok(())
    }
    async fn commit(&self) -> InfallibleVoid {
        if self.is_commited.load(Ordering::Relaxed) {
            return Err(crate::entities::errors::base::ErrorKind::Unexpected(
                InvalidTransactionStateError::AlreadyCommited.into(),
            ));
        }
        unexpected_err!(self.conn.execute("COMMIT", &[]).await);
        self.is_commited.store(true, Ordering::Relaxed);
        tracing::debug!("Transaction commited");
        Ok(())
    }

    async fn rollback(&self) -> InfallibleVoid {
        if self.is_rollbacked.load(Ordering::Relaxed) {
            return Err(crate::entities::errors::base::ErrorKind::Unexpected(
                InvalidTransactionStateError::AlreadyRollbacked.into(),
            ));
        }
        unexpected_err!(self.conn.execute("ROLLBACK", &[]).await);
        self.is_rollbacked.store(true, Ordering::Relaxed);
        Ok(())
    }
}

pub(crate) async fn finalize_tx_manager(dep: Arc<PgTxManager>) {
    let is_begun = dep.is_begun.load(Ordering::Relaxed);
    let is_commited = dep.is_begun.load(Ordering::Relaxed);
    let is_rollbacked = dep.is_rollbacked.load(Ordering::Relaxed);

    if is_rollbacked && !is_begun {
        tracing::error!("Invalid transaction state, transaction is rollbacked but not begun");
        panic!("Invalid transaction state, transaction is rollbacked but not begun")
    }
    if is_commited && !is_begun {
        tracing::error!("Invalid transaction state, transaction is commited but not begun");
        panic!("Invalid transaction state, transaction is commited but not begun")
    }

    if (is_rollbacked || is_commited) && is_begun {
        return;
    }

    if !(is_rollbacked || is_commited || is_begun) {
        tracing::warn!("Unused tx manager");
        return;
    }
    tracing::info!("Rollbacking transaction");
    dep.rollback().await.expect("Failed to rollback transation");
}

impl ThreadSafe for PgTxManager {}
