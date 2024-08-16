use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Space, Text};

#[component]
pub fn SimpleCounter(
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let (value, set_value) = signal(initial_value);

    view! {
        <Space>
            <Button appearance=ButtonAppearance::Primary on:click=move |_| set_value.set(0)>
                "Clear"
            </Button>
            <Button on:click=move |_| *set_value.write() -= step>"-1"</Button>
            <Text>"Value: " {value} "!"</Text>
            <Button on:click=move |_| set_value.update(|value| *value += step)>"+1"</Button>
        </Space>
    }
}
