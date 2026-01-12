use dioxus::prelude::*;

use crate::components::Iconoir;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct IconoirButtonProps {
    icon: String,
    #[props(default)]
    variant: ButtonVariant,
    onclick: EventHandler,
}

#[component]
pub fn IconoirButton(props: IconoirButtonProps) -> Element {
    let color = match props.variant {
        ButtonVariant::Primary => "p",
        ButtonVariant::Secondary => "s",
        ButtonVariant::Tertiary => "t",
    };

    rsx! {
        div { class: "iconoirbutton-container",
            button { class: "iconoirbutton-content iconoirbutton-{color}", onclick: move |_| {props.onclick.call(());},
                Iconoir { icon: "{props.icon}" }
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
}
