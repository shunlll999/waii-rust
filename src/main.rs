#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod components;
use components::pages::blogs::Blogs;
// use components::pages::home::Home;
use components::ui::topBar::TopNavigationBar;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blogs { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        TopNavigationBar {}
        Router::<Route> {}
    }
}

// #[component]
// fn Blog(id: i32) -> Element {
//     rsx! {
//         Link { to: Route::Home {}, "Go to counter" }
//         "Blog post {id}"
//     }
// }

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
      div {
        style: "max-width: 1280px; margin: 0 auto;",
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
        div {
          style: "margin-top: 2rem; color: #a5abb8; font-size: 14px;",
          Link {
            style: "color: #a5abb8; font-size: 18px;",
            to: Route::Blogs {
                id: count()
            },
            "Go to blog"
          }
        }

      }

    }
}
