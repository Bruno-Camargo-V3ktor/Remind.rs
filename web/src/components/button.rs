use dioxus::prelude::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum IconSide {
    #[default]
    Left,
    Right,
}

#[derive(Props, Debug, Clone, PartialEq)]
pub struct ButtonProps {
    icon: Element,
    #[props(default)]
    icon_side: IconSide,
    text: String,
    #[props(default = false)]
    inverse: bool,
    #[props(default = false)]
    disable: bool,
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
            button { class: "button-content {color_class}", disabled: props.disable, onclick: move |e| {e.prevent_default(); props.onclick.call(());},
                if props.icon_side == IconSide::Left {
                    {props.icon.clone()}
                }

                p {class: "button-text", "{props.text}"}

                if props.icon_side == IconSide::Right {
                    {props.icon.clone()}
                }
            }
        }
    }
}
