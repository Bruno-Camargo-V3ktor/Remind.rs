use dioxus::prelude::*;

mod pages;
mod router;

const FAVICON: Asset = asset!("/assets/favicon.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Onest:wght@100..900&display=swap" }
        document::Link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/gh/iconoir-icons/iconoir@main/css/iconoir.css" }

        Router::<router::Route> {}
    }
}
