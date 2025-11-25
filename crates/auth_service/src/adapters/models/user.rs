use sea_query::Iden;

#[derive(Iden)]
pub enum User {
    #[iden = "users"]
    Table,
    Id,
    IsBanned,
    CreatedAt,
}
