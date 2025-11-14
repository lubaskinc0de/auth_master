use sea_query::Iden;

#[derive(Iden)]
pub enum DbUser {
    Table,
    Id,
    Email,
    Username,
    IsBanned,
    CreatedAt,
}
