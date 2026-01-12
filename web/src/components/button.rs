use dioxus::prelude::*;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct ButtonProps {
    icon: Element,
    text: String,
    #[props(default = false)]
    inverse: bool,
    onclick: EventHandler,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let color_class = if props.inverse {
        "secondary-button-color"
    } else {
        "primary-button-color"
    };

    rsx! {
        div { class: "button-container",
            button { class: "button-content {color_class}", onclick: move |_| {props.onclick.call(());},
                {props.icon}
                p {class: "button-text", "{props.text}"}
            }
        }
    }
}
