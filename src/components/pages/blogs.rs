use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Blogs(id: i32) -> Element {
    rsx! {
        div {
            class: "home-container",
            h1 { "Blogs {id}" }
            Link { to: Route::Home {}, "Go to counter" },
        }
    }
}
