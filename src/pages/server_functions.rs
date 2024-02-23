use leptos::*;

#[server(RegisterWin, "/api")]
pub async fn register_win() -> Result<i32, ServerFnError> {
    use crate::db;

    let mut current_score = db::scores::get();
    current_score += 1;
    db::scores::set(current_score);
    Ok(current_score)
}

#[server(GetScore, "/api", "GetJson")]
pub async fn get_score() -> Result<i32, ServerFnError> {
    use crate::db;

    Ok(db::scores::get())
}

#[server(SetScore, "/api")]
pub async fn set_score() -> Result<(), ServerFnError> {
    use crate::db;
    db::scores::set(0);
    Ok(())
}
