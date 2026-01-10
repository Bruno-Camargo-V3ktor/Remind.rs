use dioxus::prelude::*;

const STYLE: Asset = asset!("./style.css");

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
    }
}
