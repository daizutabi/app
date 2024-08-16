// use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use thaw::{Button, Input};
use uuid::Uuid;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Think of it as a mutation: some imperative async action you run,
// whereas a resource would be some async data you load
async fn add_todo(text: String) -> Uuid {
    logging::console_log(text.as_str());
    // fake a one-second delay
    // TimeoutFuture::new(1_000).await;
    // pretend this is a post ID or something
    Uuid::new_v4()
}

#[component]
pub fn Act() -> impl IntoView {
    // an action takes an async function with single argument
    // it can be a simple type, a struct, or ()
    let act = Action::new(|n: &String| add_todo(n.to_owned()));

    // actions provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = act.input();
    let pending = act.pending();
    let todo_id = act.value();

    let value = RwSignal::new(String::from(""));

    view! {
        <div>
            <Input value />

            <Button on_click=move |_| {
                act.dispatch(value.get());
            }>"Submit"</Button>
            <p>{move || pending.get().then(|| "Loading...")}</p>
            <p>"Submitted: " {move || format!("{:#?}", submitted.get())}</p>
            <p>"Pending: " {move || format!("{:#?}", pending.get())}</p>
            <p>"Todo ID: " {move || format!("{:#?}", todo_id.get())}</p>
            <p>A{submitted}A{pending}B</p>
        </div>
    }
}
