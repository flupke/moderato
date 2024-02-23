use leptos::*;

use super::server_functions::{get_score, set_score};

#[component]
pub fn AdminPage() -> impl IntoView {
    let score = create_resource(|| (), |_| async move { get_score().await });
    let reset_score = create_action(move |_: &()| async move {
        if let Ok(true) =
            window().confirm_with_message("Êtes vous sûr de vouloir remettre le score à zéro ?")
        {
            match set_score().await {
                Ok(_) => score.set(Ok(0)),
                Err(error) => {
                    window()
                        .alert_with_message(&format!("Erreur: {:?}", error))
                        .ok();
                }
            };
        }
    });

    view! {
        <h1>Admin</h1>
        <Suspense fallback=move || view! { <p>Chargement...</p> }>
            <div>
                <p>Current score: {score}</p>
                <button on:click=move |_| reset_score.dispatch(())>Reset score</button>
            </div>
        </Suspense>
    }
}
