use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, ButtonShape, Space, Text};

#[component]
pub fn SimpleCounter(
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let value = RwSignal::new(initial_value);

    let inc = move |_| value.update(|value| *value += step);

    view! {
        <Space>
            <Button appearance=ButtonAppearance::Primary on_click=move |_| value.set(0)>
                "Clear"
            </Button>
            <Button on_click=move |_| *value.write() -= step shape=ButtonShape::Circular>
                "-1"
            </Button>
            <Text>"Value: " {value} "!"</Text>
            <Button on_click=inc>"+1"</Button>
        </Space>
    }
}
