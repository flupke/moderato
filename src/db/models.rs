use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Default)]
#[diesel(table_name = super::schema::scores)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Score {
    pub id: i32,
    pub score: i32,
}
