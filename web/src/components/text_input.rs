use dioxus::prelude::*;

use crate::components::Iconoir;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct TextInputProps {
    name: String,
    label: String,
    placeholder: String,
    error: Signal<Option<String>>,
    value: Signal<String>,
    validator: EventHandler<String>,
    content: Option<Element>,
    #[props(default = "text".into())]
    r#type: String,
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    let on_changed_value = move |e: Event<FormData>| props.validator.call(e.value());

    let value_signal = props.value.clone();
    let error_signal = props.error.clone();

    rsx! {
        div { class: "text-input-container",
            label { r#for: "{props.name}", "{props.label}" }

            div { class: "text-input-content",
                 input { id: "{props.name}", r#type: "{props.r#type}", placeholder: "{props.placeholder}", value: value_signal(), oninput: on_changed_value}
                 if let Some(element) = props.content {
                    {element}
                 }
            }

            if let Some(error) = error_signal() {
                div { class: "text-input-error",
                Iconoir { icon: "xmark-circle-solid", style: "color: var(--accent-red)"}
                span { "{error}" }
                }
            }
        }
    }
}
