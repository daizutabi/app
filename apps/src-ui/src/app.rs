use crate::components::counter::SimpleCounter;
use crate::components::suspense::Sus;
use crate::components::tauri::TauriTester;
use crate::components::theme::Theme;
use leptos::prelude::*;
use thaw::ConfigProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <div style="min-height: 100vh">
                <SimpleCounter initial_value=0 step=2 />
                <TauriTester />
                <Theme />
                <Sus />
            </div>
        </ConfigProvider>
    }
}
