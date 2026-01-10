use dioxus::prelude::*;

const STYLE: Asset = asset!("./style.css");

#[component]
pub fn RegisterPage() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
    }
}
