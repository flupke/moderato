use crate::{
    error_template::{AppError, ErrorTemplate},
    instructions::random_instructions,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/moderato.css"/>
        <Title text="Moderato"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let score = create_resource(|| (), |_| async move { get_score().await });
    let instructions = create_resource(|| (), |_| async move { random_instructions() });
    let win_action = create_action(move |_: &()| async move {
        score.set(register_win().await);
        instructions.set(random_instructions());
    });
    let submitting_win = win_action.pending();

    view! {
        <h1>Moderato</h1>
        <Suspense fallback=move || view! { <p>Chargement...</p> }>
            <h2>Score: {score}</h2>
            <div>
                <p>{instructions}</p>
            </div>
            <div>
                <button on:click=move |_| instructions.set(random_instructions()) disabled={submitting_win}>Réessayer</button>
                <button on:click=move |_| win_action.dispatch(()) disabled={submitting_win}>Gagné</button>
            </div>
        </Suspense>
    }
}

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
