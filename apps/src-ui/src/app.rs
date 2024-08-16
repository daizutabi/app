use crate::components::counter::SimpleCounter;
use crate::components::tauri::TauriTester;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <SimpleCounter initial_value=0 step=2 />
            <TauriTester />
        </main>
    }
}
