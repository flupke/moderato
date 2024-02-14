use super::establish_connection;
use super::models::Score;
use super::schema::scores::dsl::*;
use diesel::associations::HasTable;
use diesel::prelude::*;

/// Create the default score entry in db if it does not exist.
pub fn init() {
    let connection = &mut establish_connection();

    let results = scores
        .select(Score::as_select())
        .load(connection)
        .expect("Error loading scores");

    if results.is_empty() {
        diesel::insert_into(scores::table())
            .values(Score::default())
            .execute(connection)
            .expect("Error creating default score");
    }
}

/// Get the score from the db.
pub fn get() -> i32 {
    let connection = &mut establish_connection();
    scores
        .select(score)
        .first(connection)
        .expect("Error loading score")
}

/// Set the score in the db.
pub fn set(new_score: i32) {
    let connection = &mut establish_connection();
    diesel::update(scores.filter(id.eq(0)))
        .set(score.eq(new_score))
        .execute(connection)
        .expect("Error updating score");
}
