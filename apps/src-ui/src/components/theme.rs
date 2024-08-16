use leptos::prelude::*;
use thaw::{Button, Theme};

#[component]
pub fn Theme() -> impl IntoView {
    let theme = Theme::use_rw_theme();
    let theme_name = Memo::new(move |_| {
        theme.with(|theme| {
            if theme.name == *"light" {
                "Dark".to_string()
            } else {
                "Light".to_string()
            }
        })
    });
    let change_theme = move |_| {
        if theme_name.get_untracked() == "Light" {
            theme.set(Theme::light());
        } else {
            theme.set(Theme::dark());
        }
    };

    view! {
        <Button
            icon=Memo::new(move |_| {
                theme
                    .with(|theme| {
                        if theme.name == "light" {
                            icondata::BiMoonRegular
                        } else {
                            icondata::BiSunRegular
                        }
                    })
            })
            on_click=change_theme
        >
            {move || theme_name.get()}
        </Button>
    }
}
