use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn remove_top_div() {
    if let Some(counter) = document().query_selector("body div").unwrap() {
        counter.remove();
    }
}

pub fn click_text(text: &str) {
    find_by_text(text).click();
}

pub fn find_by_text(text: &str) -> HtmlElement {
    let xpath = format!("//*[text()='{}']", text);
    let document = document();
    document
        .evaluate(&xpath, &document)
        .unwrap()
        .iterate_next()
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
}
