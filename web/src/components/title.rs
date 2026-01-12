use dioxus::prelude::*;

const ICON: Asset = asset!("/assets/favicon.svg");

#[component]
pub fn Title() -> Element {
    rsx! {
        div { class: "title-container",
            img { class: "title-icon", src: ICON }
            h1 {class: "title-text", "Remind.rs"}
        }
    }
}
