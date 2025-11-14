pub(crate) trait IdProvider {
    fn get_user_id(&self) -> &str;
}
