use crate::components::Note;
use dioxus::prelude::*;

const STYLE: Asset = asset!("./style.css");

#[component]
pub fn CorkBoardPage() -> Element {
    rsx! {
        Note {
            title: "Minha Nota",
            body: "",
            propertys: vec![],
            widht: 300.0,
            height: 300.0
        }
    }
}
