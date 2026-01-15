use crate::components::{
    Button, ButtonVariant, FloatBar, FloatBarButton, Iconoir, IconoirButton, Property, Subtitle,
    TextAreaInput, TextInput, Title,
};
use dioxus::prelude::*;

const STYLE: Asset = asset!("./style.css");

#[component]
pub fn PreviewPage() -> Element {
    let mut value1_signal = Signal::new(String::from(""));
    let mut value2_signal = Signal::new(String::from(""));
    let mut error_signal = Signal::new(None);

    let mut active_floatbar = use_signal(|| String::from("home"));

    let validator = move |value: String| {
        if error_signal().is_some() {
            error_signal.set(None);
        }
        if value.len() > 4 {
            error_signal.set(Some(String::from(
                "Por favor escreva no maximo 4 caracteres",
            )));
        }
        value2_signal.set(value);
    };

    let handle = move |(action, mut state): (String, Signal<String>)| {
        state.set(action);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }

        Title { }

        Subtitle { text: "Teste" }

        TextAreaInput {
            name: "test",
            label: "test",
            placeholder: "placeholder",
            value: value1_signal,
            validator: move |value| { value1_signal.set(value); }
        }

        TextInput {
            name: "teste",
            label: "teste",
            placeholder: "teste",
            error: error_signal,
            value: value2_signal,
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

        IconoirButton {
            icon: "list",
            onclick: || {}
        }

        IconoirButton {
            icon: "list",
            variant: ButtonVariant::Secondary,
            onclick: || {}
        }

        IconoirButton {
            icon: "list",
            variant: ButtonVariant::Tertiary,
            onclick: || {}
        }

        Property {
            text: "Blue",
            color: "var(--accent-blue)"
        }

        Property {
            text: "Purple",
            color: "var(--accent-purple)"
        }

        Property {
            text: "Green",
            color: "var(--accent-green)"
        }

        FloatBar { handle: handle, state: active_floatbar,

            FloatBarButton { icon: "list", action: "home" }
            FloatBarButton { icon: "user-circle" , action: "perfil"}
            FloatBarButton { icon: "log-out" , action: "exit"}

        }
    }
}
