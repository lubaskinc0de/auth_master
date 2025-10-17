use uuid::Uuid;

use crate::application::common::id_generator::IdGenerator;

pub(crate) struct V4IdGenerator {}

impl IdGenerator for V4IdGenerator {
    fn generate(&self) -> Uuid {
        Uuid::new_v4()
    }
}
