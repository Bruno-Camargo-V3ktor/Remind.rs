use dioxus::prelude::*;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct TextAreaInputProps {
    name: String,
    label: String,
    placeholder: String,
    value: Signal<String>,
    validator: EventHandler<String>,
}

#[component]
pub fn TextAreaInput(props: TextAreaInputProps) -> Element {
    let on_changed_value = move |e: Event<FormData>| props.validator.call(e.value());

    let value_signal = props.value.clone();

    rsx! {
        div { class: "textarea-input-container",
            label { r#for: "{props.name}", "{props.label}" }

            div { class: "textarea-input-content",
                 textarea { id: "{props.name}", placeholder: "{props.placeholder}", value: value_signal(), oninput: on_changed_value}
            }
        }
    }
}
