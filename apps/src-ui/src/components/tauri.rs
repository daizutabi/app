use futures::stream::StreamExt;
use leptos::prelude::*;
use leptos::spawn::spawn_local;
use serde::{Deserialize, Serialize};
use thaw::{Button, Input};
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;

//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
//     async fn listen(event: &str, closure: &Closure<dyn Fn(JsValue)>) -> JsValue;

//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
//     async fn emit(event: &str, payload: JsValue);

//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
//     async fn invoke_without_args(cmd: &str) -> JsValue;
// }

#[derive(Serialize, Deserialize)]
struct GreetArgs {
    name: String,
}

#[component]
pub fn TauriTester() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let greet_msg = RwSignal::new(String::new());
    let emit_count = RwSignal::new(0);
    let listen_event = RwSignal::new(0);

    spawn_local(async move {
        let mut listener = tauri_sys::event::listen::<i32>("front-to-back")
            .await
            .unwrap();

        while let Some(event) = listener.next().await {
            emit_count.set(event.payload + 1);
        }
    });

    spawn_local(async move {
        let mut listener = tauri_sys::event::listen::<i32>("back-to-front")
            .await
            .unwrap();

        while let Some(event) = listener.next().await {
            listen_event.set(event.payload);
            if event.payload == 100 {
                break;
            }
        }
    });

    let trigger_invoke = move |_| {
        spawn_local(async move {
            let name = name.get_untracked();
            let args = GreetArgs { name };
            let msg: String = tauri_sys::core::invoke("greet", args).await;
            greet_msg.set(msg);
        })
    };

    let trigger_emit_event = move |_| {
        spawn_local(async move {
            let n = emit_count.get_untracked();
            tauri_sys::event::emit("front-to-back", &n).await.unwrap();
        });
    };

    let trigger_listen_events = move |_| {
        spawn_local(async move {
            tauri_sys::core::invoke::<()>("trigger_listen_events", &()).await;
        });
    };

    view! {
        <div>
            <div>
                <Input value=name />
                <Button on_click=trigger_invoke>"Trigger invoke"</Button>
                <p>{greet_msg}</p>
            </div>
            <div>
                <Button on_click=trigger_emit_event>"Trigger emit event"</Button>
                <p>
                    <strong>"Events emitted: "</strong>
                    {emit_count}
                </p>
            </div>
            <div>
                <Button on_click=trigger_listen_events>"Trigger listen events"</Button>
                <p>
                    <strong>"Last listen event: "</strong>
                    {listen_event}
                </p>
            </div>
        </div>
    }
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
//     async fn invoke_without_args(cmd: &str) -> JsValue;

//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

// #[derive(Serialize, Deserialize)]
// struct MyCustomMsg {
//     invoke_msg: String,
// }

// #[component]
// pub fn App() -> impl IntoView {
//     let (name, set_name) = signal(String::new());
//     let (greet_msg, set_greet_msg) = signal(String::new());

//     let update_name = move |ev| {
//         let v = event_target_value(&ev);
//         set_name.set(v);
//     };

//     let greet = move |ev: SubmitEvent| {
//         ev.prevent_default();
//         spawn_local(async move {
//             let name = name.get_untracked();
//             if name.is_empty() {
//                 return;
//             }

//             let args = to_value(&GreetArgs { name: &name }).unwrap();
//             // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//             let new_msg = invoke("greet", args).await.as_string().unwrap();
//             set_greet_msg.set(new_msg);

//             invoke_without_args("my_custom_command").await;
//             let invoke_msg = "abc".into();
//             let args = to_value(&MyCustomMsg { invoke_msg }).unwrap();
//             invoke("my_custom_msg", args).await;
//             // let x: Vec<u8> = invoke_without_args("read_file").await.into();

//             logging::log!("invoked.");
//         });
//     };

//     view! {
//         <main>
//             <form on:submit=greet>
//                 <input id="greet-input" placeholder="Enter a name..." on:input=update_name />
//                 <button type="submit">"Greet"</button>
//             </form>
//             <p>
//                 <b>{move || greet_msg.get()}</b>
//             </p>
//         </main>
//     }
// }
