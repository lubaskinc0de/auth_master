use uuid::Uuid;

use crate::{application::common::id_generator::IdGenerator, entities::shared::ThreadSafe};

pub(crate) struct V4IdGenerator {}

impl IdGenerator for V4IdGenerator {
    fn generate(&self) -> Uuid {
        Uuid::new_v4()
    }
}

impl ThreadSafe for V4IdGenerator {}
