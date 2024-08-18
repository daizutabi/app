use leptos::prelude::*;
use leptos::spawn::tick;
use src_ui_lib::components::counter::SimpleCounter;
use wasm_bindgen_test::*;

mod common;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn increment_counter() {
    open_counter();
    click_increment();
    click_increment();
    tick().await;

    assert_eq!(get_text(), Some("Value: 12!".to_string()));
}

#[wasm_bindgen_test]
async fn decrement() {
    open_counter();
    click_decrement();
    click_decrement();
    tick().await;

    assert_eq!(get_text(), Some("Value: 8!".to_string()));
}

#[wasm_bindgen_test]
async fn clear() {
    open_counter();
    click_clear();
    tick().await;

    assert_eq!(get_text(), Some("Value: 0!".to_string()));
}

fn open_counter() {
    common::remove_top_div();
    leptos::mount::mount_to_body(move || view! { <SimpleCounter initial_value=10 step=1 /> });
}

fn click_clear() {
    common::click_text("Clear");
}

fn click_decrement() {
    common::click_text("-1");
}

fn click_increment() {
    common::click_text("+1");
}

fn get_text() -> Option<String> {
    common::find_by_text("Value: ").text_content()
}
