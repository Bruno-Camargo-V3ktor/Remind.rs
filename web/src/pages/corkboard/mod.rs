use crate::components::Note;
use dioxus::prelude::*;

const STYLE: Asset = asset!("./style.css");

#[component]
pub fn CorkBoardPage() -> Element {
    rsx! {
        Note {}
    }
}
