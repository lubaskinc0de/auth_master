use crate::application::common::idp::IdProvider;

#[derive(Clone)]
pub(crate) struct ExternalIdProvider {
    external_id: String
}

impl ExternalIdProvider {
    pub(crate) fn new(external_id: String) -> Self {
        Self { external_id }
    }
}

impl IdProvider for ExternalIdProvider {
    fn get_user_id(&self) -> &str {
        &self.external_id
    }
}