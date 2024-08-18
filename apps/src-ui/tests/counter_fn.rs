use leptos::spawn::tick;
use pretty_assertions::assert_eq;
use src_ui_lib::components::counter_fn::counter;
use wasm_bindgen_test::*;

mod common;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn increment() {
    open_counter();
    click_increment();
    click_increment();
    tick().await;

    assert_eq!(get_text(), Some("Value: 2!".to_string()));
}

#[wasm_bindgen_test]
async fn decrement() {
    open_counter();
    click_decrement();
    click_decrement();
    tick().await;

    assert_eq!(get_text(), Some("Value: -2!".to_string()));
}

#[wasm_bindgen_test]
async fn clear() {
    open_counter();
    click_increment();
    click_increment();
    click_clear();
    tick().await;

    assert_eq!(get_text(), Some("Value: 0!".to_string()));
}

fn open_counter() {
    common::remove_top_div();
    leptos::mount::mount_to_body(move || counter(0, 1));
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
