use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn TopNavigationBar() -> Element {
    let mut active = use_signal(|| "");
    rsx! {
        div {
            class: "top_bar",
            div { class: "logo" },
            ul {
              class: "menu-panel {active}",
              li {
                a {
                  href: Route::Home {}.to_string(),
                  "Home"
                }
              },
              li {
                a {
                  href: Route::Blogs { id: 0 }.to_string(),
                  "Blogs"
                }
              },
              li {
                a {
                  href: Route::Blogs { id: 0 }.to_string(),
                  "Ports"
                }
              },
              li {
                a {
                  href: Route::Blogs { id: 0 }.to_string(),
                  "What am I?"
                }
              },
            }
            div {
              class: "hamberger {active}",
              onclick:move |_| {
                if active() == "open" {
                    active.set("");
                } else {
                    active.set("open");
                }
              },
              span { class: "dash p1 {active}" },
              span { class: "dash p2 {active}" },
              span { class: "dash p3 {active}" },
            },
        }
    }
}
