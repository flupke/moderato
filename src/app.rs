use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::{AdminPage, HomePage},
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
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="admin" view=AdminPage/>
                </Routes>
            </main>
        </Router>
    }
}
