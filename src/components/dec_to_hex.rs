use leptos::prelude::*;

use crate::exercises::dec_to_hex::{DecToHex, Outcome};

#[component]
pub fn DecToHexExercise() -> impl IntoView {
    let seed = RwSignal::new(0_u64);
    let exercise = Memo::new(move |_| DecToHex::from_seed(seed.get()));
    let input = RwSignal::new(String::new());
    let outcome = RwSignal::new(None::<Outcome>);
    
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let result = input.with(|text| exercise.get().check(text));
        outcome.set(Some(result));
    };
    
    let on_next = move |_| {
        seed.update(|s| *s += 1);
        input.set(String::new());
        outcome.set(None);
    };
    
    let feedback = move || {
        outcome.get().map(|o| match o {
            Outcome::Correct => "✅ Correct!".to_string(),
            Outcome::Incorrect { expected } => {
                format!("❌ Incorrect! Expected: {expected}")
            },
            Outcome::Invalid(err) => format!("⚠️ {err}"),
        })
    };

    view! {
        <section class="exercise">
            <h2>"Decimal → Hex"</h2>
            <p>
                "Convert " <strong>{move || exercise.get().value()}</strong>
                " to hexadecimal."
            </p>

            <form on:submit=on_submit>
                <input type="text" placeholder="e.g. FF" autocomplete="off" bind:value=input/>
                <button type="submit">"Check"</button>
            </form>

            <p class="feedback">{feedback}</p>

            <button on:click=on_next>"Next question"</button>
        </section>
    }
}