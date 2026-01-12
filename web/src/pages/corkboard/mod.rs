use crate::components::{Button, Iconoir, Subtitle, TextInput};
use dioxus::prelude::*;

const STYLE: Asset = asset!("./style.css");

#[component]
pub fn CorkBoardPage() -> Element {
    let mut value_signal = Signal::new(String::from(""));
    let mut error_signal = Signal::new(None);

    let validator = move |value: String| {
        if error_signal().is_some() {
            error_signal.set(None);
        }
        if value.len() > 4 {
            error_signal.set(Some(String::from(
                "Por favor escreva no maximo 4 caracteres",
            )));
        }
        value_signal.set(value);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }

        Subtitle { text: "Teste" }

        TextInput {
            name: "teste",
            label: "teste",
            placeholder: "teste",
            error: error_signal,
            value: value_signal,
            validator: validator,
            content: rsx!{ span { "sasdasd" }}
        }

        TextInput {
            name: "teste",
            label: "teste",
            placeholder: "teste",
            value: value_signal,
            error: error_signal,
            validator: validator,
            content: rsx!{ span { "sasdasd" }}
        }

        Iconoir { icon: "check-circle-solid" }

        Button {
            icon: rsx!( Iconoir { icon: "download" } ),
            text: "label button",
            onclick: || {}
        }

        Button {
            inverse: true,
            icon: rsx!( Iconoir { icon: "download" } ),
            text: "label button",
            onclick: || {}
        }
    }
}
