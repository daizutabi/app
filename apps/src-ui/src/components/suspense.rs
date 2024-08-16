// use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

// #[server]
async fn first_wait_fn(seconds: u64) -> u64 {
    // tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;

    // std::thread::sleep(std::time::Duration::from_secs(2));
    // TimeoutFuture::new(1_000).await;

    logging::console_log("suspense.");
    seconds
}

#[component]
pub fn Sus() -> impl IntoView {
    let (count, set_count) = signal(0);
    let one_second = Resource::new(|| 1, |value| async move { first_wait_fn(value).await });

    view! {
        <Suspense fallback=|| {
            "Loading 1..."
        }>
            {move || {
                one_second
                    .get()
                    .map(|_| {
                        view! {
                            <p id="loaded-1">"One Second: Loaded 1!"</p>
                            <button on:click=move |_| {
                                set_count.update(|n| *n += 1)
                            }>{count}</button>
                        }
                    })
            }}
        </Suspense>
    }
}
