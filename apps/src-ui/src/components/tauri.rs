use leptos::prelude::*;
use leptos::spawn::spawn_local;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use thaw::Button;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn TauriTester() -> impl IntoView {
    let (greet_msg, set_greet_msg) = signal(String::new());
    let submit = move |_| {
        logging::console_log("clicked.");

        spawn_local(async move {
            let name = greet_msg.get_untracked() + "abc";
            let args = GreetArgs {
                name: name.as_str(),
            };
            let args = to_value(&args).unwrap();
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        })
    };

    view! {
        <div>
            <Button on_click=submit>"Submit"</Button>
            <p>{greet_msg}</p>
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
