use leptos::*;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};

use crate::instructions::random_instructions;

use super::server_functions::{get_score, register_win};

#[component]
pub fn HomePage() -> impl IntoView {
    let (instructions_count, set_instructions_count, _) =
        use_local_storage::<u8, JsonCodec>("instructions-count");

    if instructions_count.get_untracked() == 0 {
        set_instructions_count(1);
    }

    let (instructions, set_instructions) = create_signal(Vec::new());
    let score = create_resource(|| (), |_| async move { get_score().await });
    let win_action = create_action(move |_: &()| async move {
        score.set(register_win().await);
        set_instructions(random_instructions(instructions_count.get_untracked()));
    });
    let submitting_win = win_action.pending();

    create_effect(move |_| {
        set_instructions(random_instructions(instructions_count()));
    });

    view! {
        <h1>Moderato</h1>
        <Suspense fallback=move || view! { <p>Chargement...</p> }>
            <h2>Score: {score}</h2>
            <div>
                Nombre dinstructions
                <input
                    type="number"
                    min=1
                    max=4
                    value=instructions_count
                    on:change=move |event| {
                        let value = event_target_value(&event).parse().unwrap_or(1);
                        set_instructions_count(value);
                    }
                />

            </div>
            <div>
                <InstructionsList instructions/>
            </div>
            <div>
                <button
                    on:click=move |_| set_instructions(random_instructions(instructions_count()))
                    disabled=submitting_win
                >
                    Réessayer
                </button>
                <button on:click=move |_| win_action.dispatch(()) disabled=submitting_win>
                    Gagné
                </button>
            </div>
        </Suspense>
    }
}

#[component]
fn InstructionsList(instructions: ReadSignal<Vec<(u64, String)>>) -> impl IntoView {
    view! {
        <For
            each=instructions
            key=|item| item.0
            children=move |(_, instruction)| {
                view! { <p>{instruction}</p> }
            }
        />
    }
}
